import type { APIRoute } from "astro";

export const POST: APIRoute = async ({ request, cookies }) => {
  const data = await request.formData();
  const email = data.get("email");
  const password = data.get("password");

  try {
    const response = await fetch("http://localhost:8080/login", {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ email, password }),
    });

    if (!response.ok) {
      const errorData = await response.json();
      return new Response(
        JSON.stringify({ error: errorData.message || "Credenciales inválidas" }),
        { status: response.status },
      );
    }

    const loginData = await response.json();
    const token = loginData.token;

    cookies.set("auth_token", token, {
      path: "/",
      httpOnly: true,
      secure: import.meta.env.PROD,
      sameSite: "lax",
      maxAge: 60 * 60 * 24 * 7, // 1 semana
    });

    return new Response(null, {
      status: 200,
      headers: { "HX-Redirect": "/dashboard" },
    });
  } catch (error) {
    console.error("Login Error:", error);
    return new Response(JSON.stringify({ error: "Error de conexión con el servidor" }), {
      status: 500,
    });
  }
};
