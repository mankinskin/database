use crate::error::*;
use crate::{
    database::Database,
    document::{Document},
};

use futures::future::{
    Future,
};

pub trait Collection<'db> : Sized {
    type Database: Database<'db, Collection=Self>;
    fn new<T: ToString>(database: &'db Self::Database, id: T) -> Self;

    fn get_path(&self) -> String;

    fn get_document<T: ToString>(
        &self,
        document_id: T,
    ) -> Box<dyn Future<Item=Document, Error=DatabaseError> + Send>;

    fn create_document(
        &self,
        document: Document,
    ) -> Box<dyn Future<Item=String, Error=DatabaseError> + Send>;
    fn delete_document<T: ToString>(
        &self,
        document_id: T,
    ) -> Box<dyn Future<Item=(), Error=DatabaseError> + Send>;

    fn get_documents(
        &self
    ) -> Box<dyn Future<Item=Vec<Document>, Error=DatabaseError>>;
}
