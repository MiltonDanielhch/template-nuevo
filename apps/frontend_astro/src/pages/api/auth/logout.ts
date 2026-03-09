import type { APIRoute } from "astro";

export const POST: APIRoute = async ({ cookies }) => {
  const token = cookies.get("auth_token")?.value;

  if (token) {
    try {
      await fetch("http://localhost:8080/logout", {
        method: "POST",
        headers: { Authorization: `Bearer ${token}` },
      });
    } catch (error) {
      console.error("Logout Backend Error:", error);
    }
  }

  cookies.delete("auth_token", { path: "/" });

  return new Response(null, {
    status: 200,
    headers: { "HX-Redirect": "/login" },
  });
};
