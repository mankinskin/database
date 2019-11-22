use crate::access::{Access};
use crate::client::{Client};
use oauth2::{Token, GetToken};
use std::env;
use std::fs;
use std::io::Read;
pub use oauth2::{ServiceAccountAccess, ServiceAccountKey};
use std::error::Error;

pub type FirestoreAccess = Access<ServiceAccountAccess<Client>>;

impl From<ServiceAccountKey> for FirestoreAccess {
    fn from(key: ServiceAccountKey) -> Self {
        let access = ServiceAccountAccess::new(key.clone(), Client::default());
        let project_id = key.project_id.unwrap_or_else(|| {
            panic!("No project_id in service account key!");
        });
        Self {
            project_id,
            access
        }
    }
}

impl<A: GetToken> GetToken for Access<A> {
    fn token<'b, I, T>(&mut self, scopes: I) -> Result<Token, Box<dyn Error>>
    where
        T: AsRef<str> + Ord + 'b,
        I: IntoIterator<Item = &'b T>,
    {
        self.access.token(scopes)
    }

    fn api_key(&mut self) -> Option<String> {
        self.access.api_key()
    }
}

fn read_service_account_key(file: std::fs::File) -> Result<ServiceAccountKey, json::Error> {
    let mut file = file;
    let mut content = String::new();
    file.read_to_string(&mut content).unwrap_or_else(|e| {
        panic!("{}", e);
    });
    json::from_str::<ServiceAccountKey>(&content)
}

pub fn get_service_account_key() -> ServiceAccountKey {
    let path = env::var("GOOGLE_APPLICATION_CREDENTIALS").unwrap_or_else(|e| {
        panic!(
            "Can't read credential json file: ({})\n\
             Please set GOOGLE_APPLICATION_CREDENTIALS \
             environment variable!",
            e
        );
    });
    info!("Reading Service Account Key from '{}'", path);
    let file = fs::File::open(path).unwrap_or_else(|e| {
        panic!("{}", e);
    });
    read_service_account_key(file).unwrap_or_else(|e| {
        panic!("{}", e);
    })
}
