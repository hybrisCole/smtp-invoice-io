const DOMAIN:&str = "imap.gmail.com";
const EMAIL:&str = "mav.facturas.com@gmail.com";
const PASSWORD:&str = "sf1ct5r1";

extern crate imap;
extern crate mailparse;
extern crate native_tls;

use tokio::prelude::*;
use tokio::timer::Interval;

use std::time::{Duration, Instant};

fn main() {
    let task = Interval::new(Instant::now(), Duration::from_millis(5000))
        .for_each(|instant| {
            imap_io();
            println!("fire; instant={:?}", instant);
            Ok(())
        })
        .map_err(|e| panic!("interval errored; err={:?}", e));

    tokio::run(task);
}

fn imap_io() {
    let tls = native_tls::TlsConnector::builder().build().unwrap();
    let client = imap::connect((DOMAIN, 993), DOMAIN, &tls).unwrap();
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
                        println!("{:?}", part.ctype.mimetype);
                        println!("{:?}", part.get_body().unwrap());
                    }
                }
                println!("{:?}", parsed.subparts.len());
            }
        }
        Err(e) => println!("Error Fetching emails: {}", e),
    }
    imap_session.logout().unwrap();
}