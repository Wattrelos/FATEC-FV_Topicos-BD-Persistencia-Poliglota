use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Documento principal na collection `catalog_products` (MongoDB Read Model)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CatalogProductDocument {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub product_id: i64,
    pub name: String,
    pub model: String,
    pub slug: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<CategoryInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manufacturer: Option<ManufacturerInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub short_description: Option<String>,
    pub status: String,

    /// Eixos de variação para renderizar seletores/botões na interface (ex: Cor, Tamanho, Voltagem)
    #[serde(default)]
    pub variation_axes: Vec<VariationAxis>,

    /// Lista completa de SKUs concretos da variação
    #[serde(default)]
    pub variants: Vec<CatalogVariant>,

    /// Facetas pré-computadas para acelerar queries e filtros de catálogo
    #[serde(default)]
    pub facets: HashMap<String, Vec<String>>,
    pub min_price: f64,
    pub max_price: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CategoryInfo {
    pub id: i64,
    pub name: String,
    pub path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManufacturerInfo {
    pub id: i64,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VariationAxis {
    pub code: String,
    pub label: String,
    pub options: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CatalogVariant {
    pub variant_id: i64,
    pub sku: String,
    pub ean: Option<String>,
    /// Mapa de atributos (ex: {"volume": "18L", "acabamento": "Fosco", "cor": "Branco Neve"})
    pub attributes: HashMap<String, String>,
    pub price: f64,
    pub in_stock: bool,
    pub weight_kg: f64,
    pub dimensions: Dimensions,
    pub image_url: Option<String>,
    pub status: i8,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Dimensions {
    pub length: f64,
    pub width: f64,
    pub height: f64,
}
