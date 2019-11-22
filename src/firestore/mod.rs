use crate::client::{Client};
use crate::query::{Query};
use crate::document::{Document};
use crate::error::*;

pub mod collection;
pub mod query;
pub mod filter;
pub mod access;

use access::{
    FirestoreAccess,
};
use std::sync::{Arc, Mutex};

use actix_web::{
    web::{
        block,
    },
};
use futures::future::{
    Future,
};

pub struct Firestore
{
    pub(crate) db: Arc<Mutex<google_firestore::Firestore<Client, FirestoreAccess>>>,
    project_id: String,
}

impl Firestore {
    pub(crate) fn db(&self) -> std::sync::MutexGuard<'_, google_firestore::Firestore<Client, FirestoreAccess>> {
        self.db.lock().unwrap()
    }
}
use super::{
    collection::Collection,
    database::Database,
};
impl Database<'static> for Firestore
{
    type Access = FirestoreAccess;
    type Collection = collection::FirestoreCollection<'static>;
    type Query = query::FirestoreQuery;

    fn new(access: Self::Access) -> Self {
        Firestore {
            project_id: access.project_id.clone(),
            db: Arc::new(Mutex::new(
                    google_firestore::Firestore::new(Client::default(), access)
                    )),
        }
    }

    fn get_path(&'static self) -> String {
        format!(
            "projects/{}/databases/(default)/documents",
            self.project_id
            )
    }

    fn collection<T: ToString>(
        &'static self,
        collection_id: T,
        ) -> Self::Collection {
        Self::Collection::new(&self, collection_id)
    }

    fn query(&'static self) -> Self::Query
    {
        Self::Query::new()
    }
    fn create_document<T: ToString>(
        &'static self,
        collection_id: T,
        document: Document
        ) -> Box<dyn Future<Item=String, Error=DatabaseError> + Send> {
        let doc = google_firestore::Document {
            name: None,
            ..document.clone().into()
        };
        let collection_id = collection_id.to_string();
        let path = self.get_path().clone();
        Box::new(block(move || {
            self.db()
                .projects()
                .databases_documents_create_document(
                    doc.clone(),
                    &path,
                    &collection_id.to_string())
                .document_id(&document.name()).doit()
                .map(|(_r, _d)| document.name().to_string())
                .map_err(|e| DatabaseError::from(e))
        })
        .map_err(|e| DatabaseError::from(e))
                )
    }
    fn get_document<A: ToString, B: ToString>(
        &'static self,
        collection_id: A,
        document_id: B,
        ) -> Box<dyn Future<Item=Document, Error=DatabaseError> + Send> {
        let path = self.collection_path(collection_id);
        let document_id = document_id.to_string();
        Box::new(block(move ||
                       self.db()
                       .projects()
                       .databases_documents_get(&format!("{}/{}",
                                                         path,
                                                         &document_id)).doit()
                       .map(|(_r, d)| Document::from(d))
                       .map_err(|e| DatabaseError::from(e))
                      )
                       .map_err(|e| DatabaseError::from(e))
                )
    }
    fn delete_document<A: ToString, B: ToString>(
        &'static self,
        collection_id: A,
        document_id: B,
        ) -> Box<dyn Future<Item=(), Error=DatabaseError> + Send> {
        let path = self.collection_path(collection_id);
        let document_id = document_id.to_string();
        Box::new(block(move ||
                       self.db()
                       .projects()
                       .databases_documents_delete(&format!("{}/{}",
                                                            path,
                                                            &document_id)).doit()
                       .map(|(_r, _)| ())
                       .map_err(|e| DatabaseError::from(e))
                      )
                       .map_err(|e| DatabaseError::from(e))
                )
    }
    fn get_documents<A: ToString>(
        &'static self,
        collection_id: A,
        ) -> Box<dyn Future<Item=Vec<Document>, Error=DatabaseError>> {
        let collection_id = collection_id.to_string();
        let path = self.get_path();
        Box::new(block(move ||
                       self.db()
                       .projects()
                       .databases_documents_list(
                           &path,
                           &collection_id).doit()
                       .map_err(|e| DatabaseError::from(e))
                       .map(|(_r, res)|
                            res.documents
                            .unwrap_or(Vec::new())
                            .iter()
                            .map(|doc| doc.clone().into())
                            .collect()
                           ))
                       .map_err(|e| DatabaseError::from(e))
                )
    }
}
