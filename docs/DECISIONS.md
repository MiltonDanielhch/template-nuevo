# ADR 0001: STACK TECNOLÓGICO "EL LABORATORIO 3026"

**Estado:** 🟢 Activo (Sintonía de Seguridad Total)

**Fecha:** 2026-03-03

---

## Visión

Máximo rendimiento, seguridad Rootless y eficiencia en costos ($5 VPS).

---

## 1. Matriz de Componentes (Edición Soberana)

| Capa | Tecnología | Razón de Sintonía (Por qué aquí y ahora) |
|------|------------|------------------------------------------|
| **Lenguaje Core** | Rust (Edition 2024) | El compilador es tu socio. Seguridad total contra race conditions y fugas de memoria. |
| **Lógica Rápida** | Bun (TypeScript) | Orquestación ágil y scripts de alto rendimiento. |
| **API / Backend** | Axum | Construido sobre tokio. Es el estándar de 2026 para APIs de alto rendimiento. |
| **Persistencia** | SQLite (WAL) → SurrealDB | Evolutivo: Empezamos con SQLite + Litestream (backups S3). Escalamos a SurrealDB (Multi-model) con TiKV para clusters masivos. |
| **Validación** | ArkType | Sintonía de Velocidad: 100x más rápido que Zod. Sintaxis idéntica a TypeScript puro. |
| **Frontend UI** | Astro + Tailwind + Ark UI | Arquitectura de Islas. Cero JavaScript innecesario. SEO perfecto. |
| **Interactividad** | HTMX | Permite que el servidor (Axum) envíe fragmentos HTML directamente. Elimina la necesidad de estados complejos en el cliente. |
| **Desktop** | Tauri 2.0 | Reutiliza la UI de Astro. Acceso nativo a disco/red con la velocidad de Rust. |
| **Móvil** | React Native | Puente nativo para iOS/Android. Tipado estricto compartido. |
| **Comunicación** | ConnectRPC (Protobuf) | Sintonía de Protocolo: Contratos binarios estrictos que funcionan nativamente en Web, Móvil y Desktop sin necesidad de proxies pesados. Compatible con HTTP/3. |
| **Contenedores** | Podman (Rootless) | Seguridad: Sin daemon, sin privilegios root, compatible con Docker CLI. |
| **Despliegue** | Kamal 2 + Podman | Orquestación simple que ahora usa Podman para máxima seguridad en el VPS. |
| **Infraestructura** | VPS + Pingora/Caddy + Cloudflare | Caddy gestiona SSL/HTTP3 sobre contenedores de Podman. Rendimiento de Cloudflare (Rust). |
| **Doc de API** | Scalar | Sintonía Visual: Genera documentación interactiva y preciosa desde el código. Sustituye a Swagger con una interfaz moderna y rápida. |

---

## 2. Por qué Podman en el Código 3026

Al elegir Podman, estamos aplicando la filosofía de "Menos es Más":

### Seguridad por Diseño (Rootless)
Si alguien lograra comprometer tu contenedor, no tendría acceso root al VPS. Esto es vital cuando corres todo en un solo servidor pequeño.

### Eficiencia de Recursos
Al no tener un daemon (dockerd) consumiendo memoria constantemente, liberas valiosos MB de RAM para tu base de datos y tu app en Rust.

### Pods de Kubernetes
Podman permite agrupar contenedores en "Pods" localmente. Esto facilita la transición a una infraestructura más grande si el proyecto explota en tráfico.

### Sustitución Transparente
Puedes hacer un alias `docker=podman` y Kamal 2 seguirá funcionando igual de bien, pero con una base mucho más sólida.

---

## 3. El ADN de Validación y Tipado (ArkType + gRPC)

Para que la IA no alucine y el sistema sea 100% coherente:

1. **Contrato:** Definimos el mensaje en Protobuf.
2. **Servidor:** Rust genera las estructuras de datos.
3. **Cliente TS:** Generamos el código TS y usamos ArkType para validar que la respuesta del servidor cumple exactamente con lo esperado en tiempo de ejecución, con una latencia de validación casi nula.

---

## 4. Consecuencias Positivas

- **Inmunidad al Vendor Lock-in:** Podman es estándar OCI. Puedes mover tus imágenes a cualquier lado.

- **Costos Fijos:** Con $5 al mes tienes una infraestructura que técnicamente es superior a muchas configuraciones empresariales pesadas.

- **Sintonía con Gemini (Yo):** Al usar tecnologías tan precisas (Rust, ArkType, Podman), puedo ayudarte a depurar problemas de infraestructura y código con muchísima más exactitud.


----

# ADR 0002: Arquitectura de Sintonía Hexagonal (3026)

**Estado:** 🟢 Activo

