#[macro_use(bson, doc)]
extern crate bson;

#[macro_use]
extern crate serde_derive;
extern crate chrono;
extern crate mailparse;
extern crate serde;
extern crate serde_xml_rs as serde_xml;
extern crate sxd_document;
extern crate sxd_xpath;

use chrono::format::ParseError;
use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime};
use serde_xml_rs::from_str;
use std::time::{Duration, Instant};
use sxd_document::parser;
use sxd_xpath::{Context, Factory, Value};

use async_std::io::{self};
use async_std::prelude::*;
use async_std::stream;
use async_std::task;

mod database;
mod imap_unseen;
mod structs;
mod xml_invoice;

fn main() -> io::Result<()> {
    task::block_on(async {
        let mut interval = stream::interval(Duration::from_secs(4));
        while let Some(_) = interval.next().await {
            let now = Instant::now();
            match imap_unseen::list().await {
                Ok(list) => {
                    match xml_invoice::process(&list).await {
                        Ok(invoice_list) => {
                            println!("{:#?}", invoice_list);
                            let invoice_list = invoice_list.0;
                            if (invoice_list.len() > 0) {
                                database::insert_many_factura(invoice_list)
                                    .await
                                    .expect("Error inserting many factura");
                            }
                        }
                        Err(e) => println!("Error getting xml invoices: {}", e),
                    }
                    println!("{:#?}", list.len());
                }
                Err(e) => println!("Error getting imap information: {}", e),
            }
            let new_now = Instant::now();
            println!("{:#?}", new_now.duration_since(now));
        }
        Ok(())
    })
}
