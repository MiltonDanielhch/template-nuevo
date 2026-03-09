import type { APIRoute } from "astro";

export const POST: APIRoute = async ({ params, request, cookies }) => {
  const { id } = params;
  const token = cookies.get("auth_token")?.value;
  const body = await request.json();

  try {
    const response = await fetch(`http://localhost:8081/roles/${id}`, {
      method: "PUT",
      headers: {
        "Content-Type": "application/json",
        Authorization: `Bearer ${token}`,
      },
      body: JSON.stringify(body),
    });

    if (response.ok) {
      return new Response(
        JSON.stringify({ success: true, message: "Rol actualizado correctamente" }),
        {
          status: 200,
          headers: {
            "HX-Trigger": "role-updated",
          },
        },
      );
    }

    const errorData = await response.json().catch(() => ({ message: "Error desconocido" }));
    return new Response(
      JSON.stringify({ success: false, message: errorData.message || "Error al actualizar rol" }),
      { status: response.status },
    );
  } catch (error) {
    return new Response(
      JSON.stringify({ success: false, message: "Error de conexión con el servidor" }),
      { status: 500 },
    );
  }
};

export const DELETE: APIRoute = async ({ params, cookies }) => {
  const { id } = params;
  const token = cookies.get("auth_token")?.value;

  try {
    const response = await fetch(`http://localhost:8081/roles/${id}`, {
      method: "DELETE",
      headers: {
        Authorization: `Bearer ${token}`,
      },
    });

    if (response.ok) {
      return new Response(null, { status: 204 });
    }

    return new Response(JSON.stringify({ message: "Error al eliminar rol" }), {
      status: response.status,
    });
  } catch (error) {
    return new Response(JSON.stringify({ message: "Internal error" }), { status: 500 });
  }
};