**Fecha:** 2026-03-03

---

## Contexto

Necesitamos un sistema de gran escala que soporte Usuarios, Roles y Permisos sin volverse un caos con el tiempo, optimizando recursos en un VPS de $5.

---

## 1. El Nombre: "Sintonía Hexagonal"

Elegimos este nombre porque:

- **Hexagonal (Puertos y Adaptadores):** Representa la libertad. El núcleo no está "atado" a nada externo.

- **Sintonía:** Es el concepto del Código 3026. Todas las capas (Backend, Frontend, DB, Infra) vibran en la misma frecuencia gracias a los contratos de datos (Protobuf).

---

## 2. Por qué elegimos esta mezcla (Hexagonal + Clean Code)

### A. Independencia de la Base de Datos (Clean Code)

En el desarrollo tradicional, la base de datos dicta el código. Aquí, el Dominio dicta las reglas.

**Beneficio:** Empezamos con SQLite para ahorrar costos y simplicidad. Si el proyecto escala a millones de usuarios, cambiamos a SurrealDB sustituyendo solo una carpeta (`infra_db`), sin tocar una sola línea de la lógica de usuarios o roles.

### B. El Núcleo Inmortal (Hexagonal)

El "Hexágono Interior" (`core_logic`) es puro Rust. No sabe qué es Axum, no sabe qué es la Web.

**Beneficio:** Si en el futuro queremos cambiar la API REST por una comunicación por satélite o una interfaz de comandos (CLI), el código de "Registrar Usuario con Rol Admin" sigue siendo exactamente el mismo.

### C. Testabilidad Total

Al usar Puertos (Traits/Interfaces), podemos probar la lógica de permisos sin necesidad de encender una base de datos real. Usamos "Mocks" (simuladores) que responden instantáneamente.

---

## 3. La Estructura de Poder (Roles y Permisos)

Para este laboratorio, la arquitectura se divide así:

| Capa | Responsabilidad en Roles/Permisos |
|------|-----------------------------------|
| **Domain (Capa 1)** | Define qué es un Permission (ej: `Write_Post`). Contiene la regla: "Un Usuario no puede tener más de 3 Roles". |
| **Application (Capa 2)** | El caso de uso `AssignRoleToUser`. Orquesta: busca al usuario, busca el rol, verifica permisos y ordena guardar. |
| **Infrastructure (Capa 3)** | El adaptador SQL que sabe cómo hacer un JOIN entre las tablas `users` y `roles` en SQLite. |
| **Entry Points (Capa 4)** | El controlador de Axum que recibe un JSON y lo pasa al caso de uso. |

---

## 4. El ADN del "Código 3026"

Hemos escogido esta arquitectura para maximizar tu Potencial Humano en el código:

### Menos Carpintería, Más Arquitectura
No pierdes tiempo peleando con frameworks. El 80% de tu esfuerzo está en el Dominio (donde vive la inteligencia).

### Sintonía de Tipado
Al usar Clean Code con Rust, el compilador es tu mentor. Si intentas asignar un permiso que no existe, el código no compila.

### Eficiencia de Costos
Al ser tan modular, el binario de Rust resultante es minúsculo (aprox. 15-20MB), ideal para que tu VPS de $5 respire tranquilo.

---

## 5. Consecuencias

### Positivas
Código que dura décadas. Facilidad para añadir funciones complejas (como auditoría de roles o Event Sourcing).

### Desafío
Requiere escribir más archivos al principio (Mappers, DTOs, Repositorios). Es una inversión de tiempo inicial para tener libertad eterna después.

