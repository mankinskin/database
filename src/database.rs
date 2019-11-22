use crate::error::*;
use crate::document::{Document};
use crate::collection::{Collection};

use futures::future::{
    Future,
};

pub trait Database<'a>
{
    type Access;
    type Collection: Collection<'a>;
    type Query;

    fn new(access: Self::Access) -> Self;
    fn get_path(&'a self) -> String;
    fn collection<T: ToString>(
        &'a self,
        collection_id: T,
        ) -> Self::Collection;

    fn query(&'a self) -> Self::Query;

    fn collection_path<T: ToString>(&'a self, collection_id: T) -> String {
        format!("{}/{}", self.get_path(), collection_id.to_string())
    }

    fn create_document<T: ToString>(
        &'a self,
        collection_id: T,
        document: Document
        ) -> Box<dyn Future<Item=String, Error=DatabaseError> + Send>;
    fn get_document<A: ToString, B: ToString>(
        &'a self,
        collection_id: A,
        document_id: B,
        ) -> Box<dyn Future<Item=Document, Error=DatabaseError> + Send>;
    fn delete_document<A: ToString, B: ToString>(
        &'a self,
        collection_id: A,
        document_id: B,
        ) -> Box<dyn Future<Item=(), Error=DatabaseError> + Send>;
    fn get_documents<A: ToString>(
        &'a self,
        collection_id: A,
        ) -> Box<dyn Future<Item=Vec<Document>, Error=DatabaseError>>;
}
