import type { APIRoute } from "astro";

export const POST: APIRoute = async ({ cookies }) => {
  cookies.delete("auth_token", { path: "/" });

  return new Response(null, {
    status: 200,
    headers: {
      "HX-Redirect": "/login",
    },
  });
};

export const GET: APIRoute = async ({ cookies, redirect }) => {
  cookies.delete("auth_token", { path: "/" });
  return redirect("/login");
};
