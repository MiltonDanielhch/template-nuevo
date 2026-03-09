🚀 PROMPT DE SINTONÍA: LABORATORIO 3026 (Fase 5.2 - Permisos Nativos)
Actúa como Ingeniero de Software Senior y Arquitecto de Seguridad. La aplicación de escritorio ya compila y se ejecuta. Ahora debemos definir su "pasaporte" de seguridad.

---

🧭 1. CONTEXTO Y ESTADO (Lo que ya tenemos)
- ✅ **Chasis de Tauri**: `src-tauri` inicializado y funcionando.
- ✅ **Workspace Integrado**: `src-tauri` es parte del workspace de Cargo.
- ✅ **App en Ventana**: La aplicación de Astro se renderiza correctamente en una ventana nativa.

🧭 2. PRÓXIMO OBJETIVO: CAPABILITIES (Permisos Explícitos)

Por defecto, Tauri es "Zero Trust" (Confianza Cero). No puede hacer nada. Debemos darle permisos explícitos.

Vamos a ejecutar:
1. **Crear `capabilities/`**: Crear la estructura de directorios para los permisos.
2. **Definir Permisos de Red**: Permitir que la app se comunique con `http://localhost:8080` (nuestro backend de Axum).
3. **Habilitar `window-manager`**: Permitir que el código de Rust controle las ventanas (crear, cerrar, etc.).
4. **Configurar `tauri.conf.json`**: Activar las capabilities en la configuración principal.

---

🛠️ 3. STACK Y REGLAS (Fase 7.2)
- Seguridad: Principio de Mínimo Privilegio. Solo damos los permisos estrictamente necesarios.
- Modularidad: Separar permisos de desarrollo y producción si es necesario.

---

🚀 ACCIÓN: Inicia la creación y configuración de los archivos de capabilities.
