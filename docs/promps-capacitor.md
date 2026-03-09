🚀 PROMPT DE SINTONÍA: LABORATORIO 3026 (Fase Móvil - Capacitor)
Actúa como Ingeniero App Mobile Senior especializado en WebView Architectures y Frontend Astro. Queremos extender el alcance del Laboratorio 3026 desde Tauri (Escritorio) a Android/iOS usando CapacitorJS.

---

🧭 1. CONTEXTO Y ESTADO (Lo que ya tenemos)
- ✅ **Backend Fuerte**: Rest API en Rust / Axum port 8080.
- ✅ **Frontend Web**: Astro + HTMX + Tailwind V4. Ya es responsivo y modular.
- ✅ **Comandos Tauri Configurados**: La app desktop opera por su lado independiente.

🧭 2. NUEVO OBJETIVO: INTEGRACIÓN MOBILE MÍNIMA

Vamos a dotar al proyecto de soporte celular sin comprometer la base de desarrollo web.

1. **Instalar Dependencias**: Solicítame instalar `@capacitor/core`, `@capacitor/cli` y `@capacitor/android`.
2. **Configurar Chasis PWA**: Pídeme generar la carpeta `android` con `npx cap add android` y ajustar el archivo `capacitor.config.ts`.
3. **Persistencia de Sesión Mobile**: Como reemplazo del Storage de Tauri, indica cómo crear un store usando `@capacitor/preferences` para guardar el Auth Token en el celular y que HTMX lo pueda leer al arrancar Astro.
4. **Protección IsNative**: Configurar nuestra UI en Astro para que los menús o estilos cambien ligeramente si `Capacitor.isNativePlatform()` es `TRUE` (por ejemplo, ocultar la barra de cerrar ventana típica de escritorio).

---

🛠️ 3. STACK Y REGLAS DE LA FASE MOBILE
- Regla de Oro: **Jamás tocar Rust para la UI**. Todo el front móvil ocurre en Astro.
- Herramienta de compilación híbrida: CapacitorJS versión 6+.
- No generar lógica compleja en JS (HTMX debe seguir haciendo el 95% del trabajo asíncrono hacia Rust).

---

🚀 ACCIÓN: Genera el primer checklist con los comandos exactos de Bun/NPM para inicializar el directorio android y modificar el `capacitor.config.ts`. Sé breve y directo.
