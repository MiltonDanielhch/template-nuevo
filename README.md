# 🧪 Laboratorio 3026

> **Arquitectura Hexagonal Soberana en Rust**
> *Rendimiento extremo, seguridad total y despliegue autónomo.*

![Status](https://img.shields.io/badge/Status-Activo-success)
![Rust](https://img.shields.io/badge/Rust-2024-orange)
![Axum](https://img.shields.io/badge/Axum-0.8-blue)
![Podman](https://img.shields.io/badge/Podman-Rootless-purple)

---

## 📖 Visión

El **Laboratorio 3026** es una plantilla de ingeniería de software diseñada para crear sistemas backend robustos, escalables y eficientes. Su filosofía se basa en:

1.  **Soberanía:** Capacidad de desplegarse en cualquier infraestructura (VPS de $5) sin depender de nubes propietarias.
2.  **Arquitectura Hexagonal:** El núcleo de negocio (`core_logic`) es puro y agnóstico a la tecnología externa.
3.  **Seguridad por Diseño:** Tipado estricto, sin `null`, sin `garbage collector` impredecible, y contenedores `rootless`.

## 🛠️ Stack Tecnológico

| Capa | Tecnología | Propósito |
|------|------------|-----------|
| **Lenguaje** | Rust (2024) | Seguridad de memoria y rendimiento nativo. |
| **API** | Axum 0.8 | Framework web asíncrono sobre Tokio. |
| **Persistencia** | SQLite (WAL) | Base de datos SQL embebida de alto rendimiento. |
| **Protocolo** | Protobuf | Contratos de datos binarios estrictos. |
| **Despliegue** | Podman + Caddy | Orquestación segura y HTTPS automático. |

---

## 🚀 Inicio Rápido

### Prerrequisitos
- [Rust](https://rustup.rs/) (latest stable)
- [Just](https://github.com/casey/just) (Command runner)
- [Podman](https://podman.io/) (o Docker)

### 1. Clonar y Configurar
```bash
git clone https://github.com/tu-usuario/laboratorio-3026.git
cd laboratorio-3026
cp .env.example .env
```

### 2. Levantar Entorno de Desarrollo
```bash
# Instala dependencias, migra la DB y levanta el servidor con Hot Reload
just dev
```
La API estará disponible en `http://localhost:8080`.

### 3. Ejecutar Pruebas
```bash
# Corre tests unitarios y de integración
cargo test --workspace
```

---

## 📂 Estructura del Proyecto

El proyecto utiliza un **Workspace de Cargo** para separar responsabilidades:

```text
├── crates/
│   ├── core_logic/       # 🧠 Dominio puro (Entidades, Casos de Uso)
│   ├── infra_db/         # 💾 Adaptadores SQL (Repositorios, Migraciones)
│   └── api_server/       # 📡 API Web (Handlers, Middlewares, DI)
├── proto/                # 🧬 Definiciones Protobuf (Contratos)
├── deploy/               # 🐳 Dockerfile, Compose y Caddyfile
└── docs/                 # 📚 Documentación detallada (ADRs, Roadmap)
```

## 🛡️ Características Implementadas

- [x] **Autenticación Completa:** Registro, Login, Logout, `/me`.
- [x] **Gestión de Sesiones:** Tokens seguros con validación en base de datos.
- [x] **Auditoría:** Registro automático de eventos de seguridad.
- [x] **Despliegue Dockerizado:** Imagen optimizada (<50MB) con `distroless`.

## 📚 Documentación

Para profundizar en las decisiones técnicas y el futuro del proyecto:

- [Roadmap Backend](roadmaps/backend.md)
- [Decisiones de Arquitectura (ADR)](docs/DECISIONS.md)
- [Manual de Pruebas](docs/TESTING.md)

---

**Laboratorio 3026** — *Código que perdura.*
