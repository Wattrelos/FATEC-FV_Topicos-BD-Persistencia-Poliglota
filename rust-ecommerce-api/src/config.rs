use anyhow::Result;
use std::env;

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub database_url: String,
    pub mongodb_uri: String,
    pub mongodb_database: String,
    pub server_host: String,
    pub server_port: u16,
}

impl AppConfig {
    /// Carrega as variáveis a partir do arquivo .env ou do ambiente do sistema
    pub fn from_env() -> Result<Self> {
        // Tenta carregar o .env do diretório atual ou de diretórios pais
        let _ = dotenvy::dotenv();

        let database_url = env::var("DATABASE_URL").unwrap_or_else(|_| {
            let user = env::var("DB_USERNAME").unwrap_or_else(|_| "root".to_string());
            let pass = env::var("DB_PASSWORD").unwrap_or_default();
            let host = env::var("DB_HOSTNAME").unwrap_or_else(|_| "127.0.0.1".to_string());
            let port = env::var("DB_PORT").unwrap_or_else(|_| "3306".to_string());
            let db = env::var("DB_DATABASE").unwrap_or_else(|_| "agsc_product_variation".to_string());

            if pass.is_empty() {
                format!("mysql://{user}@{host}:{port}/{db}")
            } else {
                format!("mysql://{user}:{pass}@{host}:{port}/{db}")
            }
        });

        let mongodb_uri = env::var("MONGODB_URI").unwrap_or_else(|_| {
            let host = env::var("MONGODB_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
            let port = env::var("MONGODB_PORT").unwrap_or_else(|_| "27017".to_string());
            format!("mongodb://{host}:{port}")
        });

        let mongodb_database = env::var("MONGODB_DATABASE")
            .or_else(|_| env::var("MONGODB_NAME"))
            .unwrap_or_else(|_| "catalog_products".to_string());

        let server_host = env::var("SERVER_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
        let server_port = env::var("SERVER_PORT")
            .unwrap_or_else(|_| "3000".to_string())
            .parse::<u16>()
            .unwrap_or(3000);

        Ok(Self {
            database_url,
            mongodb_uri,
            mongodb_database,
            server_host,
            server_port,
        })
    }
}
