use crate::{
    database::Database,
    error::DatabaseError,
    document::{Document},
    firestore::{Firestore},
    collection::{Collection},
};
use std::vec::Vec;
use futures::future::{
    Future,
};

pub struct FirestoreCollection<'a> {
    firestore: &'a Firestore,
    collection_id: String,
}

impl Collection<'static> for FirestoreCollection<'static> {
    type Database = Firestore;
    fn new<T: ToString>(database: &'static Firestore, id: T) -> Self {
        Self {
            firestore: database,
            collection_id: id.to_string(),
        }
    }

    fn get_path(&self) -> String {
        self.firestore.collection_path(self.collection_id.clone())
    }

    fn get_document<T: ToString>(
        &self,
        document_id: T,
        ) -> Box<dyn Future<Item=Document, Error=DatabaseError> + Send> {
        self.firestore.get_document(self.collection_id.clone(), document_id)
    }

    fn create_document(
        &self,
        document: Document,
        ) -> Box<dyn Future<Item=String, Error=DatabaseError> + Send> {
        self.firestore.create_document(self.collection_id.clone(), document)
    }
    fn delete_document<T: ToString>(
        &self,
        document_id: T,
        ) -> Box<dyn Future<Item=(), Error=DatabaseError> + Send> {
        self.firestore.delete_document(self.collection_id.clone(), document_id)
    }

    fn get_documents(
        &self
        ) -> Box<dyn Future<Item=Vec<Document>, Error=DatabaseError>> {
        self.firestore.get_documents(self.collection_id.clone())
    }
}

#[cfg(test)]
mod tests {
    #![allow(unused)]
    use crate::{
        collection,
        document::{
            Document,
            tests::{
                test_document,
            },
        },
    };
    use super::*;
    #[test]
    fn access_collection() {
        let collection = collection("test");
    }
    #[test]
    fn get_document() {
        let id = format!("{}/{}",
                         "test",
                         "ArrayContains");
        collection("test").get_document(id).wait().unwrap();
    }
    #[test]
    fn document_test() {
        let id =  "TestDocument";
        let doc = test_document(format!("{}/{}", collection("test").get_path(), id));
        // delete to avoid possible conflict from previous tests
        collection("test").delete_document(id.clone()).wait().unwrap();
        collection("test").create_document(doc.clone()).wait().unwrap();
        let created = collection("test").get_document(id.clone()).wait().unwrap();
        assert_eq!(created, doc);
        collection("test").delete_document(id.clone()).wait().unwrap();
        collection("test").get_document(id)
            .wait().expect_err("Got document after calling DELETE!");
    }
}
