use crate::{
    document::{
        FieldValue,
    },
    query::filter::{
        Filter,
        FilterOp,
    },
};
use crate::firestore::query::{
    FirestoreQuery,
};
use google_firestore::{
    FieldFilter,
    FieldReference,
    CompositeFilter,
};

impl<T: Clone + Into<FieldValue>> Filter<T> for FirestoreQuery {
    fn filter(self, field: &str, op: FilterOp<T>) -> Self {
        Self {
            filter: FilterDef(field, op.into()).into(),
            ..self
        }
    }
    fn and(self, other: Self) -> Self {
        Self {
            filter: google_firestore::Filter{
                composite_filter: Some(CompositeFilter {
                    filters: Some(vec![self.filter, other.filter]),
                    op: Some("AND".to_string())
                }),
                ..google_firestore::Filter::default()
            },
            ..self
        }
    }
    fn or(self, other: Self) -> Self {
        Self {
            filter: google_firestore::Filter{
                composite_filter: Some(CompositeFilter {
                    filters: Some(vec![self.filter, other.filter]),
                    op: Some("OR".to_string())
                }),
                ..google_firestore::Filter::default()
            },
            ..self
        }
    }
}
#[derive(Clone)]
struct FilterDef<'a, T: Into<FieldValue>>(&'a str, FilterOp::<T>);
struct BinaryDef<'a, T: Into<FieldValue>>(&'a str, FilterOp::<T>, T);
struct UnaryDef<'a, T: Into<FieldValue>>(&'a str, FilterOp::<T>);

impl<'a, T: Clone + Into<FieldValue>> Into<google_firestore::Filter> for FilterDef::<'a, T> {
    fn into(self) -> google_firestore::Filter {
        match self.1.clone() {
            FilterOp::<T>::EQUAL(v) |
                FilterOp::<T>::LESS_THAN(v) |
                FilterOp::<T>::LESS_THAN_OR_EQUAL(v) |
                FilterOp::<T>::GREATER_THAN(v) |
                FilterOp::<T>::GREATER_THAN_OR_EQUAL(v) |
                FilterOp::<T>::ARRAY_CONTAINS(v)
                => google_firestore::Filter {
                field_filter: Some(BinaryDef(self.0, self.1, v).into()),
                ..google_firestore::Filter::default()
            },
            FilterOp::<T>::IS_NULL |
            FilterOp::<T>::IS_NAN => google_firestore::Filter {
                unary_filter: Some(UnaryDef(self.0, self.1).into()),
                ..google_firestore::Filter::default()
            }
        }
    }
}
impl<'a, T: Into<FieldValue>> Into<google_firestore::UnaryFilter> for UnaryDef<'a, T> {
    fn into(self) -> google_firestore::UnaryFilter {
        google_firestore::UnaryFilter {
            field: Some(FieldReference {
                field_path: Some(self.0.to_string())
            }),
            op: Some(self.1.to_string()),
        }
    }
}
impl<'a, T: Into<FieldValue>> Into<google_firestore::FieldFilter> for BinaryDef::<'a, T> {
    fn into(self) -> google_firestore::FieldFilter {
        FieldFilter {
            field: Some(FieldReference {
                field_path: Some(self.0.to_string())
            }),
            op: Some(self.1.to_string()),
            value: Some(self.2.into().0),
        }
    }
}
impl<T: Into<FieldValue>> ToString for FilterOp::<T> {
    fn to_string(&self) -> String {
        match self {
            FilterOp::EQUAL(_) => "EQUAL",
            FilterOp::LESS_THAN(_) => "LESS_THAN",
            FilterOp::LESS_THAN_OR_EQUAL(_) => "LESS_THAN_OR_EQUAL",
            FilterOp::GREATER_THAN(_) => "GREATER_THAN",
            FilterOp::GREATER_THAN_OR_EQUAL(_) => "GREATER_THAN_OR_EQUAL",
            FilterOp::ARRAY_CONTAINS(_) => "ARRAY_CONTAINS",
            FilterOp::IS_NULL => "IS_NULL",
            FilterOp::IS_NAN => "IS_NAN",
        }.into()
    }
}
