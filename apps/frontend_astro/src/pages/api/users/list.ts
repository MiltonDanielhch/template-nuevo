import type { APIRoute } from "astro";

export const GET: APIRoute = async ({ url, request, cookies }) => {
  const search = url.searchParams.get("search") || "";
  const page = parseInt(url.searchParams.get("page") || "1");
  const perPage = 10;
  const token = cookies.get("auth_token")?.value;

  try {
    const params = new URLSearchParams({ page: page.toString(), per_page: perPage.toString() });
    if (search) params.set("search", search);

    const response = await fetch(`http://localhost:8081/users?${params}`, {
      headers: { Authorization: `Bearer ${token}` },
    });

    if (!response.ok) {
      throw new Error("Error al obtener usuarios");
    }

    const data: any = await response.json();
    let { users, total, total_pages } = data;

    // El backend Rust ya filtra cuando se pasa search, no necesitamos filtrar de nuevo
    // Solo verificamos que users sea un array válido
    if (!users || !Array.isArray(users)) {
      users = [];
      total = 0;
      total_pages = 1;
    }

    // Filtrar usuarios undefined o inválidos
    users = users.filter((u) => u && typeof u === "object" && u.id);

    // Recalcular totales basados en los usuarios filtrados
    total = users.length;
    total_pages = Math.ceil(total / perPage) || 1;

    const isHtmx = request.headers.get("HX-Request") === "true";

    if (!isHtmx) {
      return new Response(JSON.stringify(data), {
        headers: { "Content-Type": "application/json" },
      });
    }

    const renderUser = (user: any) => {
      if (!user || typeof user !== "object") return "";
      const roles = user.roles || [];
      const primaryRole = user.role || roles[0] || "Sin rol";
      const isAdmin = roles.includes("Admin") || primaryRole === "Admin";

      return `
      <tr class="hover:bg-muted/50">
        <td class="px-4 py-3">
          ${
            user.avatar_url
              ? `<img src="${user.avatar_url}" class="h-10 w-10 rounded-full object-cover" />`
              : `<div class="h-10 w-10 rounded-full bg-primary/10 flex items-center justify-center">
                <span class="text-sm font-bold text-primary">${(user.username || user.email || "U").charAt(0).toUpperCase()}</span>
               </div>`
          }
        </td>
        <td class="px-4 py-3 font-medium">${user.username || "Sin nombre"}</td>
        <td class="px-4 py-3 text-muted-foreground">${user.email || ""}</td>
        <td class="px-4 py-3">
          <span class="px-2 py-1 rounded-full text-xs ${isAdmin ? "bg-primary/10 text-primary" : "bg-muted"}">
            ${primaryRole}
          </span>
        </td>
        <td class="px-4 py-3 text-right">
          <button type="button" data-id="${user.id}" data-user="${user.username || ""}" data-email="${user.email}" data-role="${primaryRole}" data-avatar="${user.avatar_url || ""}" onclick="openEdit(this)" class="text-sm text-muted-foreground hover:text-primary mr-3">Editar</button>
          <button type="button" data-id="${user.id}" data-name="${user.username || user.email}" onclick="deleteUser(this)" class="text-sm text-red-500 hover:text-red-700">Eliminar</button>
        </td>
      </tr>
    `;
    };

    const tbodyHtml =
      users && users.length > 0
        ? users.map(renderUser).join("")
        : `<tr><td colspan="5" class="px-4 py-8 text-center text-muted-foreground">No se encontraron usuarios</td></tr>`;

    const start = Math.min((page - 1) * perPage + 1, total);
    const end = Math.min(page * perPage, total);
    const prevPage = page > 1 ? page - 1 : 1;
    const nextPage = page < total_pages ? page + 1 : total_pages;

    const paginationHtml =
      total_pages > 1
        ? `
      <div id="pagination-container" class="mt-4 flex items-center justify-end gap-1 w-full" hx-swap-oob="true">
        <button ${page <= 1 ? "disabled" : ""}
          onclick="goToPage(${prevPage})"
          class="px-3 py-1 text-sm border rounded hover:bg-muted ${page <= 1 ? "opacity-50 cursor-not-allowed" : ""}">Anterior</button>
        <span class="px-3 py-1 text-sm">Página ${page} de ${total_pages}</span>
        <button ${page >= total_pages ? "disabled" : ""}
          onclick="goToPage(${nextPage})"
          class="px-3 py-1 text-sm border rounded hover:bg-muted ${page >= total_pages ? "opacity-50 cursor-not-allowed" : ""}">Siguiente</button>
      </div>
    `
        : `<div id="pagination-container" class="mt-4 w-full" hx-swap-oob="true"></div>`;

    return new Response(tbodyHtml + paginationHtml, {
      headers: { "Content-Type": "text/html" },
    });
  } catch (error) {
    console.error("Backend Error:", error);
    return new Response(
      '<tr><td colspan="5" class="px-4 py-8 text-center text-red-500">Error al cargar usuarios</td></tr>',
      {
        headers: { "Content-Type": "text/html" },
      },
    );
  }
};

export const ALL: APIRoute = (context) => GET(context);
