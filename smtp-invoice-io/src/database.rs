use crate::structs::{FacturaElectronica, MensajeHacienda};
use bson::{bson, doc};
use mongodb::{options::ClientOptions, Client};
use serde_json::json;

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
    let facturas_bson: Vec<bson::ordered::OrderedDocument> =
        facturas.into_iter().map(|x| {
            let serialized = bson::to_bson(&x).unwrap();
            let doc = doc!{"data": serialized.clone()};
            doc
        }).collect();
    collection.insert_many(facturas_bson, None)?;
    Ok(1)
}
