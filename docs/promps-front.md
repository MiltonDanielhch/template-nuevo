🚀 PROMPT DE SINTONÍA: LABORATORIO 3026 (Fase 4 - Telemetría y Consolidación)
Actúa como Ingeniero de Software Senior y Arquitecto Jefe. La integración real con el backend de Rust (Axum 0.8) y el sistema de telemetría están completos.

---

🧭 1. CONTEXTO Y ESTADO (Lo que ya tenemos)
- ✅ **Auth Real**: Astro conectado a los endpoints de Rust para Login/Register/Me.
- ✅ **CRUD Real**: Gestión de usuarios conectada al backend de Rust (Listar, Editar, Eliminar).
- ✅ **Telemetría Frontend**: Logger de HTMX y listeners de errores en `MainLayout`.
- ✅ **Fix de Alpine+HTMX**: Uso de `htmx.process()` para atributos dinámicos en modales.
- ✅ **Sincronía de Datos**: Unificación del campo `username` en todo el flujo.

🧭 2. PRÓXIMO OBJETIVO: SOBERANÍA MULTIPLATAFORMA (Tauri Bridge)

Vamos a crear:
1. **Tauri Config** - Configuración de `tauri.conf.json` para Astro 5.0.
2. **Window Manager** - Gestión de ventanas nativas y menús.
3. **Local Storage Bridge** - Persistencia sincronizada entre Web y Desktop.
4. **Build Pipeline** - Generación de binarios `.exe` para Windows.

---

🧭 3. SIGUIENTE OBJETIVO: GESTIÓN DE ACCESO (RBAC UI)

Vamos a implementar:
1. **Role-Based Guards** - Componentes que ocultan elementos según el rol del usuario (Admin/User).
2. **Permissions Editor** - Interfaz para asignar roles a usuarios (conectado al Bloque VI del Backend).

---

🛠️ 4. STACK Y REGLAS (Fase 6)
- Integración: Telemetría de HTMX para depuración en vivo.
- UX: Feedback visual en modales y tablas tras acciones exitosas.
- Reglas: Protocolo 3026, código limpio y modular.

---

🚀 ACCIÓN: Inicia la preparación del entorno Tauri para escritorio.
