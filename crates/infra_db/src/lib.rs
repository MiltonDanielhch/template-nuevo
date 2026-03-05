// infra_db/src/lib.rs
pub mod persistence;
 
// Exponemos públicamente el repositorio concreto para que el 'Composition Root' (en api_server) pueda instanciarlo.
pub use persistence::sqlite::repositories::sqlite_user_repo::SqliteUserRepository;