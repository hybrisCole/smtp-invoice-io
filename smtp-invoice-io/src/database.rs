use crate::structs::{FacturaElectronica, MensajeHacienda};
use bson::{bson, doc, Document};
use mongodb::{options::ClientOptions, Client};

async fn get_database() -> Result<mongodb::Database, mongodb::error::Error> {
    let mut client_options = ClientOptions::parse("mongodb://localhost:27017")?;
    client_options.app_name = Some("Factura28".to_string());
    let client = Client::with_options(client_options)?;
    let db = client.database("factura28");
    Ok(db)
}

pub async fn insert_many_factura(
    facturas: Vec<FacturaElectronica>,
) -> Result<u8, mongodb::error::Error> {
    let db = get_database().await?;
    let collection = db.collection("factura_electronica");
    let facturas_bson: Vec<_> = facturas
        .into_iter()
        .map(|x| {
            let bson_data = bson::to_bson(&x).unwrap();
            let mut doc = Document::new();
            if let bson::Bson::Document(document) = bson_data {
                doc = document;
            }
            doc
        })
        .collect();
    collection.insert_many(facturas_bson, None)?;
    Ok(1)
}

pub async fn insert_many_mensaje(
    facturas: Vec<MensajeHacienda>,
) -> Result<u8, mongodb::error::Error> {
    let db = get_database().await?;
    let collection = db.collection("mensaje_hacienda");
    let mensaje_bson: Vec<_> = facturas
        .into_iter()
        .map(|x| {
            let bson_data = bson::to_bson(&x).unwrap();
            let mut doc = Document::new();
            if let bson::Bson::Document(document) = bson_data {
                doc = document;
            }
            doc
        })
        .collect();
    collection.insert_many(mensaje_bson, None)?;
    Ok(1)
}
