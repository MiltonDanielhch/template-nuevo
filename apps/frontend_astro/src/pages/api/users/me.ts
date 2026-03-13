import type { APIRoute } from "astro";

export const POST: APIRoute = async ({ request, cookies }) => {
  const data = await request.formData();
  const username = data.get("username");
  const email = data.get("email");
  const password = data.get("password");

  const token = cookies.get("auth_token")?.value;

  try {
    const updateData: any = {
      username: username?.toString(),
      email: email?.toString(),
    };

    if (password && password.toString().length > 0) {
      updateData.password = password.toString();
    }

    const response = await fetch("http://localhost:8081/api/v1/me", {
      method: "PUT",
      headers: {
        "Content-Type": "application/json",
        Authorization: `Bearer ${token}`,
      },
      body: JSON.stringify(updateData),
    });

    if (!response.ok) {
      const errorData = await response.json();
      return new Response(errorData.message || "Error al actualizar perfil", {
        status: response.status,
      });
    }

    const user = await response.json();

    return new Response(
      JSON.stringify({
        success: true,
        message: "Perfil actualizado correctamente",
        user,
      }),
      {
        headers: {
          "Content-Type": "application/json",
          "HX-Trigger": "profile-updated",
        },
      },
    );
  } catch (error) {
    console.error("Backend Error:", error);
    return new Response("Error de conexión con el servidor", { status: 500 });
  }
};
