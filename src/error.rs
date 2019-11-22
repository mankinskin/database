pub struct DatabaseError(google_firestore::Error);

unsafe impl Send for DatabaseError {}
unsafe impl Sync for DatabaseError {}

impl From<google_firestore::Error> for DatabaseError {
    fn from(err: google_firestore::Error) -> Self {
        DatabaseError(err)
    }
}
use actix_web::error::BlockingError;
impl<E: Into<DatabaseError> + Debug> From<BlockingError<E>> for DatabaseError {
    fn from(err: BlockingError<E>) -> Self {
        match err {
            BlockingError::Error(e) => e.into(),
            BlockingError::Canceled => google_firestore::Error::Cancelled.into(),
        }
    }
}
use std::fmt::{Debug, Display, Formatter, self};
impl Debug for DatabaseError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "DatabaseError({})", self.0)
    }
}
impl Display for DatabaseError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "DatabaseError: {}", self.0)
    }
}
