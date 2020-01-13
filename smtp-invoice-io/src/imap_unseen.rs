extern crate async_native_tls;
extern crate mailparse;
use async_imap::error::Result;
use async_std::prelude::*;

const DOMAIN: &str = "imap.gmail.com";
const EMAIL: &str = "mav.facturas.com@gmail.com";
const PASSWORD: &str = "sf1ct5r1";

pub async fn list() -> Result<Vec<Vec<u8>>> {
    let mut vec_xml_raw: Vec<Vec<u8>> = Vec::new();
    let tls = async_native_tls::TlsConnector::new();
    let imap_client = async_imap::connect((DOMAIN, 993), DOMAIN, tls).await?;
    let mut imap_session = imap_client.login(EMAIL, PASSWORD).await.map_err(|e| e.0)?;
    imap_session.select("INBOX").await?;
    let msgs_ids = imap_session.search("UNSEEN").await?;
    for msg_id in &msgs_ids {
        let messages_stream = imap_session.fetch(msg_id.to_string(), "BODY[]").await?;
        let messages: Vec<_> = messages_stream.collect::<Result<_>>().await?;
        let message = messages.first().expect("No Messages");
        let body = message.body().expect("message did not have a body!");
        let parsed = mailparse::parse_mail(body).unwrap();
        for part in &parsed.subparts {
            if part.ctype.mimetype == "text/xml" {
                let body = part.get_body_raw().unwrap();
                vec_xml_raw.push(body);
            }
        }
    }
    Ok(vec_xml_raw)
}
