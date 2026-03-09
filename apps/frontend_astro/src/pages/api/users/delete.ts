import type { APIRoute } from "astro";
import { deleteUser } from "@/lib/db";

export const DELETE: APIRoute = async ({ url }) => {
  const id = url.searchParams.get("id");

  if (id) {
    deleteUser(id);
    // HTMX espera una respuesta vacía o un fragmento para eliminar el elemento (hx-swap="outerHTML" con respuesta vacía elimina el target)
    return new Response(null, { status: 200 });
  }

  return new Response(JSON.stringify({ error: "ID no proporcionado" }), { status: 400 });
};
