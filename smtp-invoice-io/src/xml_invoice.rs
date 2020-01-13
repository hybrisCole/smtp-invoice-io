use async_imap::error::Result;
use async_std::prelude::*;
use sxd_document::parser;
use sxd_xpath::{Context, Factory, Value};
use std::str;
use serde_xml_rs::from_str;
use crate::structs::{FacturaElectronica, MensajeHacienda};

const ELECTRONIC_INVOICE: &str = "//*[name()='FacturaElectronica']";
const ELECTRONICE_MESSAGE: &str = "//*[name()='MensajeHacienda']";
pub async fn process(invoice_list: &Vec<Vec<u8>>) -> Result<(Vec<FacturaElectronica>,Vec<MensajeHacienda>)> {
    let mut vec_factura_electronica: Vec<FacturaElectronica> = Vec::new();
    let mut vec_mensaje_hacienda: Vec<MensajeHacienda> = Vec::new();
    for invoice in invoice_list {
        let xml_invoice = str::from_utf8(&invoice).expect("Invoice is not a valid string");
        let parsed_invoice = parser::parse(&xml_invoice)
            .expect("<root>No XML</root>");
        let document = parsed_invoice.as_document();
        let factory = Factory::new();
        let xpath_invoice = factory.build(ELECTRONIC_INVOICE).expect("Could not compile XPath");
        let xpath_invoice = xpath_invoice.expect("No XPath was compiled");
        let context = Context::new();
        let value_xpath_invoice = xpath_invoice.evaluate(&context, document.root())
            .expect("XPath evaluation failed");
        if (&value_xpath_invoice.string() != "") {
            let factura: FacturaElectronica = from_str(&xml_invoice).expect("Not a FacturaElectronica XML");
            vec_factura_electronica.push(factura);
        }else{
            let xpath_electronic_message = factory.build(ELECTRONICE_MESSAGE).expect("Could not compile XPath");
            let xpath_electronic_message = xpath_electronic_message.expect("No XPath was compiled");
            let context = Context::new();
            let value_xpath_electronic_message = xpath_electronic_message.evaluate(&context, document.root())
                .expect("XPath evaluation failed");
            if (&value_xpath_electronic_message.string() != "") {
                let mensaje_hacienda: MensajeHacienda = from_str(&xml_invoice)
                    .unwrap();
                vec_mensaje_hacienda.push(mensaje_hacienda);
            }else {
                println!("{:#?}", "UNKNOWN XML");
            }
        }
    }
    Ok((vec_factura_electronica, vec_mensaje_hacienda))
}