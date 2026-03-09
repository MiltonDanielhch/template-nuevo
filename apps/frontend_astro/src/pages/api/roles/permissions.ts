import type { APIRoute } from "astro";

export const GET: APIRoute = async ({ cookies }) => {
  const token = cookies.get("auth_token")?.value;

  try {
    const response = await fetch("http://localhost:8081/permissions", {
      headers: {
        Authorization: `Bearer ${token}`,
      },
    });

    if (response.ok) {
      const data = await response.json();
      return new Response(JSON.stringify(data), { status: 200 });
    }

    return new Response(JSON.stringify({ message: "Error fetching permissions" }), {
      status: response.status,
    });
  } catch (error) {
    return new Response(JSON.stringify({ message: "Internal error" }), { status: 500 });
  }
};
