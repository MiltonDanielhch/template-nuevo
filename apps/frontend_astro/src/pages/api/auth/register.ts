import type { APIRoute } from "astro";

export const POST: APIRoute = async ({ request, cookies }) => {
  const data = await request.formData();
  const username = data.get("username");
  const email = data.get("email");
  const password = data.get("password");
  const confirmPassword = data.get("confirmPassword");

  if (password !== confirmPassword) {
    return new Response(JSON.stringify({ error: "Las contraseñas no coinciden" }), { status: 400 });
  }

  try {
    // 1. Registrar al usuario en el backend Rust
    const registerResponse = await fetch("http://localhost:8081/register", {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ email, password, username }),
    });

    if (registerResponse.status === 429) {
      const retryAfter = registerResponse.headers.get("Retry-After") || "60";
      return new Response(
        `<div class="bg-destructive/10 text-destructive text-sm p-3 rounded-md mb-4 animate-in fade-in slide-in-from-top-1">
          Demasiados intentos. Intenta de nuevo en ${retryAfter} segundos.
        </div>`,
        {
          status: 429,
          headers: { "Content-Type": "text/html", "Retry-After": retryAfter },
        },
      );
    }

    if (!registerResponse.ok) {
      const errorData = await registerResponse.json();
      const errorMessage = errorData.error || "Error en el registro";

      // Si es una petición HTMX, devolvemos un pequeño fragmento HTML para el error
      return new Response(
        `<div class="bg-destructive/10 text-destructive text-sm p-3 rounded-md mb-4 animate-in fade-in slide-in-from-top-1">
          ${errorMessage}
          <button class="ml-2 underline" onclick="window.location.reload()">Reintentar</button>
        </div>`,
        {
          status: 200,
          headers: { "Content-Type": "text/html" },
        },
      );
    }

    // 2. Si el registro fue exitoso, hacemos login automáticamente para obtener el token
    const loginResponse = await fetch("http://localhost:8081/login", {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ email, password }),
    });

    if (!loginResponse.ok) {
      return new Response(null, {
        status: 200,
        headers: { "HX-Redirect": "/login" },
      });
    }

    const loginData = await loginResponse.json();
    const token = loginData.token;

    cookies.set("auth_token", token, {
      path: "/",
      httpOnly: true,
      secure: import.meta.env.PROD,
      sameSite: "lax",
      maxAge: 60 * 60 * 24 * 7, // 1 semana
    });

    return new Response(null, {
      status: 201,
      headers: { "HX-Redirect": "/dashboard" },
    });
  } catch (error) {
    console.error("Auth Error:", error);
    return new Response(JSON.stringify({ error: "Error de conexión con el servidor" }), {
      status: 500,
    });
  }
};
