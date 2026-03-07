# ADR 0001: STACK TECNOLÓGICO "EL LABORATORIO 3026"

**Estado:** 🟢 Activo (Sintonía de Seguridad Total)

**Fecha Última Actualización:** 2026-03-07

---

## Visión

Máximo rendimiento, seguridad Rootless y eficiencia en costos ($5 VPS).

---

## 1. Matriz de Componentes (Edición Soberana)

| Capa | Tecnología | Razón de Sintonía (Por qué aquí y ahora) |
|------|------------|------------------------------------------|
| **Lenguaje Core** | Rust (Edition 2024) | El compilador es tu socio. Seguridad total contra race conditions y fugas de memoria. |
| **Lógica Rápida** | Bun (TypeScript) | Orquestación ágil y scripts de alto rendimiento (Frontend/Scripts). |
| **API / Backend** | Axum 0.8 | Construido sobre tokio 1.43. Estándar moderno que elimina macros complejas (`#[async_trait]`) para un código más limpio. |
| **Persistencia** | SQLite (WAL) → SurrealDB | Evolutivo: Empezamos con SQLite + Litestream (backups S3). Escalamos a SurrealDB (Multi-model) con TiKV para clusters masivos. |
| **Validación** | ArkType | Sintonía de Velocidad: 100x más rápido que Zod. Sintaxis idéntica a TypeScript puro. |
| **Frontend UI** | Astro + Tailwind + Ark UI | Arquitectura de Islas. Cero JavaScript innecesario. SEO perfecto. |
| **Interactividad** | HTMX | Permite que el servidor (Axum) envíe fragmentos HTML directamente. Elimina la necesidad de estados complejos en el cliente. |
| **Desktop** | Tauri 2.0 | Reutiliza la UI de Astro. Acceso nativo a disco/red con la velocidad de Rust. |
| **Móvil** | React Native | Puente nativo para iOS/Android. Tipado estricto compartido. |
| **Comunicación** | ConnectRPC (Protobuf) | Sintonía de Protocolo: Contratos binarios estrictos (`.proto`) que definen la verdad única del sistema. Compatible con HTTP/3. |
| **Contenedores** | Podman (Rootless) | Seguridad: Sin daemon, sin privilegios root. Usamos `podman-compose` para orquestación local y producción. |
| **Despliegue** | Kamal 2 + Podman | Orquestación simple que usa Podman para máxima seguridad en el VPS. |
| **Infraestructura** | VPS + Caddy | Caddy gestiona SSL/HTTP3 automático y actúa como Reverse Proxy hacia el contenedor de Rust. |
| **Doc de API** | Scalar | Sintonía Visual: Genera documentación interactiva y preciosa desde el código. Sustituye a Swagger con una interfaz moderna y rápida. |

---

## 2. Por qué Podman en el Código 3026

Al elegir Podman, estamos aplicando la filosofía de "Menos es Más":

### Seguridad por Diseño (Rootless)
Si alguien lograra comprometer tu contenedor, no tendría acceso root al VPS. Esto es vital cuando corres todo en un solo servidor pequeño.

### Eficiencia de Recursos
Al no tener un daemon (dockerd) consumiendo memoria constantemente, liberas valiosos MB de RAM para tu base de datos y tu app en Rust.

### Compatibilidad Docker
Podman implementa la misma interfaz CLI que Docker. Nuestros `Dockerfiles` funcionan perfectamente, pero los ejecutamos con `podman build` y `podman run`.

---

## 3. El ADN de Validación y Tipado (ArkType + gRPC)

Para que la IA no alucine y el sistema sea 100% coherente:

1. **Contrato:** Definimos el mensaje en Protobuf (`proto/auth.proto`).
2. **Servidor:** Rust genera las estructuras de datos y valida tipos en tiempo de compilación.
3. **Cliente TS:** Generamos el código TS y usamos ArkType para validar que la respuesta del servidor cumple exactamente con lo esperado.

---

# ADR 0002: Arquitectura de Sintonía Hexagonal (3026)

**Estado:** 🟢 Activo

