import type { APIRoute } from "astro";

export const GET: APIRoute = async ({ cookies }) => {
  const token = cookies.get("auth_token")?.value;

  try {
    const response = await fetch("http://localhost:8081/roles", {
      headers: {
        Authorization: `Bearer ${token}`,
      },
    });

    if (!response.ok) {
      return new Response("Error al obtener roles", { status: response.status });
    }

    const roles = await response.json();
    return new Response(JSON.stringify(roles), {
      headers: { "Content-Type": "application/json" },
    });
  } catch (error) {
    console.error("Backend Error:", error);
    return new Response("Error de conexión con el servidor", { status: 500 });
  }
};

export const POST: APIRoute = async ({ request, cookies }) => {
  const token = cookies.get("auth_token")?.value;
  const body = await request.json();

  try {
    const response = await fetch("http://localhost:8081/roles", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
        Authorization: `Bearer ${token}`,
      },
      body: JSON.stringify(body),
    });

    if (response.ok) {
      const data = await response.json();
      return new Response(
        JSON.stringify({ success: true, message: "Rol creado correctamente", data }),
        {
          status: 201,
          headers: { "Content-Type": "application/json" },
        },
      );
    }

    const errorData = await response.json().catch(() => ({ message: "Error desconocido" }));
    return new Response(
      JSON.stringify({ success: false, message: errorData.message || "Error al crear el rol" }),
      {
        status: response.status,
        headers: { "Content-Type": "application/json" },
      },
    );
  } catch (error) {
    console.error("Backend Connection Error:", error);
    return new Response(
      JSON.stringify({ success: false, message: "Error de conexión con el servidor" }),
      {
        status: 500,
        headers: { "Content-Type": "application/json" },
      },
    );
  }
};
