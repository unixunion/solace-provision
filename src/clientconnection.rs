use hyper::{Client, Body};
use tokio_core::reactor::{Core, Handle};
use hyper_tls::HttpsConnector;
use hyper::client::HttpConnector;
use solace_semp_client::apis::configuration::Configuration;
use crate::helpers;

trait SPClient<C: hyper::client::Connect> {
    fn new(client: Client<HttpsConnector<HttpConnector>, Body>) -> SPClientConnection<C>;
    fn connect(&self);
}

pub struct SPClientConnection<C: hyper::client::Connect> {
    pub configuration: Configuration<C>,
}

impl <C: hyper::client::Connect> SPClientConnection<C> {
    pub fn new(host: &str, username: &str, password: &str, client: hyper::client::Client<C>) -> SPClientConnection<C> {
        SPClientConnection {
            configuration: Configuration {
                base_path: host.to_owned(),
                user_agent: Some("solace-provision".to_owned()),
                client: client,
                basic_auth: Some(helpers::gencred(username.to_owned(), password.to_owned())),
                oauth_access_token: None,
                api_key: None,
            }
        }
    }
}
