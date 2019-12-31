/*
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_xml_rs as serde_xml;
extern crate chrono;
extern crate htmlescape;

extern crate imap;
extern crate mailparse;
extern crate native_tls;
extern crate sxd_document;
extern crate sxd_xpath;
extern crate postgres;

use tokio::prelude::*;
use tokio::timer::Interval;
use std::time::{Duration, Instant};
use sxd_document::parser;
use sxd_xpath::{Factory, Context, Value};
use serde_xml_rs::{from_str};
use postgres::{Connection, TlsMode};
use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime};
use chrono::format::ParseError;

#[derive(Deserialize, Debug)]
struct Fact {
    Emisor: Emisor
}

#[derive(Deserialize, Debug)]
struct Emisor {
    #[serde(default)]
    NombreComercial: String,
}

const DOMAIN:&str = "imap.gmail.com";
const EMAIL:&str = "mav.facturas.com@gmail.com";
const PASSWORD:&str = "sf1ct5r1";

fn main() {
    let package = parser::parse("<?xml version=\"1.0\" encoding=\"UTF-8\"?><Fact><Emisor><NombreComercial>BAC San José</NombreComercial></Emisor></Fact>")
        .expect("<root>No XML</root>");
    let fact: Fact = from_str("<?xml version=\"1.0\" encoding=\"UTF-8\"?><Fact><Emisor><NombreComercial>BAC San José</NombreComercial></Emisor></Fact>")
        .unwrap();
    println!("{:#?}", &fact.Emisor.NombreComercial);
}
*/
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_xml_rs as serde_xml;
extern crate chrono;
extern crate htmlescape;

#[derive(Deserialize, Debug)]
struct MensajeHacienda {
    Clave: String,
    NombreEmisor: String,
    TipoIdentificacionEmisor: String,
    NumeroCedulaEmisor: String,
    NombreReceptor: String,
    TipoIdentificacionReceptor: String,
    NumeroCedulaReceptor: String,
    Mensaje: String,
    #[serde(default)]
    DetalleMensaje: String,
    MontoTotalImpuesto: String,
    TotalFactura: String
}

#[derive(Deserialize, Debug)]
struct FacturaElectronica {
    Clave: String,
    CodigoActividad: String,
    NumeroConsecutivo: String,
    FechaEmision: String,
    Emisor: Emisor,
    Receptor: Receptor,
    CondicionVenta: String,
    MedioPago: String,
    DetalleServicio: DetalleServicio,
    ResumenFactura: ResumenFactura,
    #[serde(default)]
    Otros: Otros
}

#[derive(Deserialize, Debug, Default)]
struct Otros {
    OtroTexto: Vec<OtroTexto>
}

#[derive(Deserialize, Debug)]
struct OtroTexto{
    #[serde(rename = "$value", default)]
    Body: String,
    codigo: String
}

#[derive(Deserialize, Debug)]
struct ResumenFactura{
    CodigoTipoMoneda: CodigoTipoMoneda,
    #[serde(default)]
    TotalServGravados: String,
    #[serde(default)]
    TotalServExentos: String,
    #[serde(default)]
    TotalServExonerado: String,
    #[serde(default)]
    TotalMercanciasGravadas: String,
    #[serde(default)]
    TotalMercanciasExentas: String,
    #[serde(default)]
    TotalMercExonerada: String,
    #[serde(default)]
    TotalGravado: String,
    #[serde(default)]
    TotalExento: String,
    #[serde(default)]
    TotalExonerado: String,
    #[serde(default)]
    TotalVenta: String,
    #[serde(default)]
    TotalDescuentos: String,
    #[serde(default)]
    TotalVentaNeta: String,
    #[serde(default)]
    TotalImpuesto: String,
    #[serde(default)]
    TotalIVADevuelto: String,
    #[serde(default)]
    TotalOtrosCargos: String,
    #[serde(default)]
    TotalComprobante: String
}

#[derive(Deserialize, Debug)]
struct CodigoTipoMoneda {
    CodigoMoneda: String,
    TipoCambio: String
}

#[derive(Deserialize, Debug)]
struct DetalleServicio {
    LineaDetalle: Vec<LineaDetalle>
}
#[derive(Deserialize, Debug)]
struct LineaDetalle {
    NumeroLinea: String,
    #[serde(default)]
    CodigoComercial: CodigoComercial,
    Cantidad: String,
    UnidadMedida: String,
    Detalle: String,
    PrecioUnitario: String,
    MontoTotal: String,
    SubTotal: String,
    Impuesto: Impuesto,
    #[serde(default)]
    ImpuestoNeto: String,
    MontoTotalLinea: String
}

#[derive(Deserialize, Debug)]
struct Impuesto {
    Codigo: String,
    CodigoTarifa: String,
    Tarifa: String,
    Monto: String
}

#[derive(Deserialize, Debug, Default)]
struct CodigoComercial{
    Tipo: String,
    Codigo: String
}

#[derive(Deserialize, Debug)]
struct Emisor {
    Nombre: String,
    Identificacion: Identificacion,
    #[serde(default)]
    NombreComercial: String,
    Ubicacion: Ubicacion,
    #[serde(default)]
    Telefono: Telefono,
    CorreoElectronico: String,
}

#[derive(Deserialize, Debug)]
struct Receptor {
    Nombre: String,
    Identificacion: Identificacion,
    #[serde(default)]
    Telefono: Telefono,
    CorreoElectronico: String,
}

