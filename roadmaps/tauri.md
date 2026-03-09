# 📱 ROADMAP: TAURI 3026 (Soberanía Multiplataforma)

**Stack:** Tauri 2.0 | Rust | Astro 5.0 | WinAppCli

---

## 🏗️ BLOQUE I: EL CHASIS (Setup & Config)
**Objetivo:** Configurar el entorno de escritorio nativo.

| # | Tarea | Ubicación / Descripción | Estado |
|---|-------|------------------------|:------:|
| 1 | Tauri Init & Workspace | `src-tauri/`: Inicializar y añadir al workspace de Cargo. | ✅ |
| 2 | Sintonía Astro | `tauri.conf.json`: Configurar `devPath` y `distDir` para Astro 5.0. | ✅ |
| 3 | Permissions | `src-tauri/capabilities/`: Definir permisos de red y acceso local. | ✅ |
| 4 | Window Logic | `src-tauri/src/main.rs`: Lógica de creación de ventanas nativas. | ⏳ |

---

## 🏗️ BLOQUE II: EL PUENTE (IPC & Commands)
**Objetivo:** Comunicación segura entre Astro y el Backend de Rust.

| # | Tarea | Ubicación / Descripción | Estado |
|---|-------|------------------------|:------:|
| 1 | Command Bridge | `src-tauri/src/commands/`: Comandos de Rust invocables desde el frontend. | ⏳ |
| 2 | Auth Sync | Sincronizar tokens de sesión entre el navegador y el almacenamiento seguro de Tauri. | ⏳ |
| 3 | Local FS Access | Lógica para guardar/leer archivos locales desde la app de escritorio. | ⏳ |

---

## 🏗️ BLOQUE III: LA ARMADURA (Windows Native - WinAppCli)
**Objetivo:** Integración profunda con Windows y empaquetado profesional.

| # | Tarea | Ubicación / Descripción | Estado |
|---|-------|------------------------|:------:|
| 1 | WinAppCli Init | Inicializar manifiestos y activos con WinAppCli. | ⏳ |
| 2 | App Identity | Generar identidad de paquete para notificaciones nativas. | ⏳ |
| 3 | MSIX Packaging | Generar instalador profesional MSIX firmado. | ⏳ |
| 4 | Protocol Handler | Configurar `lab3026://` para abrir la app desde la web. | ⏳ |

---

## 📊 Progreso General

```
BLOQUE I:  ████████████ 100% (4/4 completados)
BLOQUE II: ░░░░░░░░░░░░ 0% (0/3 completados)
BLOQUE III:░░░░░░░░░░░░ 0% (0/4 completados)
```
