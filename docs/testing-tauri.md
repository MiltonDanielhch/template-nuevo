# 🧪 Manual de Pruebas - Desktop Tauri 3026

**Objetivo:** Guía maestra para validar la integración de la aplicación de escritorio, comandos útiles de compilación y pruebas de plugins IPC nativos.

---

## 🏗️ Nivel 1: Pruebas de Entorno y Configuración

Antes de compilar la aplicación nativa, debemos asegurar que el entorno Rust y las herramientas de compilación de Windows estén correctas.

### Comando de Verificación (Info)
```bash
cd apps/frontend_astro
bun run tauri info
```
O desde la raíz:
```bash
bunx tauri info
```

**Qué verifica:**
1.  **Node/Bun/Cargo:** Asegura que los gestores de paquetes y compiladores están en la versión adecuada.
2.  **Tauri CLI:** Verifica la versión de la CLI.
3.  **Rustup:** Muestra la versión de la toolchain de Rust instalada.

**Resultado Esperado:**
```text
[✔] Environment
    - OS: Windows 10.0.X
    ✔ rustc: 1.X.X
    ✔ cargo: 1.X.X
    ✔ rustup: 1.X.X
```

---

## 🔧 Nivel 2: Desarrollo con Hot-Reload (Dev Mode)

Para probar la comunicación entre el webview de Astro y el backend de Rust en tiempo de desarrollo.

### Comando Maestro Dev
```bash
cd apps/frontend_astro
bun run tauri dev
```

**Qué verifica:**
1.  **Levantar Frontend:** Automáticamente ejecuta `bun run dev` (Astro en puerto 4321).
2.  **Compilar Rust Debug:** Compila el binario `src-tauri` en modo debug.
3.  **Lanzar Ventana Nativa:** Abre la aplicación de escritorio en vivo, cualquier cambio en Astro se refleja al instante.

### Herramientas de Depuración (DevTools)
- Al igual que en Chrome, dentro de la ventana de Tauri en modo Dev puedes presionar `F12` o `Click Derecho -> Inspeccionar Elemento`.
- Podrás ver logs, requests network y dom elements.

---

## 📦 Nivel 3: Pruebas de Compilación (Build de Producción)

Para empaquetar la aplicación y generar los instaladores `.msi` y `.exe` (NSIS).

### Comando Maestro Build
```bash
cd apps/frontend_astro
bun run tauri build
```

**Qué verifica:**
1.  **Build Frontend Estático:** Ejecuta `bun run build` para pre-renderizar todo el HTML, JS y CSS en la carpeta `dist`.
2.  **Build Rust Release:** Compila el binario de Rust con optimización de producción (`--release`).
3.  **Wix y NSIS:** Descarga o utiliza las herramientas para generar los instaladores de Windows en `src-tauri/target/release/bundle`.

**Artefactos Generados (Dónde Encontrarlos):**
```bash
src-tauri/target/release/bundle/msi/Lab3026_0.1.0_x64_es-ES.msi
src-tauri/target/release/bundle/nsis/Lab3026_0.1.0_x64-setup.exe
```

---

## 🚀 Nivel 4: Pruebas del Puente IPC (Inter-Process Communication)

Verificar que las APIs nativas se comporten correctamente desde el frontend de Astro.

### 1. Comandos Nativos (Rust a Astro)
Asegúrese de probar el componente de saludo nativo en el Home:
1. Abrir la app en Dev.
2. Escribir un nombre en el campo "Puente Nativo".
3. Al dar clic en "Saludar", verificar en consola de Rust (terminal) o UI que la respuesta se armó en el Backend Desktop.

### 2. Sincronización de Sesión (Tauri Plugin Store)
La sesión no debe perderse si el usuario cierra el programa.
1. Haz Login en la app de Tauri.
2. Cierra la ventana ("X").
3. Vuelve a ejecutar `bun run tauri dev`.
4. Deberías estar logueado automáticamente. (El plugin intercepta `auth-token` de Astro y lo guarda en `store.json`).

### 3. File System Local (Tauri Plugin FS y Dialog)
Verificar guardado local.
1. Usar la tarjeta "Acceso a Sistema de Archivos Local" en Astro.
2. Clic en "Guardar Log".
3. Se debe abrir la ventana *Nativa de Windows* para elegir directorio.
4. Elije el escritorio.
5. Clica en Leer Archivo y selecciona el que acabas de guardar; Astro te indicará su peso en caracteres.

---

## 🛡️ Nivel 5: Preguntas y Arquitectura de Tauri (Consultoría)

### Q: "Si actualizo roles o permisos en Astro o el Backend (Rust Api), ¿tengo que actualizar también Tauri y es más complicado?"

**Respuesta: NO. 🎉**

**Tauri NO duplica tu backend.** Su única misión en este nivel (Arquitectura 3026) es ser un "Navegador de Alta Seguridad con esteroides".

- **La Verdadera API (Axum/Rust 8080):** Es la que maneja la base de datos (SQLite), el JWT de login, y sabe si eres Admin, Supervisor o Cajero.
- **Astro (4321):** Dibuja la pantalla basándose en lo que la API de Rust diga.
- **Tauri:** Simplemente empaqueta Astro en un `.exe`.

Cuando instalas el `tauri-plugin-store` que hicimos en el "Auth Sync", Tauri lo único que guardó en tu computadora fue el JWT (el string del Token). Mantiene a tu Astro hidratado para que no tengas que iniciar sesión mil veces, igualito a como lo haría Google Chrome con localStorage, pero a nivel de sistema operativo.

**Por lo tanto, si en un futuro decides añadir un Rol "Finanzas" en tu BD:**
Solo editas la API (Axum) y el Frontend (Astro). A Tauri no tienes que tocarle **ninguna** línea de código. La próxima vez que generes el instalador con `bun run tauri build`, Tauri agarrará la última versión de tu Astro y la envolverá solita. 

**Resumen:** Tauri es el chasis (el auto). Tu Backend y Astro son el Motor. Si mejoras el motor, el auto sigue corriendo perfectamente sin tener que rediseñarlo.
