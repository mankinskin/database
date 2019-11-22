extern crate google_firestore;
extern crate yup_oauth2 as oauth2;
extern crate hyper;
extern crate hyper_rustls;

extern crate chrono;
extern crate serde;
extern crate serde_json as json;
#[macro_use]
extern crate logger;
extern crate futures;
extern crate lazy_static;
extern crate actix_web;

pub mod access;
mod client;
pub mod document;
pub mod collection;
pub mod error;
pub mod query;
pub mod firestore;
pub mod database;

use lazy_static::lazy_static;
use firestore::access::get_service_account_key;
use crate::firestore::Firestore;
use crate::database::Database;
use access::{Access};

// lazy_static makes it possible to run code to initialize statics
lazy_static! {
    static ref DATABASE: Firestore = {
        let key = get_service_account_key();
        let access = Access::from(key);
        Firestore::new(access)
    };
}

pub fn init() {
    // DATABASE is initialized here! (due to lazy_static)
    database();
}

pub fn database() -> &'static Firestore {
    &DATABASE
}

pub fn collection<T: ToString>(id: T) -> firestore::collection::FirestoreCollection<'static> {
    database().collection(id)
}

#[cfg(test)]
mod tests {
    #![allow(unused)]
    use super::*;

    #[test]
    fn authentication() {
        init();
    }
}
