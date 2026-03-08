import { defineMiddleware } from "astro:middleware";

const PROTECTED_ROUTES = ["/dashboard"];
const AUTH_ROUTES = ["/login", "/register"];

export const onRequest = defineMiddleware(async (context, next) => {
  const { url, cookies, redirect } = context;
  const token = cookies.get("auth_token")?.value;

  const isProtectedRoute = PROTECTED_ROUTES.some((route) => url.pathname.startsWith(route));
  const isAuthRoute = AUTH_ROUTES.some((route) => url.pathname.startsWith(route));

  if (isProtectedRoute && !token) {
    return redirect("/login");
  }

  if (isAuthRoute && token) {
    return redirect("/dashboard");
  }

  return next();
});
