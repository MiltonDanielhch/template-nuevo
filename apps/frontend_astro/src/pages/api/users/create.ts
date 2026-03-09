import type { APIRoute } from "astro";

export const POST: APIRoute = async ({ request, cookies }) => {
  const data = await request.formData();
  const username = data.get("username")?.toString();
  const email = data.get("email")?.toString();
  const password = data.get("password")?.toString();
  const role = data.get("role")?.toString() || "User";

  const token = cookies.get("auth_token")?.value;

  try {
    const response = await fetch("http://localhost:8081/register", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
        Authorization: `Bearer ${token}`,
      },
      body: JSON.stringify({
        email,
        password,
        username,
        role,
      }),
    });

    if (!response.ok) {
      const errorData = await response.json();
      return new Response(errorData.message || "Error al crear usuario", {
        status: response.status,
      });
    }

    const newUser = await response.json();

    const html = `
      <tr id="user-${newUser.id}" class="border-b border-border hover:bg-muted/50 transition-colors animate-in fade-in slide-in-from-top-1 duration-500">
        <td class="px-4 py-3 text-sm font-medium text-foreground">${newUser.username || username || "N/A"}</td>
        <td class="px-4 py-3 text-sm text-muted-foreground">${newUser.email}</td>
        <td class="px-4 py-3 text-sm">
          <span class="inline-flex items-center rounded-full px-2 py-0.5 text-xs font-medium bg-muted text-muted-foreground">
            User
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
              @click="editUser('${newUser.id}', '${newUser.username || username || ""}', '${newUser.email}', 'User')"
            >
              Editar
            </button>
            <button
              class="text-destructive hover:text-destructive/80 transition-colors font-medium cursor-pointer"
              hx-delete="/api/users/delete?id=${newUser.id}"
              hx-target="#user-${newUser.id}"
              hx-swap="outerHTML"
              hx-confirm="¿Estás seguro de eliminar a ${newUser.username || username || newUser.email}?"
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
  } catch (error) {
    console.error("Backend Error:", error);
    return new Response("Error de conexión con el servidor", { status: 500 });
  }
};
