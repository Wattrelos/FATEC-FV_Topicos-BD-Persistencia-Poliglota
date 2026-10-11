mod config;
mod db;
mod models;
mod services;

use anyhow::Result;
use config::AppConfig;
use db::{create_mongo_db, create_mysql_pool};
use models::{
    CatalogProductDocument, ProductDescriptionRow, ProductMasterRow, ProductVariantRow,
    VariantAttributeRow,
};
use mongodb::bson::doc;
use rust_decimal_macros::dec;
use services::build_catalog_projection;
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<()> {
    // 1. Inicializa o subsistema de logs/tracing
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .init();

    println!("================================================================================");
    println!("🦀 Rust E-Commerce: Demonstração de Persistência Poliglota (MySQL + MongoDB)");
    println!("================================================================================");

    // 2. Carrega configurações do .env
    let config = AppConfig::from_env()?;
    println!("⚙️  Configuração carregada com sucesso:");
    println!("   - MySQL Database URL: {}", mask_url(&config.database_url));
    println!("   - MongoDB URI:        {}", config.mongodb_uri);
    println!("   - MongoDB Database:   {}", config.mongodb_database);
    println!("   - Server API Bind:    {}:{}", config.server_host, config.server_port);

    // 3. Conexão com o MySQL (SQLx Pool)
    println!("\n🔌 Conectando ao MySQL...");
    let mysql_pool = create_mysql_pool(&config).await?;
    let (db_name, db_version): (String, String) =
        sqlx::query_as("SELECT DATABASE(), VERSION()")
            .fetch_one(&mysql_pool)
            .await?;
    println!("✅ MySQL conectado! [Banco: '{}', Versão: '{}']", db_name, db_version);

    // 4. Conexão com o MongoDB
    println!("\n🔌 Conectando ao MongoDB...");
    let mongo_db = create_mongo_db(&config).await?;
    mongo_db.run_command(doc! { "ping": 1 }).await?;
    println!("✅ MongoDB conectado com sucesso! [Database: '{}']", config.mongodb_database);

    // 5. Demonstração Prática: Inserindo e Consultando no MySQL
    println!("\n📦 [Passo 1] Gravando Produto e Variantes no MySQL (Write Model)...");
    let model_code = "SUV-RENDE-MTO";

    // Verifica ou insere o produto mestre
    let existing_product: Option<ProductMasterRow> = sqlx::query_as(
        "SELECT product_id, model, manufacturer_id, shipping, status, sort_order, date_added, date_modified 
         FROM agsc_product WHERE model = ?"
    )
    .bind(model_code)
    .fetch_optional(&mysql_pool)
    .await?;

    let product_id = match existing_product {
        Some(p) => {
            println!("   - Produto mestre já existente com ID: {}", p.product_id);
            p.product_id
        }
        None => {
            let res = sqlx::query(
                "INSERT INTO agsc_product (model, manufacturer_id, shipping, status, sort_order) 
                 VALUES (?, 8, 1, 1, 0)"
            )
            .bind(model_code)
            .execute(&mysql_pool)
            .await?;
            let id = res.last_insert_id() as i64;
            println!("   - Novo produto mestre inserido com ID: {}", id);

            // Insere descrição
            sqlx::query(
                "INSERT INTO agsc_product_description 
                 (product_id, language_id, name, description, meta_title, meta_description) 
                 VALUES (?, 1, 'Tinta Acrílica Premium Rende Muito', 'Tinta lavável fosca', 'Tinta Rende Muito', 'Tinta acrílica lavável de alto rendimento')"
            )
            .bind(id)
            .execute(&mysql_pool)
            .await?;

            // Insere Variantes de Exemplo
            insert_variant_with_attrs(
                &mysql_pool,
                id,
                "SUV-RM-18L-FOS-BR",
                Some("7891234567890"),
                dec!(389.90),
                25,
                dec!(24.5),
                &[("volume", "18L"), ("acabamento", "Fosco"), ("cor", "Branco Neve")],
            )
            .await?;

            insert_variant_with_attrs(
                &mysql_pool,
                id,
                "SUV-RM-3.6L-FOS-BR",
                Some("7891234567891"),
                dec!(119.50),
                40,
                dec!(5.2),
                &[("volume", "3.6L"), ("acabamento", "Fosco"), ("cor", "Branco Neve")],
            )
            .await?;

            insert_variant_with_attrs(
                &mysql_pool,
                id,
                "SUV-RM-900ML-FOS-BR",
                Some("7891234567892"),
                dec!(45.90),
                15,
                dec!(1.3),
                &[("volume", "900ml"), ("acabamento", "Fosco"), ("cor", "Branco Neve")],
            )
            .await?;

            id
        }
    };

    // 6. Lê os dados normalizados do MySQL
    println!("\n🔍 [Passo 2] Lendo dados transacionais do MySQL para projeção...");
    let master: ProductMasterRow = sqlx::query_as(
        "SELECT product_id, model, manufacturer_id, shipping, status, sort_order, date_added, date_modified 
         FROM agsc_product WHERE product_id = ?"
    )
    .bind(product_id)
    .fetch_one(&mysql_pool)
    .await?;

    let description: Option<ProductDescriptionRow> = sqlx::query_as(
        "SELECT product_id, language_id, name, description, tag, meta_title, meta_description, meta_keyword 
         FROM agsc_product_description WHERE product_id = ? AND language_id = 1"
    )
    .bind(product_id)
    .fetch_optional(&mysql_pool)
    .await?;

    let variants: Vec<ProductVariantRow> = sqlx::query_as(
        "SELECT variant_id, product_id, sku, ean, price, quantity, weight, weight_class_id, 
                length, width, height, length_class_id, ncm, cest, status, date_added, date_modified 
         FROM agsc_product_variant WHERE product_id = ?"
    )
    .bind(product_id)
    .fetch_all(&mysql_pool)
    .await?;

    // Busca atributos de cada variante
    let mut attributes_by_variant = HashMap::new();
    for v in &variants {
        let attrs: Vec<VariantAttributeRow> = sqlx::query_as(
            "SELECT id, variant_id, attribute_code, attribute_value 
             FROM agsc_product_variant_attribute WHERE variant_id = ?"
        )
        .bind(v.variant_id)
        .fetch_all(&mysql_pool)
        .await?;
        attributes_by_variant.insert(v.variant_id, attrs);
    }

    println!("   - Encontradas {} variantes físicas cadastradas no MySQL.", variants.len());

    // 7. Executa a projeção desnormalizada em Rust
    println!("\n🔄 [Passo 3] Executando Projeção Desnormalizada em Rust...");
    let projection = build_catalog_projection(
        &master,
        description.as_ref(),
        &variants,
        &attributes_by_variant,
    );

    println!("   - Produto projetado: '{}'", projection.name);
    println!("   - Faixa de Preços calculada: R$ {:.2} até R$ {:.2}", projection.min_price, projection.max_price);
    println!("   - Eixos de variação identificados: {:?}", projection.variation_axes.iter().map(|a| &a.code).collect::<Vec<_>>());

    // 8. Salva/Atualiza o documento no MongoDB (Read Model)
    println!("\n🍃 [Passo 4] Persistindo Projeção na Coleção `catalog_products` (MongoDB)...");
    let collection = mongo_db.collection::<CatalogProductDocument>("catalog_products");

    let filter = doc! { "product_id": projection.product_id };
    let replace_options = mongodb::options::ReplaceOptions::builder()
        .upsert(true)
        .build();

    collection
        .replace_one(filter, projection.clone())
        .with_options(replace_options)
        .await?;

    println!("✅ Documento sincronizado com sucesso no MongoDB!");

    // 9. Consulta e exibe o documento diretamente do MongoDB
    println!("\n📄 [Passo 5] Consultando MongoDB para simular renderização de PDP (Product Detail Page):");
    let found = collection
        .find_one(doc! { "product_id": product_id })
        .await?;

    if let Some(doc_mongo) = found {
        println!("--------------------------------------------------------------------------------");
        println!("{}", serde_json::to_string_pretty(&doc_mongo)?);
        println!("--------------------------------------------------------------------------------");
    }

    println!("\n🎉 Persistência Poliglota Operacional! MySQL e MongoDB sincronizados com sucesso.");
    Ok(())
}

