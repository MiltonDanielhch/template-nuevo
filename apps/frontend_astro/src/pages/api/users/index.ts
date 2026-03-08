import type { APIRoute } from "astro";

export const GET: APIRoute = async ({ url, request }) => {
  const search = url.searchParams.get("search")?.toLowerCase() || "";

  // Datos simulados de usuarios (esto vendría del backend Rust)
  const allUsers = [
    {
      id: "1",
      name: "Admin Principal",
      email: "admin@lab3026.com",
      role: "Administrador",
      status: "Activo",
    },
    { id: "2", name: "Juan Pérez", email: "juan@example.com", role: "Usuario", status: "Activo" },
    {
      id: "3",
      name: "María García",
      email: "maria@example.com",
      role: "Editor",
      status: "Inactivo",
    },
    { id: "4", name: "Carlos López", email: "carlos@test.com", role: "Usuario", status: "Activo" },
    {
      id: "5",
      name: "Ana Martínez",
      email: "ana@lab.com",
      role: "Administrador",
      status: "Activo",
    },
  ];

  const filteredUsers = allUsers.filter(
    (user) =>
      user.name.toLowerCase().includes(search) ||
      user.email.toLowerCase().includes(search) ||
      user.role.toLowerCase().includes(search),
  );

  const isHtmx = request.headers.get("HX-Request") === "true";

  if (isHtmx) {
    // Si es una petición HTMX, devolvemos solo las filas de la tabla (fragmento)
    const html = filteredUsers
      .map(
        (user) => `
      <tr class="border-b border-border hover:bg-muted/50 transition-colors">
        <td class="px-4 py-3 text-sm font-medium">${user.name}</td>
        <td class="px-4 py-3 text-sm text-muted-foreground">${user.email}</td>
        <td class="px-4 py-3 text-sm">
          <span class="inline-flex items-center rounded-full px-2 py-0.5 text-xs font-medium ${
            user.role === "Administrador"
              ? "bg-primary/10 text-primary"
              : "bg-muted text-muted-foreground"
          }">
            ${user.role}
          </span>
        </td>
        <td class="px-4 py-3 text-sm">
          <span class="inline-flex items-center gap-1.5">
            <span class="h-1.5 w-1.5 rounded-full ${user.status === "Activo" ? "bg-green-500" : "bg-red-500"}"></span>
            ${user.status}
          </span>
        </td>
        <td class="px-4 py-3 text-right text-sm">
          <button class="text-muted-foreground hover:text-primary transition-colors">Editar</button>
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
};

// Necesitamos pasar 'request' al contexto de Astro
export const ALL: APIRoute = (context) => GET(context);
