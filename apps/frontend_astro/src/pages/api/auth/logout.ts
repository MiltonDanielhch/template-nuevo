import type { APIRoute } from "astro";

export const POST: APIRoute = async ({ cookies }) => {
  cookies.delete("auth_token", { path: "/" });

  return new Response(null, {
    status: 302,
    headers: {
      Location: "/login",
    },
  });
};

export const GET: APIRoute = async ({ cookies, redirect }) => {
  cookies.delete("auth_token", { path: "/" });
  return redirect("/login");
};
