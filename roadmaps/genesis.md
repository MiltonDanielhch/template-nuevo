# 🌑 BLOQUE -I: GÉNESIS (Arquitectura de Intención) v2.0

**Objetivo:** Consolidar el "Laboratorio 3026". Pasar de la documentación a la estructura física de alto rendimiento basada en Rust y Contratos de Sintonía (Proto).

---

## 📊 Estado de Sintonía Actual

| Métrica | Valor | Notas |
|---------|-------|-------|
| **LoC (Netas)** | 121 | Lógica base de auditoría funcional |
| **Documentación** | 41.51KB | Arquitectura de intención completada |
| **Próximo Paso** | Fase G.1.2 | Materialización del Workspace Rust |

---

## 📅 Fase G.1: La Semilla Física (Workspace)

**Objetivo:** Transformar el `template1/` en el monorepo definitivo.

| # | Tarea | Destino | Estado |
|---|-------|---------|--------|
| 1 | Soberanía Git | `.gitignore` (Rust, Podman, SQLite, Bun) | ✅ |
| 2 | Estructura Crates | `/proto, /crates/core_logic, /crates/api_server` | ✅ |
| 3 | Workspace Cargo | `Cargo.toml` (Raíz con miembros de crates) | ✅ |

---

## 📅 Fase G.2: El Oráculo y Protocolos (Gobernanza)

**Objetivo:** Calibrar a la IA (Gemini/Cursor) para que respete la arquitectura hexagonal.

| # | Tarea | Archivo | Descripción | Estado |
|---|-------|---------|-------------|--------|
| 1 | Refactor `.cursorrules` | `.cursorrules` | Inyectar reglas de "Sintonía Hexagonal 3026" | ✅ |
| 2 | ADN Binario | `proto/auth.proto` | Definir el contrato inicial User/Role | ✅ |

---

## 📅 Fase G.3: El ADN Técnico (Estándares 3026)

**Objetivo:** Establecer las reglas de oro para el VPS de $5.

| # | Tarea | Archivo | Descripción | Estado |
|---|-------|---------|-------------|--------|
| 1 | ADR Update | `docs/DECISIONS.md` | Integrar ADR 0001 y 0002 (Podman + Rust) | ✅ |
| 2 | Standard Rust | `protocols/RUST_STANDARDS.md` | Reglas de Edition 2024 y Error Handling | ✅ |
| 3 | Infra Blueprint | `deploy/Caddyfile` | Configuración de SSL y Proxy para Podman | ✅ |

---

## 📅 Fase G.4: Implementación del Motor (Rust Core)

**Objetivo:** Crear el corazón que no depende de la base de datos ni de la web.

| # | Tarea | Ubicación | Descripción | Estado |
|---|-------|-----------|-------------|--------|
| 1 | Core Entities | `crates/core_logic/src/domain` | Implementar User y Email (Value Object) | ⏳ |
| 2 | Contracts | `crates/core_logic/src/interfaces` | Definir `IUserRepository` (Trait) | ⏳ |
| 3 | Just Command | `Justfile` | Comandos `just setup`, `just audit`, `just dev` | ⏳ |
| 4 | Pre-commit | `.pre-commit-config.yaml` | Bloquear commits que no pasen clippy | ⏳ |

---

## 📅 Fase G.5: Verificación de Primera Sintonía

| # | Verificación | Comando | Resultado Esperado |
|---|--------------|---------|-------------------|
| 1 | Integridad | `python3 ver-proyecto.py` | Detectar nuevas carpetas de crates |
| 2 | Compilación | `cargo check` | Cero errores en el workspace |
| 3 | Contratos | `buf lint` | Protobuf válido y sincronizado |