pub mod filter;
pub mod ordering;

use crate::{
    error::*,
    database::{
        Database,
    },
    document::{
        Document,
    },
    firestore::query::{
        CollectionSelector,
    },
};

use ordering::{ Ordering };

pub trait Query<'a, DB: Database<'a>> {
    fn new() -> Self;
    // adds a field filter to the query

    // define the collections to query
    fn collections(
        self,
        collections: Vec<CollectionSelector>,
        ) -> Self;
    // define result orders for field
    fn order_by(
        self,
        field: &str,
        direction: Ordering,
        ) -> Self;
    // max number of results to return
    fn limit(
        self,
        limit: u32,
        ) -> Self;
    // number of results to skip
    fn skip(
        self,
        skip: u32,
        ) -> Self;
    // run the query and return the results
    fn run(self) -> Result<Vec<Document>, DatabaseError>;
}
