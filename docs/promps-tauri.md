🚀 PROMPT DE SINTONÍA: LABORATORIO 3026 (Fase 5 - Soberanía Multiplataforma con Tauri)
Actúa como Ingeniero de Software Senior y Arquitecto Jefe de Sistemas Multiplataforma. El backend de Rust y el frontend de Astro están listos y conectados. Ahora es el momento de construir el chasis de escritorio nativo.

---

🧭 1. CONTEXTO Y ESTADO (Lo que ya tenemos)
- ✅ **Backend Rust (Axum 0.8)**: Autenticación, Usuarios y Telemetría funcionando.
- ✅ **Frontend Astro (SSR)**: HTMX + Alpine.js integrados con el backend real.
- ✅ **Auditoría Prime**: Proyecto ligero (<1MB) y estructurado en arquitectura hexagonal.

🧭 2. PRÓXIMO OBJETIVO: EL CHASIS (Tauri Setup)

Vamos a ejecutar:
1. **Tauri Init** - Inicializar `src-tauri` en la raíz del proyecto.
2. **Sintonía de Configuración** - Configurar `tauri.conf.json` para Astro 5.0 (`http://localhost:4321`).
3. **Permissions & Capabilities** - Definir qué puede hacer la app nativa (Red, FS).
4. **Window Logic** - Configurar la ventana principal de la aplicación nativa.

🧭 3. SIGUIENTE OBJETIVO: INTEGRACIÓN NATIVA (WinAppCli)

Vamos a implementar:
1. **App Identity** - Usar WinAppCli para dar identidad de paquete a la aplicación.
2. **Notificaciones Nativas** - Habilitar notificaciones interactivas de Windows.
3. **Packaging** - Generar el primer instalador profesional MSIX.

---

🛠️ 4. STACK Y REGLAS (Fase 7)
- Framework: Tauri 2.0 (Seguridad y Rendimiento).
- Backend Desktop: Rust nativo (Commands).
- Frontend Desktop: Astro (SSR en modo WebView).
- Reglas: Protocolo 3026, binarios ligeros, seguridad rootless.

---

🚀 ACCIÓN: Inicia la inicialización de Tauri en el proyecto.
