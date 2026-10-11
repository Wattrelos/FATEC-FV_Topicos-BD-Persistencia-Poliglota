use crate::config::AppConfig;
use anyhow::{Context, Result};
use mongodb::{options::ClientOptions, Client, Database};
use sqlx::mysql::MySqlPoolOptions;
use sqlx::MySqlPool;

/// Cria um pool de conexões assíncrono para o MySQL
pub async fn create_mysql_pool(config: &AppConfig) -> Result<MySqlPool> {
    let pool = MySqlPoolOptions::new()
        .max_connections(10)
        .connect(&config.database_url)
        .await
        .context("Falha ao conectar no MySQL através da DATABASE_URL")?;
    Ok(pool)
}

/// Cria a conexão com o MongoDB e seleciona o banco do catálogo
pub async fn create_mongo_db(config: &AppConfig) -> Result<Database> {
    let client_options = ClientOptions::parse(&config.mongodb_uri)
        .await
        .context("Falha ao analisar MONGODB_URI")?;

    let client = Client::with_options(client_options)
        .context("Falha ao inicializar o cliente MongoDB")?;

    let db = client.database(&config.mongodb_database);
    Ok(db)
}