#[derive(Deserialize, Debug)]
struct Identificacion {
    Tipo: String,
    Numero: String
}

#[derive(Deserialize, Debug)]
struct Ubicacion {
    Provincia: String,
    Canton: String,
    Distrito: String,
    OtrasSenas: String
}

#[derive(Deserialize, Debug, Default)]
struct Telefono {
    CodigoPais: String,
    NumTelefono: String
}

extern crate imap;
extern crate mailparse;
extern crate native_tls;
extern crate sxd_document;
extern crate sxd_xpath;
extern crate postgres;

use tokio::process::Command;
use tokio::task;
use tokio::time;
use std::time::{Duration, Instant};
use sxd_document::parser;
use sxd_xpath::{Factory, Context, Value};
use serde_xml_rs::{from_str};
use postgres::{Connection, TlsMode};
use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime};
use chrono::format::ParseError;

const DOMAIN:&str = "imap.gmail.com";
const EMAIL:&str = "mav.facturas.com@gmail.com";
const PASSWORD:&str = "sf1ct5r1";

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let imap_interval = task::spawn(imap_interval());
    imap_interval.await??;
    Ok(())
}

async fn imap_interval() -> Result<(), std::io::Error> {
    let mut interval = time::interval(Duration::from_secs(1));
    loop {
        imap_io();
        interval.tick().await;
        Command::new("date").spawn()?.await?;
    }
}

fn imap_io() {
    let now = Instant::now();
    let conn = Connection::connect("postgresql://cole28:cole28@localhost:5432/factura28", TlsMode::None)
        .unwrap();
    let tls = native_tls::TlsConnector::builder().build().unwrap();
    let client = imap::connect((DOMAIN, 993), DOMAIN, &tls).unwrap();
    let stmt_create_factura =
        conn.prepare_cached("CALL create_factura($1, $2, $3, $4, $5, $6)").unwrap();
    let stmt_create_emisor =
        conn.prepare_cached("CALL create_emisor($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)")
            .unwrap();
    let mut imap_session = client
        .login(EMAIL, PASSWORD)
        .map_err(|e| e.0)
        .unwrap();
    imap_session.select("INBOX").unwrap();
    match imap_session.search("UNSEEN") {
        Ok(msgs_ids) => {
            for msg_id in &msgs_ids {
                let messages = imap_session.fetch(msg_id.to_string(), "BODY[]").unwrap();
                let message = if let Some(m) = messages.iter().next() {
                    m
                } else {
                    return ();
                };
                let body = message.body().expect("message did not have a body!");
                let parsed = mailparse::parse_mail(body).unwrap();
                for part in &parsed.subparts {
                    if part.ctype.mimetype == "text/xml" {
                        let body = part.get_body().unwrap();
                        println!("{:#?}",&body);
                        let package = parser::parse(&body)
                            .expect("<root>No XML</root>");
                        let document = package.as_document();
                        let factory = Factory::new();
                        let xpath = factory.build("//*[name()='FacturaElectronica']").expect("Could not compile XPath");
                        let xpath = xpath.expect("No XPath was compiled");
                        let context = Context::new();

                        let value = xpath.evaluate(&context, document.root())
                            .expect("XPath evaluation failed");
                        if (&value.string() != "") {
                            let new_now_factura = Instant::now();
                            let factura: FacturaElectronica = from_str(&body)
                                .unwrap();
                            stmt_create_factura.execute(&[
                                &factura.Clave,
                                &factura.CodigoActividad,
                                &factura.NumeroConsecutivo,
                                &(NaiveDateTime::parse_from_str(&factura.FechaEmision, "%+").unwrap()),
                                &factura.CondicionVenta,
                                &factura.MedioPago
                            ]).unwrap();
                            stmt_create_emisor.execute(&[
                                &factura.Emisor.Identificacion.Numero,
                                &factura.Emisor.Identificacion.Tipo,
                                &factura.Emisor.Nombre,
                                &(String::from(&factura.Emisor.NombreComercial)),
                                &factura.Emisor.Ubicacion.Provincia,
                                &factura.Emisor.Ubicacion.Canton,
                                &factura.Emisor.Ubicacion.Distrito,
                                &factura.Emisor.Ubicacion.OtrasSenas,
                                &factura.Emisor.Telefono.CodigoPais,
                                &factura.Emisor.Telefono.NumTelefono,
                                &factura.Emisor.CorreoElectronico,
                            ]).unwrap();
                            println!("{:#?}",htmlescape::decode_html(&factura.Emisor.NombreComercial));
                            println!("{:#?}", factura);
                            println!("{:?}", new_now_factura.duration_since(now));
                        } else {
                            let xpath = factory.build("//*[name()='MensajeHacienda']").expect("Could not compile XPath");
                            let xpath = xpath.expect("No XPath was compiled");
                            let context = Context::new();

                            let value = xpath.evaluate(&context, document.root())
                                .expect("XPath evaluation failed");
                            if (&value.string() != "") {
                                let mensajeHacienda: MensajeHacienda = from_str(&body)
                                    .unwrap();
                                println!("{:#?}", mensajeHacienda);
                            } else {
                                println!("YIXXX");
                            }
                        }
                     }
                }
                println!("{:?}", parsed.subparts.len());
                let new_now = Instant::now();
                println!("{:?}", new_now.duration_since(now));
            }
        }
        Err(e) => println!("Error Fetching emails: {}", e),
    }
    imap_session.logout().unwrap();
}