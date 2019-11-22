use hyper::net::{HttpsConnector};
use hyper_rustls::{TlsClient};

pub struct Client(pub hyper::Client);

impl std::borrow::Borrow<hyper::Client> for Client {
    fn borrow(&self) -> &hyper::Client {
        &self.0
    }
}
impl std::borrow::BorrowMut<hyper::Client> for Client {
    fn borrow_mut(&mut self) -> &mut hyper::Client {
        &mut self.0
    }
}

impl Default for Client {
    fn default() -> Self {
        Self(
            hyper::Client::with_connector(HttpsConnector::new(TlsClient::new()))
        )
    }
}

impl Into<hyper::Client> for Client {
    fn into(self) -> hyper::Client {
        self.0
    }
}
