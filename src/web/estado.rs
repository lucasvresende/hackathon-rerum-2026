use sqlx::SqlitePool;
use std::sync::Arc;

#[derive(Clone)]
pub struct EstadoApp {
    pub pool: SqlitePool,
    pub csrf: Arc<String>,
    pub origem: Arc<String>,
}

impl EstadoApp {
    pub fn novo(pool: SqlitePool, origem: String) -> Self {
        Self {
            pool,
            csrf: Arc::new(uuid::Uuid::new_v4().to_string()),
            origem: Arc::new(origem),
        }
    }
}