Árbol de Proyecto:
<pre>
laboratorio_3026/
├── <b>proto/</b>                          # EL ADN (Contratos Binarios)
│   ├── auth.proto                  # Esquema: User, Role, Permission, AuthService
│   ├── common.proto                # Tipos compartidos (Timestamps, UUIDs)
│   ├── buf.yaml                    # Configuración de compilación Protobuf
│   └── buf.gen.yaml                # Plugins para generar Rust y TypeScript
├── <b>crates/</b>                         # EL MOTOR (Rust Workspace)
│   ├── <b>core_logic/</b>                 # CORAZÓN (Lógica Pura y Agnóstica)
│   │   ├── src/
│   │   │   ├── <b>domain/</b>             # Capa 1: Reglas de Oro (No dependencias)
│   │   │   │   ├── <b>entities/</b>       # Objetos con Identidad
│   │   │   │   │   ├── mod.rs
│   │   │   │   │   ├── user.rs      # Entidad User (Reglas de negocio)
│   │   │   │   │   ├── role.rs      # Entidad Role
│   │   │   │   │   └── permission.rs# Entidad Permission
│   │   │   │   ├── <b>value_objects/</b>  # Atributos Inmutables Validados
│   │   │   │   │   ├── mod.rs
│   │   │   │   │   ├── user_id.rs
│   │   │   │   │   ├── email.rs     # Autovalidación de formato
│   │   │   │   │   └── password.rs  # Reglas de complejidad
│   │   │   │   ├── <b>interfaces/</b>     # PUERTOS (Traits/Contratos)
│   │   │   │   │   ├── mod.rs
│   │   │   │   │   ├── user_repo.rs # IUserRepository
│   │   │   │   │   ├── role_repo.rs # IRoleRepository
│   │   │   │   │   └── hasher.rs    # IHasherService
│   │   │   │   ├── <b>events/</b>         # Hechos (UserCreated, RoleAssigned)
│   │   │   │   └── <b>errors.rs</b>       # DomainErrors (Unauthorized, NotFound)
│   │   │   ├── <b>application/</b>        # Capa 2: Orquestación (Casos de Uso)
│   │   │   │   ├── <b>use_cases/</b>      # Lógica de flujo
│   │   │   │   │   ├── mod.rs
│   │   │   │   │   ├── <b>user/</b>       # register.rs, login.rs, update_profile.rs
│   │   │   │   │   └── <b>role/</b>       # create_role.rs, assign_to_user.rs
│   │   │   │   └── <b>dtos/</b>           # Estructuras de paso (Request/Response)
│   │   │   └── lib.rs              # Re-exporta el dominio y aplicación
│   ├── <b>infra_db/</b>                   # INFRAESTRUCTURA (Adaptadores de Salida)
│   │   ├── src/
│   │   │   ├── <b>persistence/</b>        # Implementaciones de base de datos
│   │   │   │   ├── <b>sqlite/</b>         # Adaptador Actual (SQLx)
│   │   │   │   │   ├── mod.rs
│   │   │   │   │   ├── <b>repositories/</b> # sqlite_user_repo.rs, sqlite_role_repo.rs
│   │   │   │   │   ├── <b>models/</b>       # Schemas de tablas SQL (DbUser)
│   │   │   │   │   └── migrations/   # Archivos .sql de creación
│   │   │   │   └── surreal/        # Adaptador Futuro (Placeholder)
│   │   │   ├── <b>external_services/</b>  # BcryptHasher, ResendEmail, S3Storage
│   │   │   ├── <b>mappers/</b>            # Traductores: DB <-> Dominio <-> Proto
│   │   │   ├── shared/             # Utilidades técnicas (Pagination, Logger)
│   │   │   └── lib.rs              # Exporta los adaptadores
│   └── <b>api_server/</b>                 # ENTRADA Y PEGAMENTO (Adaptadores Primarios)
│       ├── src/
│       │   ├── <b>config/</b>             # COMPOSITION ROOT (El Cerebro)
│       │   │   ├── di.rs           # Inyección de Dependencias (Wiring)
│       │   │   ├── env.rs          # Validación estricta de .env (Envs struct)
│       │   │   └── server.rs       # Configuración Axum, CORS, Timeouts
│       │   ├── <b>entry_points/</b>       # Los "Enchufes" de entrada
│       │   │   ├── <b>api/v1/</b>         # Controladores REST / gRPC-Web
│       │   │   │   ├── mod.rs      # Versionamiento
│       │   │   │   ├── auth_handlers.rs
│       │   │   │   └── user_handlers.rs
│       │   │   ├── <b>middlewares/</b>    # AuthGuard, ArkType/Zod-like Validator
│       │   │   ├── <b>cli/</b>            # Terminal: seeders, create_admin, migrate
│       │   │   └── <b>queue/</b>          # Consumidores (Redis/RabbitMQ)
│       │   ├── main.rs             # PUNTO CERO (Bootstrapping)
│       │   ├── routes.rs           # Definición del Router central
│       │   └── lib.rs
├── <b>test/</b>                           # Tests de Integración y E2E
├── <b>apps/</b>                           # INTERFACES DE USUARIO
│   ├── <b>frontend_astro/</b>             # Web (Astro + HTMX + ArkType)
│   │   ├── src/components/
│   │   └── src/pages/
│   └── <b>desktop_tauri/</b>              # App Nativa (Rust + WebView)
├── <b>deploy/</b>                         # DESPLIEGUE SOBERANO
│   ├── podman-compose.yml          # Orquestación Rootless
│   ├── Dockerfile                  # Multi-stage build para el binario Rust
│   └── Caddyfile                   # Reverse Proxy y SSL Automático
├── <b>Cargo.toml</b>                      # Manifiesto del Workspace (Dependencies)
├── <b>justfile</b>                        # Centro de Mando (Comandos rápidos)
├── <b>.gitignore</b>                      # Filtro de limpieza
├── .env                            # Secretos locales (No subir al git)
└── .env.example                    # Plantilla de secretos
</pre>

