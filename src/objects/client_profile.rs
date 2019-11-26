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
use solace_semp_client::models::{MsgVpnClientProfilesResponse, MsgVpnClientProfileResponse, MsgVpnClientProfile, SempMetaOnlyResponse};

// fetch Client Profile
impl Fetch<MsgVpnClientProfilesResponse> for MsgVpnClientProfilesResponse {

    fn fetch(in_vpn: &str, name: &str, select_key: &str, select_value: &str, count: i32, cursor: &str, selector: &str, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<MsgVpnClientProfilesResponse, &'static str> {
        let (wherev, selectv) = helpers::getwhere(select_key, select_value, selector);
        let request = apiclient
            .msg_vpn_api()
            .get_msg_vpn_client_profiles(in_vpn, count, cursor, wherev, selectv)
            .and_then(|acl| {
                futures::future::ok(acl)
            });

        core_run!(request, core)

    }
}

// provision client profile
impl Provision<MsgVpnClientProfileResponse> for MsgVpnClientProfileResponse {

    fn provision_with_file(in_vpn: &str, item_name: &str, file_name: &str, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<MsgVpnClientProfileResponse, &'static str> {
        let file = std::fs::File::open(file_name).unwrap();
        let deserialized: Option<MsgVpnClientProfile> = serde_yaml::from_reader(file).unwrap();
        match deserialized {
            Some(mut item) => {
                item.set_msg_vpn_name(in_vpn.to_owned());
                let request = apiclient
                    .default_api()
                    .create_msg_vpn_client_profile(in_vpn, item, helpers::getselect("*"));
                core_run!(request, core)

            }
            _ => unimplemented!()
        }
    }
}

// save Client Profile
impl Save<MsgVpnClientProfile> for MsgVpnClientProfile {
    fn save(dir: &str, data: &MsgVpnClientProfile) -> Result<(), &'static str> where MsgVpnClientProfile: Serialize {
        let vpn_name = data.msg_vpn_name();
        let item_name = data.client_profile_name();
        debug!("save client-profile: {:?}, {:?}", vpn_name, item_name);
        data.save_in_dir(dir, "client-profile", &vpn_name, &item_name);
        Ok(())
    }
}

// save client profiles response
impl Save<MsgVpnClientProfilesResponse> for MsgVpnClientProfilesResponse {
    fn save(dir: &str, data: &MsgVpnClientProfilesResponse) -> Result<(), &'static str> where MsgVpnClientProfilesResponse: Serialize {
        match data.data() {
            Some(items) => {
                for item in items {
                    match MsgVpnClientProfile::save(dir, item) {
                        Ok(t) => debug!("success saving"),
                        Err(e) => error!("error writing: {:?}", e)
                    }

                }
                Ok(())
            },
            _ => {
                error!("no profiles");
                Err("no profiles")
            }
        }
    }
}

// update client-profile
impl Update<MsgVpnClientProfileResponse> for MsgVpnClientProfileResponse {

    fn update(msg_vpn: &str, file_name: &str, sub_item: &str, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<MsgVpnClientProfileResponse, &'static str> {
        let file = std::fs::File::open(file_name).unwrap();
        let deserialized: Option<MsgVpnClientProfile> = serde_yaml::from_reader(file).unwrap();

        match deserialized {
            Some(mut item) => {
                item.set_msg_vpn_name(msg_vpn.to_owned());
                let item_name = item.client_profile_name().cloned();
                let request = apiclient
                    .default_api()
                    .update_msg_vpn_client_profile(msg_vpn, &*item_name.unwrap(), item, helpers::getselect("*"));
                core_run!(request, core)

            }
            _ => unimplemented!()
        }
    }

    fn delete(msg_vpn: &str, item_name: &str, sub_identifier: &str, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<SempMetaOnlyResponse, &'static str> {
        let request = apiclient.default_api().delete_msg_vpn_client_profile(msg_vpn, item_name);
        core_run_meta!(request, core)

    }
}