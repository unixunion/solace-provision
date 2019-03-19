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

mod tests {
    use hyper::{Client, Body};
    use tokio_core::reactor::{Core, Handle};
    use hyper_tls::HttpsConnector;
    use hyper::client::HttpConnector;
    use crate::clientconnection::SPClientConnection;
    use solace_semp_client::apis::client::APIClient;
    use futures::future::Future;
    use solace_semp_client::models::MsgVpnsResponse;
    use crate::fetch::Fetch;

    #[test]
    fn it_works() {
        let mut core = Core::new().unwrap();
        let handle = core.handle();
        let hyperclient = Client::configure()
            .connector(hyper_tls::HttpsConnector::new(4, &handle)
                .unwrap()
            )
            .build(&handle);
        let c = SPClientConnection::new("https://localhost:8080/SEMP/v2/config", "admin", "admin", hyperclient);
        let client = APIClient::new(c.configuration);

        match MsgVpnsResponse::fetch("default", "default", "default", 10, "", "*", &mut core, &client) {
            Ok(v) => {
                assert_eq!(v.meta().response_code(), &200)
            },
            Err(e) => {
                panic!("panic")
            }
        }

    }

}