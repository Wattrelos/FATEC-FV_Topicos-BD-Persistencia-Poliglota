use chrono::NaiveDateTime;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

/// Representa a tabela `agsc_product` (Produto Pai / Agrupador)
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ProductMasterRow {
    pub product_id: i64,
    pub model: String,
    pub manufacturer_id: Option<i64>,
    pub shipping: i8,
    pub status: i8,
    pub sort_order: i32,
    pub date_added: Option<NaiveDateTime>,
    pub date_modified: Option<NaiveDateTime>,
}

/// Representa a tabela `agsc_product_description` (Descrições e Metadados SEO)
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ProductDescriptionRow {
    pub product_id: i64,
    pub language_id: i32,
    pub name: String,
    pub description: Option<String>,
    pub tag: Option<String>,
    pub meta_title: Option<String>,
    pub meta_description: Option<String>,
    pub meta_keyword: Option<String>,
}

/// Representa a tabela `agsc_product_variant` (SKU Filho Transacionável)
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ProductVariantRow {
    pub variant_id: i64,
    pub product_id: i64,
    pub sku: String,
    pub ean: Option<String>,
    pub price: Decimal,
    pub quantity: i32,
    pub weight: Decimal,
    pub weight_class_id: i32,
    pub length: Decimal,
    pub width: Decimal,
    pub height: Decimal,
    pub length_class_id: i32,
    pub ncm: Option<String>,
    pub cest: Option<String>,
    pub status: i8,
    pub date_added: Option<NaiveDateTime>,
    pub date_modified: Option<NaiveDateTime>,
}

/// Representa a tabela `agsc_product_variant_attribute` (Atributos Determinantes da Variante)
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct VariantAttributeRow {
    pub id: i64,
    pub variant_id: i64,
    pub attribute_code: String,
    pub attribute_value: String,
}

/// Representa a tabela `agsc_outbox_event` (Garantia de Entrega no MongoDB via Outbox Pattern)
#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct OutboxEventRow {
    pub id: i64,
    pub aggregate_type: String,
    pub aggregate_id: i64,
    pub event_type: String,
    pub payload: serde_json::Value,
    pub status: String,
    pub retry_count: i32,
    pub created_at: Option<NaiveDateTime>,
    pub processed_at: Option<NaiveDateTime>,
}
