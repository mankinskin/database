pub mod fields;

use crate::logger::indent_lines;
pub use fields::*;
use google_firestore;
use std::collections::HashMap;

#[derive(Clone)]
pub struct Document {
    id: String, // includes path in database
    fields: HashMap<String, FieldValue>,
    create_time: Option<String>, // should be special types,
    update_time: Option<String>, // convertable to string
}
unsafe impl Send for Document {}

// ignoring create/update_time in eq comparision for easier testing
impl std::cmp::PartialEq for Document {
    fn eq(&self, other: &Document) -> bool {
        self.id == other.id &&
        self.fields == other.fields
    }
}

impl Document {
    pub fn builder() -> DocumentBuilder {
        DocumentBuilder::default()
    }
    // only the name of the document itself
    pub fn name(&self) -> &str {
        self.id.rsplit("/").next().unwrap_or("")
    }
    // the id of the document (with path)
    pub fn id(&self) -> &str {
        &self.id
    }
    pub fn fields(&self) -> &HashMap<String, FieldValue> {
        &self.fields
    }
    pub fn get(&self, key: &str) -> Result<&FieldValue, String> {
        self.fields
            .get(key)
            .ok_or(format!(
                    "Document {} does not have field \"{}\"",
                    self.id,
                    key))
    }
    pub fn create_time(&self) -> Option<String> {
        self.create_time.clone()
    }
    pub fn update_time(&self) -> Option<String> {
        self.update_time.clone()
    }
}

impl std::convert::From<google_firestore::Document> for Document {
    fn from(document: google_firestore::Document) -> Self {
        Document {
            id: document.name.unwrap_or("".to_string()),
            fields: document.fields
                .unwrap_or(HashMap::new())
                .iter()
                .map(|(k,v)| (k.clone(), FieldValue::from(v.clone())))
                .collect(),
            create_time: document.create_time,
            update_time: document.update_time,
        }
    }
}
impl std::convert::From<Document> for google_firestore::Document {
    fn from(document: Document) -> Self {
        google_firestore::Document {
            name: Some(document.id),
            fields: Some(document.fields
                             .iter()
                             .map(|(k, v)|
                                  (k.clone(), v.clone().into()))
                             .collect()),
            create_time: document.create_time,
            update_time: document.update_time,
        }
    }
}
impl std::default::Default for Document {
    fn default() -> Self {
        Document {
            id: String::new(),
            fields: HashMap::new(),
            create_time: None,
            update_time: None,
        }
    }
}

impl std::fmt::Debug for Document {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let indent_level = 2;
        let id = self.id.clone();
        let fields = indent_lines(
            &format!("{:?}", self.fields()),
            indent_level,
        );
        let contents = format!(
            "Name: '{}'\n{}{}Fields:\n{}",
            id,
            self.create_time.clone()
                .map(|s| format!("Created: {}\n", s))
                .unwrap_or("".to_string()),
            self.update_time.clone()
                .map(|s| format!("Updated: {}\n", s))
                .unwrap_or("".to_string()),
            fields
        );
        write!(
            f,
            "Document {{\n{}}}",
            indent_lines(&contents, indent_level)
        )
    }
}

impl std::fmt::Display for Document {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub struct DocumentBuilder(Document);

impl Default for DocumentBuilder {
    fn default() -> Self {
        DocumentBuilder(Document::default())
    }
}

impl DocumentBuilder {
    pub fn name(self, name: &str) -> Self {
            DocumentBuilder(
                Document {
                    id: name.to_string(),
                    ..self.0
                })
    }
    pub fn field<T: Into<FieldValue>>(mut self, key: &str, value: T) -> Self {
        self.0.fields.insert(key.to_string(), value.into());
        DocumentBuilder(Document {
            fields: self.0.fields,
            ..self.0
        })
    }
    pub fn build(self) -> Document {
        //let current_time: String = chrono::offset::Utc::now().to_string();
        //Document {
        //    create_time: Some(current_time.clone()),
        //    update_time: Some(current_time),
        //    ..self.0
        //}
        self.0
    }
}

#[cfg(test)]
pub mod tests {
    use super::{
        Document,
    };
    pub fn test_document<T: ToString>(id: T) -> Document {
        Document::builder()
            .name(&id.to_string())
            .field("test_string", "TestString")
            .field("test_number", 42)
            .build()
    }
}
