🚀 PROMPT DE SINTONÍA: LABORATORIO 3026 (Fase 6 - El Puente IPC)
Actúa como Ingeniero de Software Senior y Arquitecto de Sistemas Nativos. El chasis de la aplicación de escritorio está completo y configurado. Es hora de construir el puente de comunicación entre el frontend y el backend nativo de Rust.

---

🧭 1. CONTEXTO Y ESTADO (Lo que ya tenemos)
- ✅ **Chasis de Tauri Completo**: Ventana configurada, permisos de red listos y workspace integrado.
- ✅ **App de Escritorio Funcional**: La aplicación se ejecuta y muestra el frontend de Astro.

- ✅ **Comunicación IPC Establecida**: Comandos creados y ejecutándose mediante `@tauri-apps/api/core`.
- ✅ **Sincronización de Sesión**: `tauri-plugin-store` configurado para persistir sesión.
- ✅ **FS Access**: Integración nativa a sistema de directorios y diálogos de Windows.
- ✅ **Bundle Windows**: Instaladores `.msi` y `.nsis` (MSIX) construidos satisfactoriamente.

🧭 2. PRÓXIMO OBJETIVO: MANTENIMIENTO Y PERFECCIONAMIENTO

Vamos a generar documentación y preparar manuales de calidad.

1. **Crear Manual de Pruebas**: Redactar `testing-tauri.md` que contenga comandos útiles para verificar que el binario compila y ejecuta sin errores.
2. **Clarificar Arquitectura de Estado**: Explicar claramente que la lógica de roles vive en el Backend de Axum (Rust) y no es necesario re-escribir lógica en Tauri.

---

🛠️ 3. STACK Y REGLAS (Fase 9 - Mantenimiento)
- Comunicación clara teórica.
- Explicar sin acoplar código innecesario.

---

🚀 ACCIÓN: Genera el archivo de testing y responde las dudas de arquitectura del puente Frontend/Tauri/Backend.
