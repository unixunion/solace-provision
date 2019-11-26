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
use solace_semp_client::models::{MsgVpnAuthorizationGroupsResponse, MsgVpnAuthorizationGroupResponse, MsgVpnAuthorizationGroup, SempMetaOnlyResponse};

// authorization groups
impl Fetch<MsgVpnAuthorizationGroupsResponse> for MsgVpnAuthorizationGroupsResponse {
    fn fetch(in_vpn: &str, sub_item: &str, select_key: &str, select_value: &str, count: i32, cursor: &str, selector: &str, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<MsgVpnAuthorizationGroupsResponse, &'static str> {
        let (wherev, mut selectv) = helpers::getwhere(select_key, select_value, selector);

        let request = apiclient
            .authorization_group_api()
            .get_msg_vpn_authorization_groups(in_vpn, count, cursor, wherev, selectv)
            .and_then(|item| {
                futures::future::ok(item)
            });
        core_run!(request, core)
    }
}


// authorization group

impl Provision<MsgVpnAuthorizationGroupResponse> for MsgVpnAuthorizationGroupResponse {

    fn provision_with_file(in_vpn: &str, item_name: &str, file_name: &str, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<MsgVpnAuthorizationGroupResponse, &'static str> {
        let file = std::fs::File::open(file_name).unwrap();
        let deserialized: Option<MsgVpnAuthorizationGroup> = serde_yaml::from_reader(file).unwrap();
        match deserialized {
            Some(mut item) => {
                item.set_msg_vpn_name(in_vpn.to_owned());
                let request = apiclient
                    .default_api()
                    .create_msg_vpn_authorization_group(in_vpn, item, helpers::getselect("*"));
                core_run!(request, core)
            }
            _ => unimplemented!()
        }
    }

}

// authorization group

impl Save<MsgVpnAuthorizationGroup> for MsgVpnAuthorizationGroup {
    fn save(dir: &str, data: &MsgVpnAuthorizationGroup) -> Result<(), &'static str> where MsgVpnAuthorizationGroup: Serialize {
        let vpn_name = data.msg_vpn_name();
        let item_name = data.authorization_group_name();

        debug!("save authorization-group: {:?}, {:?}", vpn_name, item_name);
        data.save_in_dir(dir, "authorization-group", &vpn_name, &item_name);
        Ok(())
    }
}


impl Save<MsgVpnAuthorizationGroupsResponse> for MsgVpnAuthorizationGroupsResponse {
    fn save(dir: &str, data: &MsgVpnAuthorizationGroupsResponse) -> Result<(), &'static str> where MsgVpnAuthorizationGroupsResponse: Serialize {
        match data.data() {
            Some(items) => {
                for item in items {
                    match MsgVpnAuthorizationGroup::save(dir, item) {
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

// authorization group

impl Update<MsgVpnAuthorizationGroupResponse> for MsgVpnAuthorizationGroupResponse {

    fn update(msg_vpn: &str, file_name: &str, sub_item: &str, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<MsgVpnAuthorizationGroupResponse, &'static str> {

        let file = std::fs::File::open(file_name).unwrap();
        let deserialized: Option<MsgVpnAuthorizationGroup> = serde_yaml::from_reader(file).unwrap();

        match deserialized {
            Some(mut item) => {
                item.set_msg_vpn_name(msg_vpn.to_owned());
                let item_name = item.authorization_group_name().cloned();
                let request = apiclient
                    .default_api()
                    .update_msg_vpn_authorization_group(msg_vpn, &*item_name.unwrap(), item, helpers::getselect("*"));
                core_run!(request, core)
            }
            _ => unimplemented!()
        }
    }

    fn enabled(msg_vpn: &str, item_name: &str, selector: Vec<&str>, state: bool, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<MsgVpnAuthorizationGroupResponse, &'static str> {
        println!("retrieving current authorization-group from appliance");
        let mut item = MsgVpnAuthorizationGroupsResponse::fetch(msg_vpn, item_name, "authorizationGroupName",item_name, 10, "", "", core, apiclient)?;
        let mut titem = item.data().unwrap().clone();

        if titem.len() == 1 {
            println!("changing enabled state to: {}", state.to_string());
            let mut x = titem.pop().unwrap();
            x.set_enabled(state);
            let request = apiclient.default_api().update_msg_vpn_authorization_group(msg_vpn, item_name, x, helpers::getselect("*"));
            core_run!(request, core)

        } else {
            error!("error, did not find exactly one item matching query");
            Err("unable to change enabled state")
        }

    }

    fn delete(msg_vpn: &str, item_name: &str, sub_identifier: &str, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<SempMetaOnlyResponse, &'static str> {
        let request = apiclient.default_api().delete_msg_vpn_authorization_group(msg_vpn, item_name);
        core_run_meta!(request, core)
    }

}