import type { APIRoute } from "astro";

export const POST: APIRoute = async ({ request, cookies }) => {
  const data = await request.formData();
  const email = data.get("email");
  const password = data.get("password");

  // En un entorno real, aquí llamaríamos al backend Axum
  // const response = await fetch("http://localhost:8080/api/auth/login", { ... });

  // Por ahora simulamos una respuesta exitosa del backend
  if (email && password) {
    // Simulamos que el backend nos devuelve un token
    const token = `simulated_session_token_${Date.now()}`;

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
};
