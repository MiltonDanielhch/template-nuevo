import type { APIRoute } from "astro";

export const prerender = false;

export const POST: APIRoute = async ({ request, cookies }) => {
  const token = cookies.get("auth_token")?.value;

  if (!token) {
    return new Response(JSON.stringify({ error: "No autorizado" }), {
      status: 401,
      headers: { "Content-Type": "application/json" },
    });
  }

  try {
    const formData = await request.formData();
    const file = formData.get("avatar") as File | null;

    if (!file || file.size === 0) {
      return new Response(JSON.stringify({ error: "No se proporcionó imagen" }), {
        status: 400,
        headers: { "Content-Type": "application/json" },
      });
    }

    // Validar tipo de archivo
    const allowedTypes = ["image/jpeg", "image/png", "image/gif", "image/webp"];
    if (!allowedTypes.includes(file.type)) {
      return new Response(JSON.stringify({ error: "Tipo de archivo no permitido" }), {
        status: 400,
        headers: { "Content-Type": "application/json" },
      });
    }

    // Validar tamaño (max 2MB)
    if (file.size > 2 * 1024 * 1024) {
      return new Response(JSON.stringify({ error: "La imagen debe ser menor a 2MB" }), {
        status: 400,
        headers: { "Content-Type": "application/json" },
      });
    }

    // Convertir a array buffer
    const arrayBuffer = await file.arrayBuffer();
    const uint8Array = new Uint8Array(arrayBuffer);

    // Generar nombre único
    const ext = file.name.split(".").pop() || "jpg";
    const filename = `avatar_${Date.now()}_${Math.random().toString(36).substring(7)}.${ext}`;

    // Guardar en la carpeta pública
    const fs = await import("node:fs/promises");
    const path = await import("node:path");

    const publicDir = path.join(process.cwd(), "apps/frontend_astro/public");
    const uploadsDir = path.join(publicDir, "uploads");

    // Crear directorio si no existe
    await fs.mkdir(uploadsDir, { recursive: true });

    const filePath = path.join(uploadsDir, filename);
    await fs.writeFile(filePath, uint8Array);

    const url = `/uploads/${filename}`;

    return new Response(
      JSON.stringify({
        success: true,
        url,
        message: "Imagen subida correctamente",
      }),
      {
        status: 200,
        headers: { "Content-Type": "application/json" },
      },
    );
  } catch (error) {
    console.error("Error uploading avatar:", error);
    return new Response(JSON.stringify({ error: "Error al subir imagen" }), {
      status: 500,
      headers: { "Content-Type": "application/json" },
    });
  }
};
