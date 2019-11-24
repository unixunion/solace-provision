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
use solace_semp_client::models::{MsgVpnAclProfilesResponse, MsgVpnAclProfileResponse, MsgVpnAclProfile, SempMetaOnlyResponse};
use std::process;

// Fetch ACL
impl Fetch<MsgVpnAclProfilesResponse> for MsgVpnAclProfilesResponse {

    fn fetch(in_vpn: &str, unused_1: &str, select_key: &str, select_value: &str, count: i32, cursor: &str, selector: &str, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<MsgVpnAclProfilesResponse, &'static str> {
        let (wherev, selectv) = helpers::getwhere(select_key, select_value, selector);
        let request = apiclient
            .msg_vpn_api()
            .get_msg_vpn_acl_profiles(in_vpn, count, cursor, wherev, selectv)
            .and_then(|acl| {
                futures::future::ok(acl)
            });

        core_run!(request, core)

    }
}

// provision ACL

impl Provision<MsgVpnAclProfileResponse> for MsgVpnAclProfileResponse {

    fn provision_with_file(msg_vpn_name: &str, item_name: &str, file_name: &str, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<MsgVpnAclProfileResponse, &'static str> {
        let deserialized = deserialize_file_into_type!(file_name, MsgVpnAclProfile);
        match deserialized {
            Some(mut item) => {
//                item.set_msg_vpn_name(in_vpn.to_owned());
                if (&msg_vpn_name != &"") {
                    &item.set_msg_vpn_name(msg_vpn_name.to_owned());
                }
                let request = apiclient
                    .default_api()
                    .create_msg_vpn_acl_profile(msg_vpn_name, item, helpers::getselect("*"));

                core_run!(request, core)

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

    fn update(msg_vpn: &str, file_name: &str, sub_item: &str, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<MsgVpnAclProfileResponse, &'static str> {
        let deserialized = deserialize_file_into_type!(file_name, MsgVpnAclProfile);
        match deserialized {
            Some(mut item) => {
                item.set_msg_vpn_name(msg_vpn.to_owned());
                let item_name = item.acl_profile_name().cloned();
                let request = apiclient
                    .default_api()
                    .update_msg_vpn_acl_profile(msg_vpn, &*item_name.unwrap(), item, helpers::getselect("*"));
                core_run!(request, core)
            }
            _ => unimplemented!()
        }
    }

    fn delete(msg_vpn: &str, item_name: &str, sub_identifier: &str, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<SempMetaOnlyResponse, &'static str> {
        let request = apiclient.default_api().delete_msg_vpn_acl_profile(msg_vpn, item_name);
        core_run_meta!(request, core)
    }
}

mod tests {
    extern crate rand;
    use solace_semp_client::models::{MsgVpnAclProfile, MsgVpnAclProfileResponse, MsgVpnAclProfilesResponse, MsgVpnResponse};
    use crate::provision::Provision;
    use solace_semp_client::models::MsgVpnQueue;
    use tokio_core::reactor::Core;
    use hyper::client::HttpConnector;
    use native_tls::TlsConnector;
    use hyper::Client;
    use crate::helpers;
    use solace_semp_client::apis::configuration::Configuration;
    use solace_semp_client::apis::client::APIClient;
    use std::error::Error;
    use crate::update::Update;
    use crate::fetch::Fetch;
    use crate::save::Save;
    use rand::Rng;

    #[test]
    fn provision() {
        let mut rng = rand::thread_rng();
        let random_vpn = format!("acl_testvpn_{}", rng.gen_range(0, 10));
        println!("acl tests in tmp vpn: {}", &random_vpn);

        let (mut core, mut client) = solace_connect!();

        println!("acl delete testvpn");
        let d = MsgVpnResponse::delete(&random_vpn, "", "", &mut core, &client);

        println!("acl create vpn");
        let v = MsgVpnResponse::provision_with_file(&random_vpn,
                                                    "",
                                                    "test_yaml/acl/vpn.yaml", &mut core,
                                                    &client);
        match v {
            Ok(vpn) => {
                assert_eq!(vpn.data().unwrap().msg_vpn_name().unwrap(), &random_vpn);
            },
            Err(e) => {
                error!("acl cannot create testvpn");
            }
        }

        println!("acl provision");
        let a = MsgVpnAclProfileResponse::provision_with_file(
            &random_vpn,
              "myacl",
              "test_yaml/acl/acl.yaml", &mut core,
              &client
        );

        println!("acl provision verify");
        match a {
            Ok(acl) => {
                assert_eq!(acl.data().unwrap().acl_profile_name().unwrap(), "myacl");
            },
            Err(e) => {
                error!("acl could not be provisioned");
            }
        }

        println!("acl fetch");
        let fa = MsgVpnAclProfilesResponse::fetch(
            &random_vpn,
            "myacl",
            "aclProfileName",
            "myacl",
            10,
            "",
            "*",
            &mut core,
            &client
        );

        println!("acl fetch verify");
        match fa {
            Ok(acls) => {
                assert_eq!(acls.data().unwrap().len(), 1);
            },
            Err(e) => {
                error!("acl fetch failed");
            }
        }

        // save single acl
        println!("acl save");
        let mut acl = MsgVpnAclProfile::new();
        acl.set_acl_profile_name("tmpacl".to_owned());
        acl.set_msg_vpn_name(random_vpn.clone());
        MsgVpnAclProfile::save("tmp", &acl);
        let deserialized = deserialize_file_into_type!(format!("tmp/{}/acl/tmpacl.yaml", random_vpn), MsgVpnAclProfile);
        match deserialized {
            Some(acl) => {
                assert_eq!(acl.acl_profile_name().unwrap(), "tmpacl");
            },
            _ => {
                error!("acl save error");
            }
        }

        // save acls
        println!("acl save response");
        let acls = MsgVpnAclProfilesResponse::fetch(
            &random_vpn,
            "",
            "aclProfileName",
            "*",
            10,
            "",
            "*",
            &mut core,
            &client
        );

        match acls {
            Ok(acls) => {
                MsgVpnAclProfilesResponse::save("tmp", &acls);
                let default_acl = deserialize_file_into_type!(format!("tmp/{}/acl/default.yaml", random_vpn), MsgVpnAclProfile);
                let myacl_acl = deserialize_file_into_type!(format!("tmp/{}/acl/myacl.yaml", random_vpn), MsgVpnAclProfile);
                assert_eq!(default_acl.unwrap().acl_profile_name().unwrap(), "default");
                assert_eq!(myacl_acl.unwrap().acl_profile_name().unwrap(), "myacl");
            },
            Err(e) => {
                error!("save multiple acls failed");
            }
        }



        println!("acl update");
        let updated = MsgVpnAclProfileResponse::update(
            &random_vpn,
            "test_yaml/acl/update.yaml",
            "",
            &mut core,
            &client
        );
        match updated {
            Ok(acl) => {
                assert_eq!(acl.data().unwrap().client_connect_default_action().unwrap(), &"disallow");
            },
            Err(e) => {
                error!("acl update failed");
            }
        }


        println!("acl delete");
        let da = MsgVpnAclProfileResponse::delete(&random_vpn, "myacl", "", &mut core, &client);
        match da {
            Ok(resp) => {
                assert_eq!(resp.meta().response_code(), &200);
            },
            Err(e) => {
                error!("acl delete failed");
            }
        }

        println!("acl delete test vpn");
        let d = MsgVpnResponse::delete(&random_vpn, "", "", &mut core, &client);

    }
}