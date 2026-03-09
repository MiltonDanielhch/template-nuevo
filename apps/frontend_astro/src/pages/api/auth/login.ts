import type { APIRoute } from "astro";

export const POST: APIRoute = async ({ request, cookies }) => {
  const data = await request.formData();
  const email = data.get("email");
  const password = data.get("password");

  try {
    const response = await fetch("http://localhost:8081/login", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({ email, password }),
    });

    if (response.ok) {
      const data = await response.json();
      const token = data.token;

      cookies.set("auth_token", token, {
        path: "/",
        httpOnly: true,
        secure: import.meta.env.PROD,
        sameSite: "lax",
        maxAge: 60 * 60 * 24 * 7, // 1 semana
      });

      return new Response(null, {
        status: 200,
        headers: {
          "HX-Redirect": "/dashboard",
        },
      });
    }

    return new Response(JSON.stringify({ error: "Credenciales inválidas" }), { status: 401 });
  } catch (error) {
    console.error("Login Error:", error);
    return new Response("Error de conexión con el servidor", { status: 500 });
  }
};
