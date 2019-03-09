
mod tests {

    use solace_semp_client::models::MsgVpn;
    use crate::update::Update;
    use solace_semp_client::models::MsgVpnQueue;

    #[test]
    fn it_works() {}
}


use std::process;
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
use log::{info, warn, error, debug};
use serde::Deserialize;
use std::io::Error;
use solace_semp_client::models::MsgVpnResponse;
use crate::fetch;
use solace_semp_client::models::MsgVpnsResponse;
use std::mem::size_of;
use crate::fetch::Fetch;
use std::process::exit;
use solace_semp_client::models::MsgVpnQueueResponse;
use solace_semp_client::models::MsgVpnQueuesResponse;
use solace_semp_client::models::MsgVpnAclProfileResponse;
use solace_semp_client::models::MsgVpnAclProfilesResponse;
use solace_semp_client::models::MsgVpnClientProfileResponse;
use solace_semp_client::models::MsgVpnClientProfile;
use solace_semp_client::models::MsgVpnClientProfilesResponse;
use solace_semp_client::models::MsgVpnClientUsernameResponse;
use solace_semp_client::models::MsgVpnClientUsername;
use solace_semp_client::models::MsgVpnClientUsernamesResponse;
use crate::helpers::getselect;

// shared base trait for all solace update-able objects
pub trait Update<T> {
    // update a object, shutting it down first if shutdown is true
    fn update(vpn_name: &str, file_name: &str, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<(), &'static str>;
    // change the enabled state fo a object
    fn enabled(msg_vpn: &str, item_name: &str, state: bool, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<(), &'static str>;
    // delete vpn
    fn delete(msg_vpn: &str, item_name: &str, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<(), &'static str>;
}

impl Update<MsgVpnResponse> for MsgVpnResponse {

    fn update(vpn_name: &str, file_name: &str, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<(), &'static str> {
        info!("updating message-vpn: {} from file", vpn_name);
        let file = std::fs::File::open(file_name).unwrap();
        let deserialized: Option<MsgVpn> = serde_yaml::from_reader(file).unwrap();

        match deserialized {
            Some(mut item) => {
                item.set_msg_vpn_name(vpn_name.to_owned());
                let request = apiclient
                    .default_api()
                    .update_msg_vpn(vpn_name, item, getselect("*"));
                match core.run(request) {
                    Ok(response) => {
                        info!("{}",format!("{}", serde_yaml::to_string(&response.data().unwrap()).unwrap()));
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

    fn enabled(msg_vpn: &str, item_name: &str, state: bool, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<(), &'static str> {
        info!("changing enabled state to: {:?} for message-vpn: {}", state, msg_vpn);
        let mut vpn = MsgVpnsResponse::fetch(item_name, item_name, 10, "", "", core, apiclient)?;

        let mut tvpn = vpn.data().unwrap().clone();
        if tvpn.len() == 1 {
            info!("changing enabled state to: {}", state.to_string());
            let mut x = tvpn.pop().unwrap();
            x.set_enabled(state);
            let r = core.run(apiclient.default_api().update_msg_vpn(msg_vpn, x, getselect("*")));
            match r {
                Ok(t) => info!("state successfully changed to {:?}", state),
                Err(e) => {
                    error!("error changing enabled state for vpn: {}, {:?}", item_name, e);
                    exit(126);
                }
            }
        } else {
            error!("error, did not find exactly one item matching query");
            exit(126);
        }


        Ok(())

    }

    fn delete(msg_vpn: &str, item_name: &str, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<(), &'static str> {
        let t = apiclient.default_api().delete_msg_vpn(item_name);
        match core.run(t) {
            Ok(vpn) => {
                info!("vpn deleted");
                Ok(())
            },
            Err(e) => {
                error!("unable to delete vpn: {:?}", e);
                process::exit(126);
                Err("unable to delete vpn")
            }
        }
    }
}

impl Update<MsgVpnQueueResponse> for MsgVpnQueueResponse {

    fn update(vpn_name: &str, file_name: &str, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<(), &'static str> {
        let file = std::fs::File::open(file_name).unwrap();
        let deserialized: Option<MsgVpnQueue> = serde_yaml::from_reader(file).unwrap();

        match deserialized {
            Some(mut item) => {
                item.set_msg_vpn_name(vpn_name.to_owned());
                let item_name = item.queue_name().cloned();
                let request = apiclient
                    .default_api()
                    .update_msg_vpn_queue(vpn_name, &*item_name.unwrap(), item, getselect("*"));
                match core.run(request) {
                    Ok(response) => {
                        info!("{}",format!("{}", serde_yaml::to_string(&response.data().unwrap()).unwrap()));
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

    fn enabled(vpn_name: &str, item_name: &str, state: bool, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<(), &'static str> {
        info!("retrieving current queue from appliance");
        let mut item = MsgVpnQueuesResponse::fetch(vpn_name, item_name, 10, "", "", core, apiclient)?;
        let mut titem = item.data().unwrap().clone();

        if titem.len() == 1 {
            info!("changing enabled state to: {}", state.to_string());
            let mut x = titem.pop().unwrap();
            x.set_ingress_enabled(state);
            x.set_egress_enabled(state);
            let r = core.run(apiclient.default_api().update_msg_vpn_queue(vpn_name, item_name, x, getselect("*")));
            match r {
                Ok(t) => info!("state successfully changed to {:?}", state),
                Err(e) => {
                    error!("error changing enabled state for vpn: {}, {:?}", item_name, e);
                    exit(126);
                }
            }

        } else {
            error!("error, did not find exactly one item matching query");
            process::exit(126);
        }

        Ok(())

    }

    fn delete(vpn_name: &str, item_name: &str, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<(), &'static str> {
        let t = apiclient.default_api().delete_msg_vpn_queue(vpn_name, item_name);
        match core.run(t) {
            Ok(vpn) => {
                info!("queue deleted");
                Ok(())
            },
            Err(e) => {
                error!("unable to delete queue: {:?}", e);
                process::exit(126);
                Err("unable to delete queue")
            }
        }
    }
}

impl Update<MsgVpnAclProfileResponse> for MsgVpnAclProfileResponse {

    fn update(vpn_name: &str, file_name: &str, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<(), &'static str> {
        let file = std::fs::File::open(file_name).unwrap();
        let deserialized: Option<MsgVpnAclProfile> = serde_yaml::from_reader(file).unwrap();

        match deserialized {
            Some(mut item) => {
                item.set_msg_vpn_name(vpn_name.to_owned());
                let item_name = item.acl_profile_name().cloned();
                let request = apiclient
                    .default_api()
                    .update_msg_vpn_acl_profile(vpn_name, &*item_name.unwrap(), item, getselect("*"));
                match core.run(request) {
                    Ok(response) => {
                        info!("{}",format!("{}", serde_yaml::to_string(&response.data().unwrap()).unwrap()));
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

    fn enabled(msg_vpn: &str, item_name: &str, state: bool, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<(), &'static str> {
        unimplemented!()
    }

    fn delete(vpn_name: &str, item_name: &str, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<(), &'static str> {
        let t = apiclient.default_api().delete_msg_vpn_acl_profile(vpn_name, item_name);
        match core.run(t) {
            Ok(vpn) => {
                info!("acl deleted");
                Ok(())
            },
            Err(e) => {
                error!("unable to delete acl: {:?}", e);
                process::exit(126);
                Err("unable to delete acl")
            }
        }
    }
}




impl Update<MsgVpnClientProfileResponse> for MsgVpnClientProfileResponse {

    fn update(vpn_name: &str, file_name: &str, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<(), &'static str> {
        let file = std::fs::File::open(file_name).unwrap();
        let deserialized: Option<MsgVpnClientProfile> = serde_yaml::from_reader(file).unwrap();

        match deserialized {
            Some(mut item) => {
                item.set_msg_vpn_name(vpn_name.to_owned());
                let item_name = item.client_profile_name().cloned();
                let request = apiclient
                    .default_api()
                    .update_msg_vpn_client_profile(vpn_name, &*item_name.unwrap(), item, getselect("*"));
                match core.run(request) {
                    Ok(response) => {
                        info!("{}",format!("{}", serde_yaml::to_string(&response.data().unwrap()).unwrap()));
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

    fn enabled(msg_vpn: &str, item_name: &str, state: bool, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<(), &'static str> {
        unimplemented!()
    }

    fn delete(vpn_name: &str, item_name: &str, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<(), &'static str> {
        let t = apiclient.default_api().delete_msg_vpn_client_profile(vpn_name, item_name);
        match core.run(t) {
            Ok(vpn) => {
                info!("client-profile deleted");
                Ok(())
            },
            Err(e) => {
                error!("unable to delete client-profile: {:?}", e);
                process::exit(126);
                Err("unable to delete client-profile")
            }
        }
    }
}



impl Update<MsgVpnClientUsernameResponse> for MsgVpnClientUsernameResponse {

    fn update(vpn_name: &str, file_name: &str, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<(), &'static str> {
        let file = std::fs::File::open(file_name).unwrap();
        let deserialized: Option<MsgVpnClientUsername> = serde_yaml::from_reader(file).unwrap();

        match deserialized {
            Some(mut item) => {
                item.set_msg_vpn_name(vpn_name.to_owned());
                let item_name = item.client_username().cloned();
                let request = apiclient
                    .default_api()
                    .update_msg_vpn_client_username(vpn_name, &*item_name.unwrap(), item, getselect("*"));
                match core.run(request) {
                    Ok(response) => {
                        info!("{}",format!("{}", serde_yaml::to_string(&response.data().unwrap()).unwrap()));
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

    fn enabled(vpn_name: &str, item_name: &str, state: bool, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<(), &'static str> {
        println!("retrieving current client-username from appliance");
        let mut item = MsgVpnClientUsernamesResponse::fetch(vpn_name, item_name, 10, "", "", core, apiclient)?;
        let mut titem = item.data().unwrap().clone();

        if titem.len() == 1 {
            println!("changing enabled state to: {}", state.to_string());
            let mut x = titem.pop().unwrap();
            x.set_enabled(state);
            x.reset_password();
            let r = core.run(apiclient.default_api().update_msg_vpn_client_username(vpn_name, item_name, x, getselect("*")));
            match r {
                Ok(t) => info!("state successfully changed to {:?}", state),
                Err(e) => {
                    error!("error changing enabled state for client-username: {}, {:?}", item_name, e);
                    exit(126);
                }
            }

        } else {
            error!("error, did not find exactly one item matching query");
            process::exit(126);
        }

        Ok(())

    }

    fn delete(vpn_name: &str, item_name: &str, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<(), &'static str> {
        let t = apiclient.default_api().delete_msg_vpn_client_username(vpn_name, item_name);
        match core.run(t) {
            Ok(vpn) => {
                info!("client-username deleted");
                Ok(())
            },
            Err(e) => {
                error!("unable to delete client-username: {:?}", e);
                process::exit(126);
                Err("unable to delete client-username")
            }
        }
    }
}


