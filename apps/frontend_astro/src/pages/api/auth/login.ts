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

    if (response.status === 429) {
      const retryAfter = response.headers.get("Retry-After") || "60";
      return new Response(
        JSON.stringify({
          error: `Demasiados intentos. Intenta de nuevo en ${retryAfter} segundos.`,
        }),
        {
          status: 429,
          headers: { "Retry-After": retryAfter },
        },
      );
    }

    if (response.ok) {
      const respData = await response.json();
      const token = respData.token;
      const remember = data.get("remember") === "on";

      // Si se marca "Recordarme", la cookie dura 30 días. Si no, es una "Session Cookie".
      const maxAge = remember ? 60 * 60 * 24 * 30 : undefined;

      cookies.set("auth_token", token, {
        path: "/",
        httpOnly: true,
        secure: import.meta.env.PROD,
        sameSite: "lax",
        ...(maxAge && { maxAge }),
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
