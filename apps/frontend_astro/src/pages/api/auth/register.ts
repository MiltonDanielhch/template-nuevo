import type { APIRoute } from "astro";

export const POST: APIRoute = async ({ request, cookies }) => {
  const data = await request.formData();
  const email = data.get("email");
  const password = data.get("password");
  const confirmPassword = data.get("confirmPassword");

  // Validación básica
  if (password !== confirmPassword) {
    return new Response(JSON.stringify({ error: "Las contraseñas no coinciden" }), { status: 400 });
  }

  // En un entorno real, aquí llamaríamos al backend Axum
  // const response = await fetch("http://localhost:8080/api/auth/register", { ... });

  if (email && password) {
    // Simulamos que el backend nos devuelve un token tras el registro
    const token = `simulated_session_token_${Date.now()}`;

    cookies.set("auth_token", token, {
      path: "/",
      httpOnly: true,
      secure: import.meta.env.PROD,
      sameSite: "lax",
      maxAge: 60 * 60 * 24 * 7, // 1 semana
    });

    return new Response(null, {
      status: 201, // Created
      headers: {
        "HX-Redirect": "/dashboard",
      },
    });
  }

  return new Response(JSON.stringify({ error: "Datos de registro incompletos" }), { status: 400 });
};
