import type { APIRoute } from "astro";

export const POST: APIRoute = async ({ request }) => {
  const data = await request.formData();
  const name = data.get("name");
  const email = data.get("email");
  const role = data.get("role");

  // En un entorno real, aquí guardaríamos en la base de datos a través del backend Rust
  // const response = await fetch("http://localhost:8080/api/users", { ... });

  if (name && email && role) {
    const newUser = {
      id: Math.random().toString(36).substr(2, 9),
      name: name.toString(),
      email: email.toString(),
      role: role.toString(),
      status: "Activo",
    };

    // Devolvemos el fragmento HTML de la nueva fila para que HTMX la inserte
    // Usamos HX-Trigger para cerrar el modal en el cliente
    const html = `
      <tr class="border-b border-border hover:bg-muted/50 transition-colors animate-in fade-in slide-in-from-top-1 duration-500">
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
          <button class="text-muted-foreground hover:text-primary transition-colors font-medium">Editar</button>
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
