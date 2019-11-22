use crate::document::{
    FieldValue,
};

// Filters are objects which represent simple
// predicate functions to be used in a query
pub trait Filter<T: Into<FieldValue> = ()> {
    fn filter(self, field: &str, op: FilterOp<T>) -> Self;
    fn and(self, other: Self) -> Self;
    fn or(self, other: Self) -> Self;
}
#[allow(non_camel_case_types)]
#[derive(Clone, Debug)]
pub enum FilterOp<T: Into<FieldValue> = ()> {
    EQUAL(T),
    LESS_THAN(T),
    LESS_THAN_OR_EQUAL(T),
    GREATER_THAN(T),
    GREATER_THAN_OR_EQUAL(T),
    ARRAY_CONTAINS(T),
    IS_NULL,
    IS_NAN,
}
