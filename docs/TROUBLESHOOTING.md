# 🩺 PROTOCOLO DE SINTONÍA: TROUBLESHOOTING 3026

**Estado:** 🟢 Activo

---

## Visión

Este documento es la bitácora de combate del Laboratorio 3026. No documenta el "cómo funciona" (para eso está `docs-fases/`), sino el **"qué hacer cuando no funciona"**. Cada error superado es una lección aprendida y una vacuna para el futuro.

---

### 1. Error de Compilación: `file not found for module '...'`

- **Síntoma:** `cargo check` o `clippy` fallan con un error indicando que no pueden encontrar un módulo que has declarado con `mod my_module;`.
  ```text
  error[E0583]: file not found for module `persistence`
   --> crates\infra_db\src\lib.rs:2:1
    |
  2 | pub mod persistence;
    | ^^^^^^^^^^^^^^^^^^^^
  ```

- **Diagnóstico:** En Rust, la declaración de un módulo (`mod module_name;`) le dice al compilador que busque un archivo `module_name.rs` o una carpeta `module_name/mod.rs` en el mismo directorio. Este error significa que esa estructura de archivos no existe.

- **Cura (Sintonía 3026):**
  1.  Si declaras `pub mod persistence;` en `lib.rs`, debes crear un archivo `persistence.rs` o, para una estructura más compleja, una carpeta `persistence/` con un archivo `mod.rs` dentro.
  2.  **Ejemplo:** Para nuestra estructura `infra_db/src/persistence/sqlite/repositories/`, necesitamos una cadena de archivos `mod.rs` que conecten todo:
      - `infra_db/src/lib.rs` contiene `pub mod persistence;`
      - `infra_db/src/persistence/mod.rs` contiene `pub mod sqlite;`
      - `infra_db/src/persistence/sqlite/mod.rs` contiene `pub mod repositories;`
      - `infra_db/src/persistence/sqlite/repositories/mod.rs` contiene `pub mod sqlite_user_repo;`

---

### 2. Error de SQLx: `set 'DATABASE_URL' to use query macros`

- **Síntoma:** Al compilar, `sqlx` falla en las macros `query!` o `query_as!` porque no puede conectarse a la base de datos para verificar las consultas.

- **Diagnóstico:** `sqlx` necesita validar las consultas SQL en tiempo de compilación para garantizar la seguridad de tipos. Para ello, debe poder acceder a la base de datos o a un caché de metadatos.

- **Cura (Sintonía 3026):**
  1.  **Asegurar la DB:** Ejecuta `just db-migrate`. Esto crea el archivo `backend.db` y las tablas necesarias.
  2.  **Generar el Caché Offline:** Ejecuta el comando que hemos creado en nuestro `Justfile`. Esto genera un archivo `sqlx-data.json` en la raíz del crate, que `sqlx` usará para la compilación si no puede acceder a la DB.
      ```bash
      just db-prepare
      ```
  3.  Este enfoque es vital para entornos de CI/CD donde la base de datos no está disponible durante la compilación.

---

### 3. Error de Compilación: `temporary value dropped while borrowed`

- **Síntoma:** El compilador se queja de que un valor temporal se elimina mientras todavía está siendo "prestado".
  ```text
  error[E0716]: temporary value dropped while borrowed
  --> crates\infra_db\src\persistence\sqlite\repositories\sqlite_user_repo.rs:94:13
     |
  94 |             email.as_str()
     |             ^^^^^^^^^^^^^^ creates a temporary value which is freed while still in use
  ```

- **Diagnóstico:** Ocurre cuando pasas el resultado de una función (ej: `email.as_str()`) directamente a otra función o macro que necesita una referencia que viva más tiempo. El `&str` devuelto por `as_str()` es temporal y Rust lo elimina inmediatamente, pero la macro `sqlx` y la llamada `.await` posterior todavía lo necesitan.

- **Cura (Sintonía 3026):**
  - Almacena el valor temporal en una variable con `let` antes de usarlo. Esto extiende su ciclo de vida para toda la función.
    ```rust
    // INCORRECTO
    let db_user = sqlx::query_as!(..., email.as_str()).await?;

    // CORRECTO (Sintonía 3026)
    let email_str = email.as_str();
    let db_user = sqlx::query_as!(..., email_str).await?;
    ```

---

### 4. Error de SQLx: `the trait bound ... is not satisfied` (Conversión de Tipos)

- **Síntoma:** `sqlx::query_as!` falla porque no puede convertir un tipo de dato de la base de datos (ej: `Vec<u8>` para `BLOB`) a un tipo de dato del dominio (ej: `uuid::Uuid`).

- **Diagnóstico:** La macro `query_as!` intenta hacer una conversión `From`/`Into` directa. Si no existe una implementación de ese trait entre el tipo de la DB y el tipo de tu struct, la compilación falla.

- **Cura (Sintonía 3026):**
  1.  **Acepta el Tipo Crudo:** Modifica tu struct DTO (ej: `DbUser`) para que sus campos coincidan con los tipos que `sqlx` lee de la base de datos (`Vec<u8>` para `BLOB`, `NaiveDateTime` para `DATETIME`).
  2.  **Mapea Explícitamente:** En tu función "mapper" (ej: `to_domain_user`), realiza la conversión manual y controlada, manejando los posibles errores.
    ```rust
    // En el mapper
    let uuid = Uuid::from_slice(&db_user.id)
        .map_err(|e| anyhow::anyhow!("Error al parsear UUID: {}", e))?;

    let created_at = DateTime::<Utc>::from_naive_utc_and_offset(db_user.created_at, Utc);
    ```
  - Esto refuerza la Arquitectura Hexagonal: el adaptador es el único responsable de conocer y traducir los detalles "sucios" de la infraestructura.