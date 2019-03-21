
mod tests {

    use solace_semp_client::models::MsgVpn;
    use crate::provision::Provision;
    use solace_semp_client::models::MsgVpnQueue;

    #[test]
    fn it_works() {
        // create a new vpn, then test if our new traits and functions are bound

    }
}



use solace_semp_client::apis::client::APIClient;
use solace_semp_client::models::{MsgVpn, MsgVpnQueueSubscription, MsgVpnQueueSubscriptionsResponse, MsgVpnQueueSubscriptionResponse, MsgVpnSequencedTopicResponse, MsgVpnSequencedTopic};
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
use crate::helpers;
use std::io;
use solace_semp_client::models::MsgVpnResponse;
use solace_semp_client::models::MsgVpnQueueResponse;
use solace_semp_client::models::MsgVpnAclProfilesResponse;
use solace_semp_client::models::MsgVpnAclProfileResponse;
use solace_semp_client::models::MsgVpnClientProfileResponse;
use solace_semp_client::models::MsgVpnClientProfile;
use solace_semp_client::models::MsgVpnClientUsernameResponse;
use solace_semp_client::models::MsgVpnClientUsername;
use std::process::exit;
use crate::helpers::getselect;


pub trait Provision<T> {
    fn provision(in_vpn: &str, item_name: &str, file_name: &str, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<T, &'static str>;
}

impl Provision<MsgVpnResponse> for MsgVpnResponse {

    fn provision(in_vpn: &str, item_name: &str, file_name: &str, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<MsgVpnResponse, &'static str> {
        let file = std::fs::File::open(file_name).unwrap();
        let deserialized: Option<MsgVpn> = serde_yaml::from_reader(file).unwrap();
        match deserialized {
            Some(mut item) => {
                item.set_msg_vpn_name(in_vpn.to_owned());
                let request = apiclient
                    .default_api()
                    .create_msg_vpn(item, getselect("*"));
                match core.run(request) {
                    Ok(response) => {
                        info!("{}",format!("{}", serde_yaml::to_string(&response.data().unwrap()).unwrap()));
                        Ok(response)
                    },
                    Err(e) => {
                        error!("provision error: {:?}", e);
                        exit(126);
                        Err("provision error")
                    }
                }
            }
            _ => unimplemented!()
        }
    }
}


impl Provision<MsgVpnQueueResponse> for MsgVpnQueueResponse {

    fn provision(in_vpn: &str, item_name: &str, file_name: &str, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<MsgVpnQueueResponse, &'static str> {
        let file = std::fs::File::open(file_name).unwrap();
        let deserialized: Option<MsgVpnQueue> = serde_yaml::from_reader(file).unwrap();
        match deserialized {
            Some(mut item) => {
                item.set_msg_vpn_name(in_vpn.to_owned());
                let request = apiclient
                    .default_api()
                    .create_msg_vpn_queue(in_vpn, item, getselect("*"));
                match core.run(request) {
                    Ok(response) => {
                        info!("{}",format!("{}", serde_yaml::to_string(&response.data().unwrap()).unwrap()));
                        Ok(response)
                    },
                    Err(e) => {
                        error!("provision error: {:?}", e);
                        exit(126);
                        Err("provision error")
                    }
                }
            }
            _ => unimplemented!()
        }
    }
}

impl Provision<MsgVpnAclProfileResponse> for MsgVpnAclProfileResponse {

    fn provision(in_vpn: &str, item_name: &str, file_name: &str, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<MsgVpnAclProfileResponse, &'static str> {
        let file = std::fs::File::open(file_name).unwrap();
        let deserialized: Option<MsgVpnAclProfile> = serde_yaml::from_reader(file).unwrap();
        match deserialized {
            Some(mut item) => {
                item.set_msg_vpn_name(in_vpn.to_owned());
                let request = apiclient
                    .default_api()
                    .create_msg_vpn_acl_profile(in_vpn, item, getselect("*"));
                match core.run(request) {
                    Ok(response) => {
                        info!("{}",format!("{}", serde_yaml::to_string(&response.data().unwrap()).unwrap()));
                        Ok(response)
                    },
                    Err(e) => {
                        error!("provision error: {:?}", e);
                        exit(126);
                        Err("provision error")
                    }
                }
            }
            _ => unimplemented!()
        }
    }
}

impl Provision<MsgVpnClientProfileResponse> for MsgVpnClientProfileResponse {

