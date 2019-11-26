use crate::fetch::Fetch;
use crate::helpers;
use tokio_core::reactor::Core;
use solace_semp_client::apis::client::APIClient;
use futures::future::Future;
use hyper_tls::HttpsConnector;
use hyper::client::HttpConnector;
use crate::provision::Provision;
use std::process::exit;
use crate::save::Save;
use serde::Serialize;
use crate::update::Update;
use std::process;
use std::borrow::ToOwned;
use solace_semp_client::models::{MsgVpnClientUsernameResponse, MsgVpnClientUsername, MsgVpnClientUsernamesResponse, SempMetaOnlyResponse};

// fetch for client-username
impl Fetch<MsgVpnClientUsernamesResponse> for MsgVpnClientUsernamesResponse {

    fn fetch(in_vpn: &str, name: &str, select_key: &str, select_value: &str, count: i32, cursor: &str, selector: &str, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<MsgVpnClientUsernamesResponse, &'static str> {
        let (wherev, selectv) = helpers::getwhere(select_key, select_value, selector);
        let request = apiclient
            .msg_vpn_api()
            .get_msg_vpn_client_usernames(in_vpn, count, cursor, wherev, selectv)
            .and_then(|cu| {
                debug!("{:?}", cu);
                futures::future::ok(cu)
            });

        core_run!(request, core)

    }
}

// provision client-username
impl Provision<MsgVpnClientUsernameResponse> for MsgVpnClientUsernameResponse {

    fn provision_with_file(in_vpn: &str, item_name: &str, file_name: &str, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<MsgVpnClientUsernameResponse, &'static str> {
        let file = std::fs::File::open(file_name).unwrap();
        let deserialized: Option<MsgVpnClientUsername> = serde_yaml::from_reader(file).unwrap();
        match deserialized {
            Some(mut item) => {
                item.set_msg_vpn_name(in_vpn.to_owned());
                let request = apiclient
                    .default_api()
                    .create_msg_vpn_client_username(in_vpn, item, helpers::getselect("*"));
                core_run!(request, core)

            }
            _ => unimplemented!()
        }
    }
}

// save client usernames response
impl Save<MsgVpnClientUsernamesResponse> for MsgVpnClientUsernamesResponse {
    fn save(dir: &str, data: &MsgVpnClientUsernamesResponse) -> Result<(), &'static str> where MsgVpnClientUsernamesResponse: Serialize {
        match data.data() {
            Some(items) => {
                for item in items {
                    match MsgVpnClientUsername::save(dir, item) {
                        Ok(t) => debug!("success saving"),
                        Err(e) => error!("error writing: {:?}", e)
                    }

                }
                Ok(())
            },
            _ => {
                error!("no users");
                Err("no users")
            }
        }
    }
}

// save client-username
impl Save<MsgVpnClientUsername> for MsgVpnClientUsername {
    fn save(dir: &str, data: &MsgVpnClientUsername) -> Result<(), &'static str> where MsgVpnClientUsername: Serialize {
        let vpn_name = data.msg_vpn_name();
        let item_name = data.client_username();
        debug!("save client-username: {:?}, {:?}", vpn_name, item_name);
        data.save_in_dir(dir, "client-username", &vpn_name, &item_name);
        Ok(())
    }
}

// update client-username
impl Update<MsgVpnClientUsernameResponse> for MsgVpnClientUsernameResponse {

    fn update(msg_vpn: &str, file_name: &str, sub_item: &str, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<MsgVpnClientUsernameResponse, &'static str> {
        let file = std::fs::File::open(file_name).unwrap();
        let deserialized: Option<MsgVpnClientUsername> = serde_yaml::from_reader(file).unwrap();

        match deserialized {
            Some(mut item) => {
                item.set_msg_vpn_name(msg_vpn.to_owned());
                let item_name = item.client_username().cloned();
                let request = apiclient
                    .default_api()
                    .update_msg_vpn_client_username(msg_vpn, &*item_name.unwrap(), item, helpers::getselect("*"));
                core_run!(request, core)

            }
            _ => unimplemented!()
        }
    }

    fn enabled(msg_vpn: &str, client_username: &str, selector: Vec<&str>, state: bool, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<MsgVpnClientUsernameResponse, &'static str> {
        println!("retrieving current client-username from appliance");
        let mut item = MsgVpnClientUsernamesResponse::fetch(msg_vpn, client_username, "clientUsername",client_username, 10, "", "", core, apiclient)?;
        let mut titem = item.data().unwrap().clone();

        if titem.len() == 1 {
            println!("changing enabled state to: {}", state.to_string());
            let mut x = titem.pop().unwrap();
//            let client_username = x.client_username().clone().unwrap();
            x.set_enabled(state);
            // this sets the password to None, so its not sent back to the appliance
            x.reset_password();
            let request = apiclient
                .default_api()
                .update_msg_vpn_client_username(msg_vpn, client_username, x, helpers::getselect("*"));

            core_run!(request, core)

        } else {
            error!("error, did not find exactly one item matching query");
            process::exit(126);
        }

//        Ok(())
    }

    fn delete(msg_vpn: &str, item_name: &str, sub_identifier: &str, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<SempMetaOnlyResponse, &'static str> {
        let request = apiclient
            .default_api()
            .delete_msg_vpn_client_username(msg_vpn, item_name);
        core_run_meta!(request, core)

    }
}
