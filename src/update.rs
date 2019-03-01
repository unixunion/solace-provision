
mod tests {

    use solace_semp_client::models::MsgVpn;
    use crate::update::Update;
    use solace_semp_client::models::MsgVpnQueue;

    #[test]
    fn it_works() {}
}



use solace_semp_client::apis::client::APIClient;
use solace_semp_client::models::MsgVpn;
use tokio_core::reactor::Core;
use hyper_tls::HttpsConnector;
use hyper::client::HttpConnector;
use solace_semp_client::models::MsgVpnBridge;
use solace_semp_client::models::MsgVpnQueue;
use solace_semp_client::models::MsgVpnAclProfile;
use serde::Serialize;
use colored::Colorize;
use futures::future::Future;
use futures::future::AndThen;
use std::fs::File;
use serde::Deserialize;
use std::io::Error;
use solace_semp_client::models::MsgVpnResponse;
use crate::fetch;
use solace_semp_client::models::MsgVpnsResponse;
use std::mem::size_of;
use crate::fetch::Fetch;

// shared base trait for all solace provisionable objects
pub trait Update<T> {
    fn update(shutdown: bool, vpn_name: &str, file_name: &str, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<(), &'static str>;
    fn enabled(msg_vpn: &str, item: &str, state: bool, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<(), &'static str>;
}

impl Update<MsgVpnResponse> for MsgVpnResponse {

    fn update(shutdown: bool, vpn_name: &str, file_name: &str, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<(), &'static str> {
        let file = std::fs::File::open(file_name).unwrap();
        let deserialized: Option<MsgVpn> = serde_yaml::from_reader(file).unwrap();

        match deserialized {
            Some(mut item) => {

                if shutdown {
                    MsgVpnResponse::enabled(vpn_name, vpn_name, true, core, apiclient);
                }

                let request = apiclient
                    .default_api()
                    .update_msg_vpn(vpn_name, item, Vec::new());
                match core.run(request) {
                    Ok(response) => {
                        println!("{}",format!("{}", serde_yaml::to_string(&response.data().unwrap()).unwrap()));
                        Ok(())
                    },
                    Err(e) => {
                        println!("update error: {:?}", e);
                        Err("update error")
                    }
                }
            }
            _ => unimplemented!()
        }
    }

    fn enabled(msg_vpn: &str, item: &str, state: bool, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<(), &'static str>{
        println!("retrieving current vpn from appliance");
        let mut vpn = MsgVpnsResponse::fetch(item, item, 10, "", "", core, apiclient)?;
        let mut tvpn = vpn.data().unwrap().clone();

        if tvpn.len() == 1 {

            println!("changing enabled state to: {}", state.to_string());
            let mut x = tvpn.pop().unwrap();
            x.set_enabled(state);

            match core.run(apiclient.default_api().update_msg_vpn(msg_vpn, x, Vec::new())) {
                Ok(respone) => {
                    println!("shutdown");
                    Ok(())
                },
                Err(e) => {
                    println!("Error: {:?}", e);
                    Err("error")
                }
            }
        } else {
            println!("error, found more than one item matching that name");
            Err("found more than one hit, aborting...")
        }


    }
}




