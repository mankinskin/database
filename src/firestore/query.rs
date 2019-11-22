use crate::{
    database,
    database::Database,
    error::DatabaseError,
    Firestore,
    document::{
        Document,
    },
    query::ordering::{
        Ordering,
    },
};

use google_firestore::{
    RunQueryRequest,
    StructuredQuery,
};
pub struct FirestoreQuery {
    pub(crate) collections: Vec<CollectionSelector>,
    pub(crate) filter: google_firestore::Filter,
    pub(crate) orders: Vec<google_firestore::Order>,
    pub(crate) limit: u32,
    pub(crate) skip: u32,
}

// CollectionSelectors are used to select
// which collections to query
#[derive(Clone)]
pub struct CollectionSelector(google_firestore::CollectionSelector);

impl CollectionSelector {
    pub fn set_all_descendants(&mut self, all_descendants: bool) {
        self.0.all_descendants = Some(all_descendants);
    }
}

impl<T: ToString> From<T> for CollectionSelector {
    fn from(collection_id: T) -> Self {
        CollectionSelector(google_firestore::CollectionSelector {
            all_descendants: None,
            collection_id: Some(collection_id.to_string()),
        })
    }
}

impl From<CollectionSelector> for google_firestore::CollectionSelector {
    fn from(c: CollectionSelector) -> Self {
        c.0
    }
}

impl Default for CollectionSelector {
    fn default() -> Self {
        CollectionSelector(google_firestore::CollectionSelector::default())
    }
}
use crate::query::{Query};
impl Query<'static, Firestore> for FirestoreQuery {
    fn new() -> Self {
        FirestoreQuery {
            collections: Vec::new(),
            filter: google_firestore::Filter::default(),
            orders: Vec::new(),
            limit: 0,
            skip: 0,
        }
    }
    fn collections(self, mut collections: Vec<CollectionSelector>) -> Self
    {
        Self {
            collections: {
                let mut newcollections = self.collections.clone();
                newcollections.append(&mut collections);
                newcollections
            },
            ..self
        }
    }
    fn order_by(self, field: &str, direction: Ordering) -> Self {
        Self {
            orders: {
                let mut neworders = self.orders.clone();
                neworders.push(google_firestore::Order {
                    field: Some(google_firestore::FieldReference {
                        field_path: Some(field.to_string()),
                    }),
                    direction: Some(direction.to_string()),
                });
                neworders
            },
            ..self
        }
    }
    fn limit(self, limit: u32) -> Self {
        Self {
            limit,
            ..self
        }
    }
    fn skip(self, skip: u32) -> Self {
        Self {
            skip,
            ..self
        }
    }

    fn run(self) -> Result<Vec<Document>, DatabaseError> {
        let req = RunQueryRequest {
            structured_query: Some(StructuredQuery {
                    from: Some(self.collections
                                   .iter()
                                   .map(|c| c.0.clone())
                                   .collect()),
                    where_: Some(self.filter),
                    order_by: Some(self.orders.clone()),
                    limit:  if self.limit == 0 {
                                None
                            } else {
                                Some(self.limit as i32)
                            },
                    offset:  if self.skip == 0 {
                                None
                            } else {
                                Some(self.skip as i32)
                            },
                    ..StructuredQuery::default()
                }),
            ..RunQueryRequest::default()
        };
        let (_httpresponse, results) = database().db()
            .projects()
            .databases_documents_run_query(req,
                                           &database().get_path())
            .doit()?;
        Ok(results.iter()
                  .flat_map(|res| (*res).clone().document)
                  .map(|d| Document::from(d))
                  .collect())
    }
}

#[cfg(test)]
mod tests {
    #![allow(unused)]
    use crate::{
        Firestore,
    };
    use super::{
        CollectionSelector,
        Query,
    };
    use crate::document::{
        Document,
        FieldValue,
        ArrayValue,
    };
    mod single_collection {
        use super::*;
        use crate::{
            database,
            database::Database,
            query::{
                filter::{
                    Filter,
                    FilterOp,
                },
            },
        };

        fn test_collection() -> Vec<CollectionSelector> {
            vec![CollectionSelector::from("test")]
        }

