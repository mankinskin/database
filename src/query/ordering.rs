#[derive(Debug)]
pub enum Ordering {
    ASCENDING,
    DESCENDING,
}

impl ToString for Ordering {
    fn to_string(&self) -> String {
        format!("{:?}", self)
    }
}

