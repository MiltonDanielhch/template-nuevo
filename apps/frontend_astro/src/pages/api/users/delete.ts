import type { APIRoute } from "astro";

export const DELETE: APIRoute = async ({ url, cookies }) => {
  const id = url.searchParams.get("id");
  const token = cookies.get("auth_token")?.value;

  if (id) {
    try {
      const response = await fetch(`http://localhost:8081/users/${id}`, {
        method: "DELETE",
        headers: {
          Authorization: `Bearer ${token}`,
        },
      });

      if (!response.ok) {
        throw new Error("Error al eliminar usuario en el backend");
      }

      return new Response(null, { status: 200 });
    } catch (error) {
      console.error("Backend Error:", error);
      return new Response("Error de conexión con el servidor", { status: 500 });
    }
  }

  return new Response(JSON.stringify({ error: "ID no proporcionado" }), { status: 400 });
};
