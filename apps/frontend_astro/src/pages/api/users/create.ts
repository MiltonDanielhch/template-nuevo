import type { APIRoute } from "astro";
import { addUser } from "@/lib/db";

export const POST: APIRoute = async ({ request }) => {
  const data = await request.formData();
  const name = data.get("name");
  const email = data.get("email");
  const password = data.get("password");
  const role = data.get("role");

  if (name && email && role) {
    const newUser = addUser({
      name: name.toString(),
      email: email.toString(),
      password: password?.toString(),
      role: role.toString(),
    });

    const html = `
      <tr id="user-${newUser.id}" class="border-b border-border hover:bg-muted/50 transition-colors animate-in fade-in slide-in-from-top-1 duration-500">
        <td class="px-4 py-3 text-sm font-medium text-foreground">${newUser.name}</td>
        <td class="px-4 py-3 text-sm text-muted-foreground">${newUser.email}</td>
        <td class="px-4 py-3 text-sm">
          <span class="inline-flex items-center rounded-full px-2 py-0.5 text-xs font-medium ${
            newUser.role === "Administrador"
              ? "bg-primary/10 text-primary"
              : "bg-muted text-muted-foreground"
          }">
            ${newUser.role}
          </span>
        </td>
        <td class="px-4 py-3 text-sm">
          <span class="inline-flex items-center gap-1.5 text-foreground">
            <span class="h-1.5 w-1.5 rounded-full bg-green-500"></span>
            ${newUser.status}
          </span>
        </td>
        <td class="px-4 py-3 text-right text-sm">
          <div class="flex justify-end gap-2">
            <button
              class="text-muted-foreground hover:text-primary transition-colors font-medium cursor-pointer"
              @click="editUser('${newUser.id}', '${newUser.name}', '${newUser.email}', '${newUser.role}')"
            >
              Editar
            </button>
            <button
              class="text-destructive hover:text-destructive/80 transition-colors font-medium cursor-pointer"
              hx-delete="/api/users/delete?id=${newUser.id}"
              hx-target="#user-${newUser.id}"
              hx-swap="outerHTML"
              hx-confirm="¿Estás seguro de eliminar a ${newUser.name}?"
            >
              Eliminar
            </button>
          </div>
        </td>
      </tr>
    `;

    return new Response(html, {
      status: 201,
      headers: {
        "Content-Type": "text/html",
        "HX-Trigger": "user-created",
      },
    });
  }

  return new Response(JSON.stringify({ error: "Datos incompletos" }), { status: 400 });
};