        mod unary_filter {
            use super::*;
            #[test]
            fn is_null() {
                let query = database().query()
                              .collections(test_collection())
                              .filter("test_null", FilterOp::<()>::IS_NULL);
                let expected = vec![
                            Document::builder()
                                .name(&(database().get_path() + "/test/" + "IsNull"))
                                .field("test_null", FieldValue::null_value())
                                .build()
                           ];
                match query.run() {
                    Ok(results) => {
                        assert_eq!(results, expected);
                    },
                    Err(e) => {
                        panic!("Failed to run query: {}", e);
                    }
                }
            }
            // TODO: Test IS_NAN
        }

        mod field_filter {
            use super::*;
            #[test]
            fn equal() {
                let query = database().query()
                              .collections(test_collection())
                              .filter("string", FilterOp::EQUAL("TestString"));
                let expected = vec![
                            Document::builder()
                                .name(&(database().get_path() + "/test/" + "StringEquals"))
                                .field("string", "TestString")
                                .build()
                           ];
                match query.run() {
                    Ok(results) => {
                        assert_eq!(results, expected);
                    },
                    Err(e) => {
                        panic!("Failed to run query: {}", e);
                    }
                }
            }
            #[test]
            fn less_than() {
                let query = database().query()
                              .collections(test_collection())
                              .filter("integer", FilterOp::LESS_THAN(5));
                let expected = vec![
                            Document::builder()
                                .name(&(database().get_path() + "/test/" + "LessThan"))
                                .field("integer", 3)
                                .build()
                           ];
                match query.run() {
                    Ok(results) => {
                        assert_eq!(results, expected);
                    },
                    Err(e) => {
                        panic!("Failed to run query: {}", e);
                    }
                }
            }
            #[test]
            fn less_than_or_equal() {
                let query = database().query()
                              .collections(test_collection())
                              .filter("integer", FilterOp::LESS_THAN_OR_EQUAL(5));
                let expected = vec![
                            Document::builder()
                                .name(&(database().get_path() + "/test/" + "LessThan"))
                                .field("integer", 3)
                                .build(),
                            Document::builder()
                                .name(&(database().get_path() + "/test/" + "GreaterThanOrEqual"))
                                .field("integer", 5)
                                .build(),
                            Document::builder()
                                .name(&(database().get_path() + "/test/" + "LessThanOrEqual"))
                                .field("integer", 5)
                                .build(),
                           ];
                match query.run() {
                    Ok(results) => {
                        assert_eq!(results, expected);
                    },
                    Err(e) => {
                        panic!("Failed to run query: {}", e);
                    }
                }
            }
            #[test]
            fn greater_than() {
                let query = database().query()
                              .collections(test_collection())
                              .filter("integer", FilterOp::GREATER_THAN(5));
                let name = database().get_path() + "/test/" + "GreaterThan";
                let expected = vec![
                            Document::builder()
                                .name(&name)
                                .field("integer", 6)
                                .build(),
                           ];
                match query.run() {
                    Ok(results) => {
                        assert_eq!(results, expected);
                    },
                    Err(e) => {
                        panic!("Failed to run query: {}", e);
                    }
                }
            }
            #[test]
            fn greater_than_or_equal() {
                let query = database().query()
                              .collections(test_collection())
                              .filter("integer", FilterOp::GREATER_THAN_OR_EQUAL(5));
                let expected = vec![
                            Document::builder()
                                .name(&(database().get_path() + "/test/" + "GreaterThanOrEqual"))
                                .field("integer", 5)
                                .build(),
                            Document::builder()
                                .name(&(database().get_path() + "/test/" + "LessThanOrEqual"))
                                .field("integer", 5)
                                .build(),
                            Document::builder()
                                .name(&(database().get_path() + "/test/" + "GreaterThan"))
                                .field("integer", 6)
                                .build(),
                           ];
                match query.run() {
                    Ok(results) => {
                        assert_eq!(results, expected);
                    },
                    Err(e) => {
                        panic!("Failed to run query: {}", e);
                    }
                }
            }
            #[test]
            fn array_contains() {
                let query = database().query()
                              .collections(test_collection())
                              .filter("numbers", FilterOp::ARRAY_CONTAINS(5));
                let name = database().get_path() + "/test/" + "ArrayContains";
                let expected = vec![
                                Document::builder()
                                    .name(&name)
                                    .field("numbers",
                                           vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]
                                            .iter()
                                            .map(|&v|
                                                 FieldValue::from(v))
                                                    .collect::<Vec<FieldValue>>())
                                    .build()
                           ];
                match query.run() {
                    Ok(results) => {
                        assert_eq!(results, expected);
                    },
                    Err(e) => {
                        panic!("Failed to run query: {}", e);
                    }
                }
            }
        }
    }
}
