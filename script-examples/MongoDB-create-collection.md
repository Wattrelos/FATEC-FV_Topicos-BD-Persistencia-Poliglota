# Exemplo de inserção de documento na coleção catalog_products

```bash
use catalog_products
```

# Documento de exemplo
```bash
db.catalog_products.insertOne({
  _id: ObjectId("67098f9c1e7a4b810d9821a1"),
  product_id: 1042,
  name: "Tinta Acrílica Premium Rende Muito",
  model: "SUV-RENDE-MTO",
  slug: "tinta-acrilica-premium-rende-muito",
  category: {
    id: 15,
    name: "Pintura e Acessórios",
    path: "Tintas > Tintas Imobiliárias > Acrílicas"
  },
  manufacturer: {
    id: 8,
    name: "Suvinil"
  },
  short_description: "Tinta acrílica lavável de alto rendimento e cobertura.",
  status: "ACTIVE",
  variation_axes: [
    { code: "volume", label: "Volume da Embalagem", options: ["900ml", "3.6L", "18L"] },
    { code: "acabamento", label: "Acabamento da Superfície", options: ["Fosco", "Acetinado"] },
    { code: "cor", label: "Tonalidade", options: ["Branco Neve", "Palha", "Cinza Urbano"] }
  ],
  variants: [
    {
      variant_id: 5001,
      sku: "SUV-RM-18L-FOS-BR",
      ean: "7891234567890",
      attributes: { volume: "18L", acabamento: "Fosco", cor: "Branco Neve" },
      price: 389.90,
      in_stock: true,
      weight_kg: 24.50,
      dimensions: { length: 32.0, width: 32.0, height: 38.0 },
      image_url: "/images/products/suv-18l-branco.webp",
      status: 1
    },
    {
      variant_id: 5002,
      sku: "SUV-RM-3.6L-FOS-BR",
      ean: "7891234567891",
      attributes: { volume: "3.6L", acabamento: "Fosco", cor: "Branco Neve" },
      price: 119.50,
      in_stock: true,
      weight_kg: 5.20,
      dimensions: { length: 18.0, width: 18.0, height: 22.0 },
      image_url: "/images/products/suv-36l-branco.webp",
      status: 1
    }
  ],
  facets: {
    volume: ["18L", "3.6L"],
    acabamento: ["Fosco"],
    cor: ["Branco Neve"],
    min_price: 119.50,
    max_price: 389.90
  },
  updated_at: ISODate("2026-10-11T18:20:00Z")
});
```