# 🛠️ Paso a Paso: El Camino del Programador 3026

No intentes programar todo a la vez. Sigue este orden para mantener la Sintonía:

---

## Paso 1: Definir el ADN (`/proto`)

Antes de Rust o TS, escribe el archivo `.proto`.

**Por qué:** Aquí decides cómo se llama un usuario y qué permisos existen. Al compilarlo, tendrás tipos de datos idénticos en el Backend y el Frontend.

---

## Paso 2: El Corazón del Dominio (`crates/core_logic/domain`)

Escribe tus `entities.rs` y `value_objects.rs`.

**Acción:** Crea la lógica de validación. Por ejemplo: un Role no puede estar vacío, un Email debe tener una @.

**Nota:** Aquí no pienses en bases de datos, solo en "objetos reales".

---

## Paso 3: Los Casos de Uso (`crates/core_logic/application`)

Escribe `register_user.rs`.

**Acción:** Define los pasos:
1. Validar datos →
2. Verificar si existe en el Repo →
3. Encriptar password →
4. Guardar.

---

## Paso 4: La Memoria (`crates/infra_db`)

Implementa el repositorio con SQLx.

**Acción:** Escribe el SQL para insertar usuarios y roles. Usa los `mappers.rs` para asegurarte que lo que sale de la DB se convierta en una Entidad de tu Dominio.

---

## Paso 5: La Puerta de Entrada (`crates/api_server`)

Configura Axum.

**Acción:** Crea un endpoint `POST /register`. En el `main.rs`, inyecta la base de datos dentro de tu caso de uso. Aquí es donde ocurre la magia de la unión.

---

## Paso 6: La Interfaz (`apps/frontend_astro`)

Usa ArkType para validar los formularios en el navegador y envía la petición a Axum.

**Acción:** Si el usuario tiene el rol "Admin", HTMX le mostrará el botón de "Gestionar Permisos".

---

## Paso 7: El Despliegue (`deploy/`)

Levanta todo con `podman-compose up`.

**Acción:** Tu VPS de $5 ahora corre un sistema de nivel bancario, protegido por Caddy con HTTPS automático.

# 🎓 Explicación Detallada para aprender el "Código 3026"

Para que el laboratorio funcione, hemos dividido la inteligencia en capas. Aquí te explico qué hace cada una y por qué la configuramos así:

------------------

## 1. El ADN (`/proto`)

Es el **Contrato Social**. Antes de escribir código, definimos cómo se comunican las partes. Al usar Protobuf, si cambias un campo en el usuario, el compilador te avisará tanto en Rust como en TypeScript.

&gt; **Sin ADN, no hay sintonía.**

---

## 2. El Dominio (`/domain`)

Es la **Capa 1 (Pura)**. Aquí vive el "Potencial Humano" del código.

- **Value Objects:** No permitas que un "Email" sea un simple texto. Crea un objeto que se valide a sí mismo. Si el código tiene un Email, es porque es real.

- **Interfaces:** Son los "Enchufes". El dominio dice: "Necesito guardar un usuario, no me importa si es en un archivo de texto o en la NASA".

---

## 3. La Infraestructura (`/infra_db`)

Es el **Cuerpo Técnico**. Aquí es donde vive SQLite.

- **Mappers:** Son vitales. La base de datos guarda datos "sucios" o planos. El Mapper los transforma en "Objetos de Dominio" con toda su lógica. Es el traductor de la realidad técnica a la lógica pura.

---

## 4. El Composition Root (`/api_server/config`)

Es la **Magia de la Unión**. Es donde ocurre la Inyección de Dependencias.

En `di.rs`, tú le dices al sistema: "Para este laboratorio, usa SQLite y Bcrypt".

Si mañana quieres usar SurrealDB, solo cambias una línea en este archivo. El resto de las 10,000 líneas de código del proyecto no cambian.

&gt; **Eso es libertad.**

---

## 5. Entry Points (`/entry_points`)

Son las **Antenas**. Tu sistema puede ser escuchado por:
- Una API Web (Axum)
- Una terminal (CLI)
- Un robot (Queues)

Todos llaman a los mismos "Casos de Uso".

---

# 🛠️ ¿Cómo se siente trabajar aquí?

1. Defines el ADN en `proto`.
2. Creas la regla en el `domain`.
3. Implementas el almacenamiento en `infra_db` (SQLite).
4. Conectas todo en `config/di.rs`.
5. Expones el resultado en `api/v1`.

Este sistema es **indestructible en un VPS de $5** porque Rust no gasta memoria en cosas innecesarias y SQLite WAL permite miles de escrituras por segundo sin despeinarse.
