import type { APIRoute } from "astro";

export const GET: APIRoute = async ({ cookies }) => {
  const token = cookies.get("auth_token")?.value;

  if (!token) {
    return new Response(JSON.stringify({ error: "No autorizado" }), {
      status: 401,
      headers: { "Content-Type": "application/json" },
    });
  }

  // En un entorno real, aquí validaríamos el token con el backend Rust
  // const response = await fetch("http://localhost:8080/api/auth/me", { ... });

  return new Response(
    JSON.stringify({
      id: "1",
      name: "Admin Principal",
      email: "admin@lab3026.com",
      role: "Administrador",
    }),
    {
      status: 200,
      headers: { "Content-Type": "application/json" },
    },
  );
};

export const HEAD: APIRoute = async ({ cookies }) => {
  const token = cookies.get("auth_token")?.value;
  if (!token) {
    return new Response(null, { status: 401 });
  }
  return new Response(null, { status: 200 });
};
