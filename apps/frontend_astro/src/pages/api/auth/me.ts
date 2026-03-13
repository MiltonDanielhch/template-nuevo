import type { APIRoute } from "astro";

export const GET: APIRoute = async ({ cookies }) => {
  const token = cookies.get("auth_token")?.value;

  if (!token) {
    return new Response(JSON.stringify({ error: "No autorizado" }), {
      status: 401,
      headers: { "Content-Type": "application/json" },
    });
  }

  try {
    const response = await fetch("http://localhost:8081/api/v1/me", {
      headers: {
        Authorization: `Bearer ${token}`,
      },
    });

    if (!response.ok) {
      return new Response(JSON.stringify({ error: "Sesión inválida" }), {
        status: 401,
        headers: { "Content-Type": "application/json" },
      });
    }

    const userData = await response.json();
    return new Response(JSON.stringify(userData), {
      status: 200,
      headers: { "Content-Type": "application/json" },
    });
  } catch (error) {
    console.error("Me Error:", error);
    return new Response(JSON.stringify({ error: "Error de conexión con el servidor" }), {
      status: 500,
    });
  }
};

export const HEAD: APIRoute = async ({ cookies }) => {
  const token = cookies.get("auth_token")?.value;
  if (!token) return new Response(null, { status: 401 });

  try {
    const response = await fetch("http://localhost:8081/api/v1/me", {
      headers: { Authorization: `Bearer ${token}` },
    });
    return new Response(null, { status: response.ok ? 200 : 401 });
  } catch {
    return new Response(null, { status: 500 });
  }
};