    fn provision(in_vpn: &str, item_name: &str, file_name: &str, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<MsgVpnClientProfileResponse, &'static str> {
        let file = std::fs::File::open(file_name).unwrap();
        let deserialized: Option<MsgVpnClientProfile> = serde_yaml::from_reader(file).unwrap();
        match deserialized {
            Some(mut item) => {
                item.set_msg_vpn_name(in_vpn.to_owned());
                let request = apiclient
                    .default_api()
                    .create_msg_vpn_client_profile(in_vpn, item, getselect("*"));
                match core.run(request) {
                    Ok(response) => {
                        info!("{}",format!("{}", serde_yaml::to_string(&response.data().unwrap()).unwrap()));
                        Ok(response)
                    },
                    Err(e) => {
                        error!("provision error: {:?}", e);
                        exit(126);
                        Err("provision error")
                    }
                }
            }
            _ => unimplemented!()
        }
    }
}


impl Provision<MsgVpnClientUsernameResponse> for MsgVpnClientUsernameResponse {

    fn provision(in_vpn: &str, item_name: &str, file_name: &str, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<MsgVpnClientUsernameResponse, &'static str> {
        let file = std::fs::File::open(file_name).unwrap();
        let deserialized: Option<MsgVpnClientUsername> = serde_yaml::from_reader(file).unwrap();
        match deserialized {
            Some(mut item) => {
                item.set_msg_vpn_name(in_vpn.to_owned());
                let request = apiclient
                    .default_api()
                    .create_msg_vpn_client_username(in_vpn, item, getselect("*"));
                match core.run(request) {
                    Ok(response) => {
                        info!("{}",format!("{}", serde_yaml::to_string(&response.data().unwrap()).unwrap()));
                        Ok(response)
                    },
                    Err(e) => {
                        error!("provision error: {:?}", e);
                        exit(126);
                        Err("provision error")
                    }
                }
            }
            _ => unimplemented!()
        }
    }
}



impl Provision<MsgVpnQueueSubscriptionResponse> for MsgVpnQueueSubscriptionResponse {

    fn provision(in_vpn: &str, item_name: &str, file_name: &str, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<MsgVpnQueueSubscriptionResponse, &'static str> {
        let file = std::fs::File::open(file_name).unwrap();
        let deserialized: Option<MsgVpnQueueSubscription> = serde_yaml::from_reader(file).unwrap();
        match deserialized {
            Some(mut item) => {
                item.set_msg_vpn_name(in_vpn.to_owned());
                let request = apiclient
                    .default_api()
                    .create_msg_vpn_queue_subscription(in_vpn, item_name, item, getselect("*"));
                match core.run(request) {
                    Ok(response) => {
                        info!("{}",format!("{}", serde_yaml::to_string(&response.data().unwrap()).unwrap()));
                        Ok(response)
                    },
                    Err(e) => {
                        error!("provision error: {:?}", e);
                        exit(126);
                        Err("provision error")
                    }
                }
            }
            _ => unimplemented!()
        }
    }
}


impl Provision<MsgVpnSequencedTopicResponse> for MsgVpnSequencedTopicResponse {

    fn provision(in_vpn: &str, item_name: &str, file_name: &str, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<MsgVpnSequencedTopicResponse, &'static str> {
        let file = std::fs::File::open(file_name).unwrap();
        let deserialized: Option<MsgVpnSequencedTopic> = serde_yaml::from_reader(file).unwrap();
        match deserialized {
            Some(mut item) => {
                item.set_msg_vpn_name(in_vpn.to_owned());
                let request = apiclient
                    .default_api()
                    .create_msg_vpn_sequenced_topic(in_vpn, item, getselect("*"));
                match core.run(request) {
                    Ok(response) => {
                        info!("{}",format!("{}", serde_yaml::to_string(&response.data().unwrap()).unwrap()));
                        Ok(response)
                    },
                    Err(e) => {
                        error!("provision error: {:?}", e);
                        exit(126);
                        Err("provision error")
                    }
                }
            }
            _ => unimplemented!()
        }
    }
}