**Fecha:** 2026-03-07

---

## Contexto

Necesitamos un sistema modular que soporte Autenticación y, en el futuro, Gestión de Acceso (RBAC), optimizando recursos en un VPS de $5.

---

## 1. El Nombre: "Sintonía Hexagonal"

Elegimos este nombre porque:

- **Hexagonal (Puertos y Adaptadores):** Representa la libertad. El núcleo (`core_logic`) no está "atado" a nada externo.
- **Sintonía:** Es el concepto del Código 3026. Todas las capas (Backend, Frontend, DB, Infra) vibran en la misma frecuencia gracias a los contratos de datos (Protobuf) y el Workspace de Cargo.

---

## 2. Por qué elegimos esta mezcla (Hexagonal + Clean Code)

### A. Independencia de la Base de Datos (Clean Code)

En el desarrollo tradicional, la base de datos dicta el código. Aquí, el Dominio dicta las reglas.

**Beneficio:** Empezamos con SQLite para ahorrar costos y simplicidad. Si el proyecto escala, cambiamos a PostgreSQL o SurrealDB sustituyendo solo el crate `infra_db`, sin tocar una sola línea de la lógica de negocio en `core_logic`.

### B. El Núcleo Inmortal (Hexagonal)

El "Hexágono Interior" (`core_logic`) es puro Rust. No sabe qué es Axum, no sabe qué es HTTP.

**Beneficio:** Si en el futuro queremos cambiar la API REST por gRPC puro o una CLI, el código de "Registrar Usuario" sigue siendo exactamente el mismo.

### C. Testabilidad Total

Al usar Puertos (Traits/Interfaces), podemos probar la lógica de negocio sin necesidad de una base de datos real. Usamos "Mocks" o implementaciones en memoria para tests unitarios ultrarrápidos.

---

## 3. Estrategia de Implementación (MVP vs Futuro)

Para evitar la parálisis por análisis, hemos dividido la implementación:

| Capa | Estado Actual (MVP Auth) | Estado Futuro (RBAC Completo) |
|------|--------------------------|-------------------------------|
| **Base de Datos** | Tablas `users`, `sessions`, `roles`, `permissions` creadas. | Igual (Ya existen). |
| **Core Logic** | Entidades `User`, `Session`. Casos de uso `Register`, `Login`. | Añadir `Role`, `Permission` y lógica de asignación. |
| **API Server** | Endpoints `/register`, `/login`, `/me`. Middleware `CurrentUser`. | Añadir endpoints `/roles` y Middleware `RequirePermission`. |

---

## 4. Estructura del Workspace (Mapa Mental Completo)

```text
laboratorio_3026/
├── proto/                          # EL ADN (Contratos Binarios)
├── crates/                         # EL MOTOR (Rust Workspace)
│   ├── core_logic/                 # CORAZÓN (Lógica Pura y Agnóstica)
│   │   ├── domain/                 # Capa 1: Reglas de Oro
│   │   │   ├── entities/           # User, Role, Permission
│   │   │   └── value_objects/      # Email, UserId
│   │   └── application/            # Capa 2: Orquestación (Casos de Uso)
│   ├── infra_db/                   # INFRAESTRUCTURA (Adaptadores de Salida)
│   │   └── persistence/sqlite/     # Adaptador Actual (SQLx)
│   └── api_server/                 # ENTRADA Y PEGAMENTO (Adaptadores Primarios)
│       ├── config/di.rs            # Inyección de Dependencias
│       └── entry_points/           # Controladores Axum
├── apps/                           # INTERFACES DE USUARIO (Futuro)
│   ├── frontend_astro/             # Web (Astro + HTMX + ArkType)
│   └── desktop_tauri/              # App Nativa (Rust + WebView)
├── deploy/                         # DESPLIEGUE SOBERANO
│   ├── podman-compose.yml          # Orquestación Rootless
│   ├── Dockerfile                  # Multi-stage build para el binario Rust
│   └── Caddyfile                   # Reverse Proxy y SSL Automático
└── Justfile                        # Centro de Mando
```

---

