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
use solace_semp_client::models::{MsgVpnAclProfilesResponse, MsgVpnAclProfileResponse, MsgVpnAclProfile};
use std::process;

// Fetch ACL
impl Fetch<MsgVpnAclProfilesResponse> for MsgVpnAclProfilesResponse {

    fn fetch(in_vpn: &str, name: &str, select_key: &str, select_value: &str, count: i32, cursor: &str, selector: &str, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<MsgVpnAclProfilesResponse, &'static str> {
        let (wherev, selectv) = helpers::getwhere(select_key, select_value, selector);
        let request = apiclient
            .msg_vpn_api()
            .get_msg_vpn_acl_profiles(in_vpn, count, cursor, wherev, selectv)
            .and_then(|acl| {
                futures::future::ok(acl)
            });

        core_run!(request, core)
//        match core.run(request) {
//            Ok(response) => {
//                println!("{}",format!("{}", serde_yaml::to_string(&response.data().unwrap()).unwrap()));
//                Ok(response)
//            },
//            Err(e) => {
//                error!("error fetching: {:?}", e);
//                panic!("fetch error: {:?}", e);
//                Err("fetch error")
//            }
//        }

    }
}

// provision ACL

impl Provision<MsgVpnAclProfileResponse> for MsgVpnAclProfileResponse {

    fn provision_with_file(in_vpn: &str, item_name: &str, file_name: &str, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<MsgVpnAclProfileResponse, &'static str> {
        let file = std::fs::File::open(file_name).unwrap();
        let deserialized: Option<MsgVpnAclProfile> = serde_yaml::from_reader(file).unwrap();
        match deserialized {
            Some(mut item) => {
                item.set_msg_vpn_name(in_vpn.to_owned());
                let request = apiclient
                    .default_api()
                    .create_msg_vpn_acl_profile(in_vpn, item, helpers::getselect("*"));
                core_run!(request, core)
//                match core.run(request) {
//                    Ok(response) => {
//                        info!("{}",format!("{}", serde_yaml::to_string(&response.data().unwrap()).unwrap()));
//                        Ok(response)
//                    },
//                    Err(e) => {
//                        println!("provision error: {:?}", e);
//                        exit(126);
//                        Err("provision error")
//                    }
//                }
            }
            _ => unimplemented!()
        }
    }

}

// save ACL profiles
impl Save<MsgVpnAclProfilesResponse> for MsgVpnAclProfilesResponse {
    fn save(dir: &str, data: &MsgVpnAclProfilesResponse) -> Result<(), &'static str> where MsgVpnAclProfilesResponse: Serialize {
        match data.data() {
            Some(items) => {
                for item in items {
                    match MsgVpnAclProfile::save(dir, item) {
                        Ok(t) => debug!("success saving"),
                        Err(e) => error!("error writing: {:?}", e)
                    }
                }
                Ok(())
            },
            _ => {
                error!("no acls");
                Err("no acl")
            }
        }
    }
}

// save single acl

impl Save<MsgVpnAclProfile> for MsgVpnAclProfile {
    fn save(dir: &str, data: &MsgVpnAclProfile) -> Result<(), &'static str> where MsgVpnAclProfile: Serialize {
        let vpn_name = data.msg_vpn_name();
        let item_name = data.acl_profile_name();
        debug!("save acl: {:?}, {:?}", vpn_name, item_name);
        data.save_in_dir(dir, "acl", &vpn_name, &item_name);
        Ok(())
    }
}

// update ACL profile

impl Update<MsgVpnAclProfileResponse> for MsgVpnAclProfileResponse {

    fn update(msg_vpn: &str, file_name: &str, sub_item: &str, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<(), &'static str> {
        let file = std::fs::File::open(file_name).unwrap();
        let deserialized: Option<MsgVpnAclProfile> = serde_yaml::from_reader(file).unwrap();

        match deserialized {
            Some(mut item) => {
                item.set_msg_vpn_name(msg_vpn.to_owned());
                let item_name = item.acl_profile_name().cloned();
                let request = apiclient
                    .default_api()
                    .update_msg_vpn_acl_profile(msg_vpn, &*item_name.unwrap(), item, helpers::getselect("*"));
//                core_run!(request, core)
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

    fn delete(msg_vpn: &str, item_name: &str, sub_identifier: &str, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<(), &'static str> {
        let t = apiclient.default_api().delete_msg_vpn_acl_profile(msg_vpn, item_name);
        match core.run(t) {
            Ok(vpn) => {
                info!("acl deleted");
                Ok(())
            },
            Err(e) => {
                error!("unable to delete acl: {:?}", e);
                Err("unable to delete acl")
            }
        }
    }
}