async fn insert_variant_with_attrs(
    pool: &sqlx::MySqlPool,
    product_id: i64,
    sku: &str,
    ean: Option<&str>,
    price: rust_decimal::Decimal,
    quantity: i32,
    weight: rust_decimal::Decimal,
    attributes: &[(&str, &str)],
) -> Result<i64> {
    let res = sqlx::query(
        "INSERT INTO agsc_product_variant 
         (product_id, sku, ean, price, quantity, weight, weight_class_id, length, width, height, length_class_id, ncm, cest, status) 
         VALUES (?, ?, ?, ?, ?, ?, 1, 0, 0, 0, 1, '32091010', '0100100', 1)"
    )
    .bind(product_id)
    .bind(sku)
    .bind(ean)
    .bind(price)
    .bind(quantity)
    .bind(weight)
    .execute(pool)
    .await?;

    let variant_id = res.last_insert_id() as i64;

    for (code, value) in attributes {
        sqlx::query(
            "INSERT INTO agsc_product_variant_attribute (variant_id, attribute_code, attribute_value) 
             VALUES (?, ?, ?)"
        )
        .bind(variant_id)
        .bind(code)
        .bind(value)
        .execute(pool)
        .await?;
    }

    Ok(variant_id)
}

fn mask_url(url: &str) -> String {
    if let Some(at_idx) = url.find('@') {
        if let Some(colon_slash) = url.find("://") {
            let scheme = &url[..colon_slash + 3];
            let host_part = &url[at_idx..];
            return format!("{scheme}****:****{host_part}");
        }
    }
    url.to_string()
}
