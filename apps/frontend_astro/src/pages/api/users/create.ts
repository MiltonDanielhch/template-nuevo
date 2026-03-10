import type { APIRoute } from "astro";

export const POST: APIRoute = async ({ request, cookies }) => {
  const data = await request.formData();
  const username = data.get("username")?.toString();
  const email = data.get("email")?.toString();
  const password = data.get("password")?.toString();
  const role = data.get("role")?.toString() || "User";
  const avatar_url = data.get("avatar_url")?.toString();

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
        avatar_url,
      }),
    });

    if (!response.ok) {
      const errorData = await response.json();
      return new Response(errorData.message || "Error al crear usuario", {
        status: response.status,
      });
    }

    const newUser = await response.json();
    const userRole = newUser.roles && newUser.roles.length > 0 ? newUser.roles[0] : "User";

    const html = `
      <tr id="user-${newUser.id}" class="border-b border-border hover:bg-muted/50 transition-colors animate-in fade-in slide-in-from-top-1 duration-500">
        <td class="px-4 py-3">
          ${
            newUser.avatar_url
              ? `<img src="${newUser.avatar_url}" class="h-10 w-10 rounded-full object-cover" />`
              : `<div class="h-10 w-10 rounded-full bg-primary/10 flex items-center justify-center text-xs font-bold text-primary">${(newUser.username || newUser.email || "U").charAt(0).toUpperCase()}</div>`
          }
        </td>
        <td class="px-4 py-3 font-medium">${newUser.username || "Sin nombre"}</td>
        <td class="px-4 py-3 text-muted-foreground">${newUser.email}</td>
        <td class="px-4 py-3">
          <span class="px-2 py-1 rounded-full text-xs ${userRole === "Admin" ? "bg-primary/10 text-primary" : "bg-muted"}">
            ${userRole}
          </span>
        </td>
        <td class="px-4 py-3 text-right">
          <button type="button" data-id="${newUser.id}" data-user="${newUser.username || ""}" data-email="${newUser.email}" data-role="${userRole}" data-avatar="${newUser.avatar_url || ""}" onclick="openEdit(this)" class="text-sm text-muted-foreground hover:text-primary mr-3">Editar</button>
          <button type="button" data-id="${newUser.id}" data-name="${newUser.username || newUser.email}" onclick="deleteUser(this)" class="text-sm text-red-500 hover:text-red-700">Eliminar</button>
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
