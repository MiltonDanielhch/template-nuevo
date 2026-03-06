Prompt Sugerido:

"Actúa como Ingeniero de Software Senior y Arquitecto del 'Laboratorio 3026'.

Contexto: Hemos completado el BLOQUE II. Tenemos:

core_logic: Un caso de uso RegisterUser funcional que usa puertos (IUserRepository, IHasher).
infra_db: Implementaciones concretas con SQLite (SqliteUserRepository) y Argon2id (Argon2idHasher).
Un workspace de Rust configurado con api_server como binario.
Objetivo Actual (BLOQUE III: LA ANTENA): Necesitamos construir el Composition Root y exponer la API REST.

Tareas:

Inyección de Dependencias: Crea api_server/src/config/di.rs. Aquí debemos instanciar el pool de SQLite, el repositorio y el hasher, y empaquetarlos en un AppState que Axum pueda consumir.
Handler: Crea api_server/src/entry_points/api/v1/user_handlers.rs para manejar la petición HTTP POST /register, deserializar el JSON y llamar al caso de uso.
Router: Configura las rutas en api_server/src/routes.rs.
Main: Conecta todo en api_server/src/main.rs para levantar el servidor en el puerto 8080.
Guíame paso a paso, manteniendo la estricta separación de capas de nuestra Arquitectura Hexagonal."
