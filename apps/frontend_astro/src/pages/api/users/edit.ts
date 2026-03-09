import type { APIRoute } from "astro";

export const POST: APIRoute = async ({ request, url, cookies }) => {
  const id = url.searchParams.get("id");
  const data = await request.formData();
  const username = data.get("username");
  const email = data.get("email");
  const password = data.get("password");
  const role = data.get("role") || "User";

  const token = cookies.get("auth_token")?.value;

  try {
    const updateData: any = {
      username: username?.toString(),
      email: email?.toString(),
      role: role?.toString(),
    };

    if (password && password.toString().length > 0) {
      updateData.password = password.toString();
    }

    const response = await fetch(`http://localhost:8081/users/${id}`, {
      method: "PUT",
      headers: {
        "Content-Type": "application/json",
        Authorization: `Bearer ${token}`,
      },
      body: JSON.stringify(updateData),
    });

    if (!response.ok) {
      const errorData = await response.json();
      return new Response(errorData.message || "Error al actualizar usuario", {
        status: response.status,
      });
    }

    const user = await response.json();

    const html = `
      <tr id="user-${user.id}" class="border-b border-border hover:bg-muted/50 transition-colors">
        <td class="px-4 py-3 text-sm font-medium text-foreground">${user.username || "Sin nombre"}</td>
        <td class="px-4 py-3 text-sm text-muted-foreground">${user.email}</td>
        <td class="px-4 py-3 text-sm">
          <span class="inline-flex items-center rounded-full px-2 py-0.5 text-xs font-medium ${
            role === "Admin" ? "bg-primary/10 text-primary" : "bg-muted text-muted-foreground"
          }">
            ${role}
          </span>
        </td>
        <td class="px-4 py-3 text-sm">
          <span class="inline-flex items-center gap-1.5 text-foreground">
            <span class="h-1.5 w-1.5 rounded-full bg-green-500"></span>
            Activo
          </span>
        </td>
        <td class="px-4 py-3 text-right text-sm">
          <div class="flex justify-end gap-2">
            <button
              class="text-muted-foreground hover:text-primary transition-colors font-medium cursor-pointer"
              @click="editUser('${user.id}', '${user.username || ""}', '${user.email}', '${role}')"
            >
              Editar
            </button>
            <button
              class="text-destructive hover:text-destructive/80 transition-colors font-medium cursor-pointer"
              hx-delete="/api/users/delete?id=${user.id}"
              hx-target="#user-${user.id}"
              hx-swap="outerHTML"
              hx-confirm="¿Estás seguro de eliminar a ${user.username || user.email}?"
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
  } catch (error) {
    console.error("Backend Error:", error);
    return new Response("Error de conexión con el servidor", { status: 500 });
  }
};