# 🛠️ Paso a Paso: El Camino del Programador 3026

No intentes programar todo a la vez. Sigue este orden para mantener la Sintonía:

## Paso 1: Definir el ADN (`/proto`)

Antes de Rust o TS, escribe el archivo `.proto`.

**Por qué:** Aquí decides cómo se llama un usuario y qué permisos existen. Al compilarlo, tendrás tipos de datos idénticos en el Backend y el Frontend.

## Paso 2: El Corazón del Dominio (`crates/core_logic/domain`)

Escribe tus `entities.rs` y `value_objects.rs`.

**Acción:** Crea la lógica de validación. Por ejemplo: un Role no puede estar vacío, un Email debe tener una @.

**Nota:** Aquí no pienses en bases de datos, solo en "objetos reales".

## Paso 3: Los Casos de Uso (`crates/core_logic/application`)

Escribe `register_user.rs`.

**Acción:** Define los pasos:
1. Validar datos →
2. Verificar si existe en el Repo →
3. Encriptar password →
4. Guardar.

## Paso 4: La Memoria (`crates/infra_db`)

Implementa el repositorio con SQLx.

**Acción:** Escribe el SQL para insertar usuarios y roles. Usa los `mappers.rs` para asegurarte que lo que sale de la DB se convierta en una Entidad de tu Dominio.

## Paso 5: La Puerta de Entrada (`crates/api_server`)

Configura Axum.

**Acción:** Crea un endpoint `POST /register`. En el `main.rs`, inyecta la base de datos dentro de tu caso de uso. Aquí es donde ocurre la magia de la unión.

## Paso 6: La Interfaz (`apps/frontend_astro`) [Fase Futura]

Usa ArkType para validar los formularios en el navegador y envía la petición a Axum.

**Acción:** Si el usuario tiene el rol "Admin", HTMX le mostrará el botón de "Gestionar Permisos".

## Paso 7: El Despliegue (`deploy/`)

Levanta todo con `podman-compose up`.

**Acción:** Tu VPS de $5 ahora corre un sistema de nivel bancario, protegido por Caddy con HTTPS automático.

---

# 🎓 Explicación Detallada para aprender el "Código 3026"

Para que el laboratorio funcione, hemos dividido la inteligencia en capas. Aquí te explico qué hace cada una y por qué la configuramos así:

## 1. El ADN (`/proto`)

Es el **Contrato Social**. Antes de escribir código, definimos cómo se comunican las partes. Al usar Protobuf, si cambias un campo en el usuario, el compilador te avisará tanto en Rust como en TypeScript.

&gt; **Sin ADN, no hay sintonía.**

## 2. El Dominio (`/domain`)

Es la **Capa 1 (Pura)**. Aquí vive el "Potencial Humano" del código.

- **Value Objects:** No permitas que un "Email" sea un simple texto. Crea un objeto que se valide a sí mismo. Si el código tiene un Email, es porque es real.
- **Interfaces:** Son los "Enchufes". El dominio dice: "Necesito guardar un usuario, no me importa si es en un archivo de texto o en la NASA".

## 3. La Infraestructura (`/infra_db`)

Es el **Cuerpo Técnico**. Aquí es donde vive SQLite.

- **Mappers:** Son vitales. La base de datos guarda datos "sucios" o planos. El Mapper los transforma en "Objetos de Dominio" con toda su lógica. Es el traductor de la realidad técnica a la lógica pura.

## 4. El Composition Root (`/api_server/config`)

Es la **Magia de la Unión**. Es donde ocurre la Inyección de Dependencias.

En `di.rs`, tú le dices al sistema: "Para este laboratorio, usa SQLite y Bcrypt".

Si mañana quieres usar SurrealDB, solo cambias una línea en este archivo. El resto de las 10,000 líneas de código del proyecto no cambian.

&gt; **Eso es libertad.**

## 5. Entry Points (`/entry_points`)

Son las **Antenas**. Tu sistema puede ser escuchado por:
- Una API Web (Axum)
- Una terminal (CLI)
- Un robot (Queues)

Todos llaman a los mismos "Casos de Uso".
