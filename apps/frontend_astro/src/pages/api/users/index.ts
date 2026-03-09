import type { APIRoute } from "astro";
import { getUsers } from "@/lib/db";

export const GET: APIRoute = async ({ url, request }) => {
  const search = url.searchParams.get("search")?.toLowerCase() || "";

  const allUsers = getUsers();

  const filteredUsers = allUsers.filter(
    (user) =>
      user.username.toLowerCase().includes(search) ||
      user.email.toLowerCase().includes(search) ||
      user.role.toLowerCase().includes(search),
  );

  const isHtmx = request.headers.get("HX-Request") === "true";

  if (isHtmx) {
    const html = filteredUsers
      .map(
        (user) => `
      <tr id="user-${user.id}" class="border-b border-border hover:bg-muted/50 transition-colors">
        <td class="px-4 py-3 text-sm font-medium text-foreground">${user.username}</td>
        <td class="px-4 py-3 text-sm text-muted-foreground">${user.email}</td>
        <td class="px-4 py-3 text-sm">
          <span class="inline-flex items-center rounded-full px-2 py-0.5 text-xs font-medium ${
            user.role === "Admin" ? "bg-primary/10 text-primary" : "bg-muted text-muted-foreground"
          }">
            ${user.role}
          </span>
        </td>
        <td class="px-4 py-3 text-sm">
          <span class="inline-flex items-center gap-1.5 text-foreground">
            <span class="h-1.5 w-1.5 rounded-full ${user.status === "Activo" ? "bg-green-500" : "bg-red-500"}"></span>
            ${user.status}
          </span>
        </td>
        <td class="px-4 py-3 text-right text-sm">
          <div class="flex justify-end gap-2">
            <button
              class="text-muted-foreground hover:text-primary transition-colors font-medium cursor-pointer"
              @click="editUser('${user.id}', '${user.username}', '${user.email}', '${user.role}')"
            >
              Editar
            </button>
            <button
              class="text-destructive hover:text-destructive/80 transition-colors font-medium cursor-pointer"
              hx-delete="/api/users/delete?id=${user.id}"
              hx-target="#user-${user.id}"
              hx-swap="outerHTML"
              hx-confirm="¿Estás seguro de eliminar a ${user.username}?"
            >
              Eliminar
            </button>
          </div>
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

export const ALL: APIRoute = (context) => GET(context);
