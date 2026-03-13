import type { APIRoute } from "astro";

interface User {
  id: string;
  username?: string;
  email?: string;
  role?: string;
  roles?: string[];
  avatar_url?: string;
  email_verified?: boolean;
}

interface UsersResponse {
  users: User[];
  total?: number;
  total_pages?: number;
}

function escapeHtml(str: string | undefined): string {
  if (!str) return "";
  return str
    .replace(/&/g, "&amp;")
    .replace(/</g, "&lt;")
    .replace(/>/g, "&gt;")
    .replace(/"/g, "&quot;")
    .replace(/'/g, "&#039;");
}

export const GET: APIRoute = async ({ url, request, cookies }) => {
  const search = url.searchParams.get("search") || "";
  const page = parseInt(url.searchParams.get("page") || "1", 10);
  const perPage = 10;
  const token = cookies.get("auth_token")?.value;

  if (!token || token === "undefined" || token === "null") {
    return new Response(JSON.stringify({ error: "Sin token de sesión" }), { status: 401 });
  }

  try {
    const params = new URLSearchParams({ page: page.toString(), per_page: perPage.toString() });
    if (search) params.set("search", search);

    const response = await fetch(`http://localhost:8081/api/v1/users?${params}`, {
      headers: {
        Authorization: `Bearer ${token}`,
        "Content-Type": "application/json",
      },
    });

    if (response.status === 401 || response.status === 403) {
      return new Response(JSON.stringify({ error: "Sesión expirada" }), {
        status: 401,
        headers: { "Content-Type": "application/json" },
      });
    }

    if (!response.ok) {
      const errText = await response.text();
      console.log("Backend error:", response.status, errText);
      throw new Error(`Error al obtener usuarios: ${response.status}`);
    }

    const data: UsersResponse = await response.json();
    let users: User[] = data.users || [];
    let total = data.total || 0;
    let total_pages = data.total_pages || 1;

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

    const renderUser = (user: User) => {
      if (!user || typeof user !== "object") return "";
      const roles = user.roles || [];
      const primaryRole = user.role || roles[0] || "Sin rol";
      const isAdmin = roles.includes("Admin") || primaryRole === "Admin";
      const safeUsername = escapeHtml(user.username);
      const safeEmail = escapeHtml(user.email);
      const safeAvatar = escapeHtml(user.avatar_url);
      const safeId = escapeHtml(user.id);

      return `
      <tr class="hover:bg-muted/50">
        <td class="px-4 py-3">
          ${
            user.avatar_url
              ? `<img src="${safeAvatar}" class="h-10 w-10 rounded-full object-cover" />`
              : `<div class="h-10 w-10 rounded-full bg-primary/10 flex items-center justify-center">
                <span class="text-sm font-bold text-primary">${(safeUsername || safeEmail || "U").charAt(0).toUpperCase()}</span>
               </div>`
          }
        </td>
        <td class="px-4 py-3 font-medium">${safeUsername || "Sin nombre"}</td>
        <td class="px-4 py-3 text-muted-foreground">${safeEmail}</td>
        <td class="px-4 py-3">
          <span class="px-2 py-1 rounded-full text-xs ${isAdmin ? "bg-primary/10 text-primary" : "bg-muted"}">
            ${escapeHtml(primaryRole)}
          </span>
        </td>
        <td class="px-4 py-3 text-right">
          <button type="button" data-id="${safeId}" data-user="${safeUsername}" data-email="${safeEmail}" data-role="${escapeHtml(primaryRole)}" data-avatar="${safeAvatar}" onclick="openEdit(this)" class="text-sm text-muted-foreground hover:text-primary mr-3">Editar</button>
          <button type="button" data-id="${safeId}" data-name="${safeUsername || safeEmail}" onclick="deleteUser(this)" class="text-sm text-red-500 hover:text-red-700">Eliminar</button>
        </td>
      </tr>
    `;
    };

    const tbodyHtml =
      users && users.length > 0
        ? users.map(renderUser).join("")
        : `<tr><td colspan="5" class="px-4 py-8 text-center text-muted-foreground">No se encontraron usuarios</td></tr>`;

    const _start = Math.min((page - 1) * perPage + 1, total);
    const _end = Math.min(page * perPage, total);
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
