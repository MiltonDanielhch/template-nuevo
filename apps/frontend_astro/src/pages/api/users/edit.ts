import type { APIRoute } from "astro";

export const POST: APIRoute = async ({ request, url, cookies }) => {
  const id = url.searchParams.get("id");
  const data = await request.formData();
  const username = data.get("username");
  const email = data.get("email");
  const password = data.get("password");
  const role = data.get("role") || "User";
  const avatar_url = data.get("avatar_url");

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

    if (avatar_url && avatar_url.toString().length > 0) {
      updateData.avatar_url = avatar_url.toString();
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
    const userRole = user.roles && user.roles.length > 0 ? user.roles[0] : "User";

    const html = `
      <tr id="user-${user.id}" class="border-b border-border hover:bg-muted/50 transition-colors">
        <td class="px-4 py-3">
          ${
            user.avatar_url
              ? `<img src="${user.avatar_url}" class="h-10 w-10 rounded-full object-cover" />`
              : `<div class="h-10 w-10 rounded-full bg-primary/10 flex items-center justify-center text-xs font-bold text-primary">${(user.username || "U").charAt(0).toUpperCase()}</div>`
          }
        </td>
        <td class="px-4 py-3 font-medium">${user.username || "Sin nombre"}</td>
        <td class="px-4 py-3 text-muted-foreground">${user.email}</td>
        <td class="px-4 py-3">
          <span class="px-2 py-1 rounded-full text-xs ${userRole === "Admin" ? "bg-primary/10 text-primary" : "bg-muted"}">
            ${userRole}
          </span>
        </td>
        <td class="px-4 py-3 text-right">
          <button type="button" data-id="${user.id}" data-user="${user.username || ""}" data-email="${user.email}" data-role="${userRole}" data-avatar="${user.avatar_url || ""}" onclick="openEdit(this)" class="text-sm text-muted-foreground hover:text-primary mr-3">Editar</button>
          <button type="button" data-id="${user.id}" data-name="${user.username || user.email}" onclick="deleteUser(this)" class="text-sm text-red-500 hover:text-red-700">Eliminar</button>
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
