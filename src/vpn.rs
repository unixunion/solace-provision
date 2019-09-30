use tokio_core::reactor::Core;
use hyper::client::HttpConnector;
use hyper_tls::HttpsConnector;
use solace_semp_client::apis::client::APIClient;
use solace_semp_client::models::MsgVpnResponse;
use solace_semp_client::models::MsgVpn;
use crate::helpers::getselect;
use std::process::exit;
use solace_semp_client::models::MsgVpnQueueResponse;
use crate::provisionable::SolaceProvision;
use std::cell::RefCell;
use std::process;
use solace_semp_client::models::MsgVpnsResponse;
use crate::helpers;


pub struct Vpn<'a> {
    pub vpn_name: String,
    pub file_name: String,
    pub selector: &'a Vec<&str>,
    pub enabled: bool,
    pub ingress: bool,
    pub egress: bool,
    pub count: i32,
}

impl <'a>SolaceProvision<MsgVpnResponse> for Vpn<'a> {
    fn provision(&self, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<MsgVpnResponse, &'static str> {
        info!("{}", format!("Provisioning VPN: {}", &self.vpn_name));
        let file = std::fs::File::open(self.file_name.to_owned()).unwrap();
        let deserialized: Option<MsgVpn> = serde_yaml::from_reader(file).unwrap();
        match deserialized {
            Some(mut item) => {
                item.set_msg_vpn_name(self.vpn_name.to_owned());
                let request = apiclient
                    .default_api()
                    .create_msg_vpn(item, getselect("*"));
                match core.run(request) {
                    Ok(response) => {
                        info!("{}", format!("{}", serde_yaml::to_string(&response.data().unwrap()).unwrap()));
                        Ok(response)
                    },
                    Err(e) => {
                        println!("provision error: {:?}", e);
                        exit(126);
                        Err("provision error")
                    }
                }
            }
            _ => unimplemented!()
        }
    }

    fn update(&self, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<(), &'static str> {

        info!("updating message-vpn: {} from file", self.vpn_name);

        let file = std::fs::File::open(self.file_name).unwrap();

        let deserialized: Option<MsgVpn> = serde_yaml::from_reader(self.file_name).unwrap();

        match deserialized {
            Some(mut item) => {
                item.set_msg_vpn_name(self.vpn_name.to_owned());
                let request = apiclient
                    .default_api()
                    .update_msg_vpn(self.vpn_name.as_ref(), item, getselect("*"));
                match core.run(request) {
                    Ok(response) => {
                        info!("{}", format!("{}", serde_yaml::to_string(&response.data().unwrap()).unwrap()));
                        Ok(())
                    },
                    Err(e) => {
                        error!("update error: {:?}", e);
                        process::exit(126);
                        Err("update error")
                    }
                }
            }
            _ => unimplemented!()
        }
    }

    fn enabled(&self, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<(), &'static str> {

        info!("changing enabled state to: {:?} for message-vpn: {}", self.enabled, self.vpn_name);

        let mut vpn = Vpn::fetch(&self, core, apiclient)?;

        let mut tvpn = vpn.data().unwrap().clone();

        if tvpn.len() == 1 {
            info!("changing enabled state to: {}", self.enabled);
            let mut x = tvpn.pop().unwrap();
            x.set_enabled(self.enabled);
            let r = core.run(apiclient.default_api().update_msg_vpn(self.vpn_name.as_ref(), x, getselect("*")));
            match r {
                Ok(t) => info!("state successfully changed to {:?}", self.enabled),
                Err(e) => {
                    error!("error changing enabled state for vpn: {:?}, {:?}", self.vpn_name, e);
                    exit(126);
                }
            }
        } else {
            error!("error, did not find exactly one item matching query");
            exit(126);
        }

        Ok(())
    }

    fn ingress(&self, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<(), &'static str> {
        unimplemented!()
    }

    fn egress(&self, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<(), &'static str> {
        unimplemented!()
    }

    fn delete(&self, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<(), &'static str> {
        let t = apiclient.default_api().delete_msg_vpn(self.vpn_name.as_ref());
        match core.run(t) {
            Ok(vpn) => {
                info!("vpn deleted");
                Ok(())
            },
            Err(e) => {
                error!("unable to delete vpn: {:?}", e);
                Err("unable to delete vpn")
            }
        }
    }

    fn fetch(&self, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<MsgVpnsResponse, &str> {

        let (wherev, selectv) = helpers::getwhere("msgVpnName", self.vpn_name.as_ref(), "*");

        let request = apiclient
            .msg_vpn_api()
            .get_msg_vpns(count, cursor, wherev, selectv)
            .and_then(|vpn| {
                debug!("{:?}", vpn);
                futures::future::ok(vpn)
            });

        match core.run(request) {
            Ok(response) => {
                let t = format!("{}", serde_yaml::to_string(&response.data().unwrap()).unwrap());
                println!("{}", &t);
                Ok(response)
            },
            Err(e) => {
                error!("error fetching: {:?}", e);
                panic!("fetch error: {:?}", e);
                Err("fetch error")
            }
        }
    }
}

mod tests {
    use tokio_core::reactor::Core;
    use hyper::client::HttpConnector;
    use native_tls::TlsConnector;
    use hyper::Client;
    use crate::helpers;
    use solace_semp_client::apis::configuration::Configuration;
    use solace_semp_client::apis::client::APIClient;
    use crate::vpn::Vpn;
    use crate::vpn::SolaceProvision;

    #[test]
    fn provision() {
        println!("provision tests");

        // configure the http client
        let mut core = Core::new().unwrap();
        let handle = core.handle();

        let mut http = HttpConnector::new(4, &handle);
        http.enforce_http(false);

        let mut tls = TlsConnector::builder().unwrap();

        let hyperclient = Client::configure()
            .connector(hyper_tls::HttpsConnector::from((http, tls.build().unwrap()))).build(&handle);

        let auth = helpers::gencred("admin".to_owned(), "admin".to_owned());

        // the configuration for the APIClient
        let mut configuration = Configuration {
            base_path: "http://localhost:8081/SEMP/v2/config".to_owned(),
            user_agent: Some("solace-provision".to_owned()),
            client: hyperclient,
            basic_auth: Some(auth),
            oauth_access_token: None,
            api_key: None,
        };

        let client = APIClient::new(configuration);

        println!("create vpn");

        let vpn = Vpn {
            vpn_name: "testvpn".to_owned(),
            file_name: "examples/vpn.yaml".to_owned(),
            selector: vec![],
            enabled: false,
            ingress: false,
            egress: false
        };

        Vpn::provision(&vpn, &mut core, &client);
//        vpn::provision(&mut core, &client);

    }


}