import type { APIRoute } from "astro";

export const GET: APIRoute = async ({ url, request, cookies }) => {
  const search = url.searchParams.get("search")?.toLowerCase() || "";
  const mobile = url.searchParams.get("mobile") === "true";
  const token = cookies.get("auth_token")?.value;

  try {
    const response = await fetch("http://localhost:8081/users", {
      headers: {
        Authorization: `Bearer ${token}`,
      },
    });

    if (!response.ok) {
      throw new Error("Error al obtener usuarios del backend");
    }

    const allUsers: any[] = await response.json();

    const filteredUsers = allUsers.filter(
      (user) =>
        (user.username || "").toLowerCase().includes(search) ||
        (user.email || "").toLowerCase().includes(search) ||
        (user.role || "").toLowerCase().includes(search),
    );

    const isHtmx = request.headers.get("HX-Request") === "true";

    if (isHtmx) {
      if (mobile) {
        const html = filteredUsers
          .map(
            (user) => `
          <div id="user-${user.id}" class="bg-card rounded-lg border p-4 space-y-3">
            <div class="flex items-center justify-between">
              <div class="flex items-center gap-3">
                <div class="h-10 w-10 rounded-full bg-primary/10 flex items-center justify-center">
                  <span class="text-sm font-medium text-primary">${(user.username || user.email).charAt(0).toUpperCase()}</span>
                </div>
                <div>
                  <p class="font-medium text-foreground">${user.username || "Sin nombre"}</p>
                  <p class="text-sm text-muted-foreground">${user.email}</p>
                </div>
              </div>
              <span class="inline-flex items-center rounded-full px-2 py-0.5 text-xs font-medium ${
                user.role === "Admin"
                  ? "bg-primary/10 text-primary"
                  : "bg-muted text-muted-foreground"
              }">
                ${user.role}
              </span>
            </div>
            <div class="flex gap-2 pt-2 border-t">
              <button
                class="flex-1 text-center py-2 text-sm text-muted-foreground hover:text-primary transition-colors font-medium cursor-pointer border rounded-lg"
                onclick="editUser('${user.id}', '${user.username || ""}', '${user.email}', '${user.role}')"
              >
                Editar
              </button>
              <button
                class="flex-1 text-center py-2 text-sm text-destructive hover:text-destructive/80 transition-colors font-medium cursor-pointer border border-destructive/30 rounded-lg"
                hx-delete="/api/users/delete?id=${user.id}"
                hx-target="#user-${user.id}"
                hx-swap="outerHTML"
                hx-confirm="¿Eliminar a ${user.username || user.email}?"
              >
                Eliminar
              </button>
            </div>
          </div>
        `,
          )
          .join("");

        return new Response(
          html ||
            '<p class="text-center text-muted-foreground py-4">No se encontraron usuarios</p>',
          { headers: { "Content-Type": "text/html" } },
        );
      }

      const html = filteredUsers
        .map(
          (user) => `
        <tr id="user-${user.id}" class="border-b border-border hover:bg-muted/50 transition-colors">
          <td class="px-3 py-3 text-sm">
            <div class="flex items-center gap-3">
              <div class="h-8 w-8 rounded-full bg-primary/10 flex items-center justify-center flex-shrink-0">
                <span class="text-xs font-medium text-primary">${(user.username || user.email).charAt(0).toUpperCase()}</span>
              </div>
              <span class="font-medium text-foreground whitespace-nowrap">${user.username || "Sin nombre"}</span>
            </div>
          </td>
          <td class="px-3 py-3 text-sm text-muted-foreground whitespace-nowrap">${user.email}</td>
          <td class="px-3 py-3 text-center text-sm">
            <span class="inline-flex items-center rounded-full px-2 py-0.5 text-xs font-medium ${
              user.role === "Admin"
                ? "bg-primary/10 text-primary"
                : "bg-muted text-muted-foreground"
            }">
              ${user.role}
            </span>
          </td>
          <td class="px-3 py-3 text-center text-sm">
            <span class="inline-flex items-center gap-1.5 text-foreground">
              <span class="h-1.5 w-1.5 rounded-full bg-green-500"></span>
              Activo
            </span>
          </td>
          <td class="px-3 py-3 text-right text-sm whitespace-nowrap">
            <button
              class="text-muted-foreground hover:text-primary transition-colors font-medium cursor-pointer"
              onclick="editUser('${user.id}', '${user.username || ""}', '${user.email}', '${user.role}')"
            >
              Editar
            </button>
            <button
              class="ml-3 text-destructive hover:text-destructive/80 transition-colors font-medium cursor-pointer"
              hx-delete="/api/users/delete?id=${user.id}"
              hx-target="#user-${user.id}"
              hx-swap="outerHTML"
              hx-confirm="¿Eliminar a ${user.username || user.email}?"
            >
              Eliminar
            </button>
          </td>
        </tr>
      `,
        )
        .join("");

      return new Response(
        html ||
          '<tr><td colspan="5" class="px-4 py-8 text-center text-muted-foreground">No se encontraron usuarios</td></tr>',
        {
          headers: { "Content-Type": "text/html" },
        },
      );
    }

    return new Response(JSON.stringify(filteredUsers), {
      headers: { "Content-Type": "application/json" },
    });
  } catch (error) {
    console.error("Backend Error:", error);
    return new Response("Error de conexión con el servidor", { status: 500 });
  }
};

export const ALL: APIRoute = (context) => GET(context);
