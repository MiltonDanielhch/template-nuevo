import type { APIRoute } from "astro";
import { updateUser } from "@/lib/db";

export const POST: APIRoute = async ({ request, url }) => {
  const id = url.searchParams.get("id");
  const data = await request.formData();
  const name = data.get("name");
  const email = data.get("email");
  const role = data.get("role");

  if (id && name && email && role) {
    const user = updateUser(id, {
      name: name.toString(),
      email: email.toString(),
      role: role.toString(),
    });

    if (!user) {
      return new Response(JSON.stringify({ error: "Usuario no encontrado" }), { status: 404 });
    }

    const html = `
      <tr id="user-${user.id}" class="border-b border-border hover:bg-muted/50 transition-colors">
        <td class="px-4 py-3 text-sm font-medium text-foreground">${user.name}</td>
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
          <span class="inline-flex items-center gap-1.5 text-foreground">
            <span class="h-1.5 w-1.5 rounded-full ${user.status === "Activo" ? "bg-green-500" : "bg-red-500"}"></span>
            ${user.status}
          </span>
        </td>
        <td class="px-4 py-3 text-right text-sm">
          <div class="flex justify-end gap-2">
            <button
              class="text-muted-foreground hover:text-primary transition-colors font-medium cursor-pointer"
              @click="editUser('${user.id}', '${user.name}', '${user.email}', '${user.role}')"
            >
              Editar
            </button>
            <button
              class="text-destructive hover:text-destructive/80 transition-colors font-medium cursor-pointer"
              hx-delete="/api/users/delete?id=${user.id}"
              hx-target="#user-${user.id}"
              hx-swap="outerHTML"
              hx-confirm="¿Estás seguro de eliminar a ${user.name}?"
            >
              Eliminar
            </button>
          </div>
        </td>
      </tr>
    `;

    return new Response(html, {
      headers: {
        "Content-Type": "text/html",
        "HX-Trigger": "user-updated",
      },
    });
  }

  return new Response(JSON.stringify({ error: "Datos incompletos" }), { status: 400 });
};
