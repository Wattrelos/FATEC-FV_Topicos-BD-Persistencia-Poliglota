use crate::models::{
    CatalogProductDocument, CatalogVariant, Dimensions, ProductDescriptionRow,
    ProductMasterRow, ProductVariantRow, VariantAttributeRow, VariationAxis,
};
use std::collections::{BTreeMap, HashMap, HashSet};

/// Constrói a projeção desnormalizada do catálogo (MongoDB) a partir das entidades relacionais (MySQL).
/// Essa função é o núcleo do Outbox Worker no padrão CQRS.
pub fn build_catalog_projection(
    master: &ProductMasterRow,
    description: Option<&ProductDescriptionRow>,
    variants: &[ProductVariantRow],
    attributes_by_variant: &HashMap<i64, Vec<VariantAttributeRow>>,
) -> CatalogProductDocument {
    let mut catalog_variants = Vec::new();
    let mut facets_map: BTreeMap<String, HashSet<String>> = BTreeMap::new();
    let mut min_price = f64::MAX;
    let mut max_price = 0.0;

    for v in variants {
        let price_f64 = v.price.to_string().parse::<f64>().unwrap_or(0.0);
        if price_f64 < min_price {
            min_price = price_f64;
        }
        if price_f64 > max_price {
            max_price = price_f64;
        }

        let mut attrs_map = HashMap::new();
        if let Some(attrs) = attributes_by_variant.get(&v.variant_id) {
            for attr in attrs {
                attrs_map.insert(attr.attribute_code.clone(), attr.attribute_value.clone());

                // Alimenta o mapa de facetas
                facets_map
                    .entry(attr.attribute_code.clone())
                    .or_default()
                    .insert(attr.attribute_value.clone());
            }
        }

        catalog_variants.push(CatalogVariant {
            variant_id: v.variant_id,
            sku: v.sku.clone(),
            ean: v.ean.clone(),
            attributes: attrs_map,
            price: price_f64,
            in_stock: v.quantity > 0,
            weight_kg: v.weight.to_string().parse::<f64>().unwrap_or(0.0),
            dimensions: Dimensions {
                length: v.length.to_string().parse::<f64>().unwrap_or(0.0),
                width: v.width.to_string().parse::<f64>().unwrap_or(0.0),
                height: v.height.to_string().parse::<f64>().unwrap_or(0.0),
            },
            image_url: None,
            status: v.status,
        });
    }

    // Gera as facetas ordenadas e os eixos de variação para os seletores de UI
    let mut facets: HashMap<String, Vec<String>> = HashMap::new();
    let mut variation_axes = Vec::new();

    for (code, values_set) in facets_map {
        let mut sorted_values: Vec<String> = values_set.into_iter().collect();
        sorted_values.sort();

        variation_axes.push(VariationAxis {
            code: code.clone(),
            label: capitalize_label(&code),
            options: sorted_values.clone(),
        });

        facets.insert(code, sorted_values);
    }

    let name = description
        .map(|d| d.name.clone())
        .unwrap_or_else(|| format!("Produto {}", master.model));

    let slug = generate_slug(&name);
    let short_desc = description.and_then(|d| d.meta_description.clone());

    CatalogProductDocument {
        id: None,
        product_id: master.product_id,
        name,
        model: master.model.clone(),
        slug,
        category: None,
        manufacturer: None,
        short_description: short_desc,
        status: if master.status == 1 {
            "ACTIVE".to_string()
        } else {
            "INACTIVE".to_string()
        },
        variation_axes,
        variants: catalog_variants,
        facets,
        min_price: if min_price == f64::MAX { 0.0 } else { min_price },
        max_price,
    }
}

fn capitalize_label(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
    }
}

fn generate_slug(text: &str) -> String {
    text.to_lowercase()
        .chars()
        .map(|c| if c.is_alphanumeric() { c } else { '-' })
        .collect::<String>()
        .split('-')
        .filter(|part| !part.is_empty())
        .collect::<Vec<_>>()
        .join("-")
}
