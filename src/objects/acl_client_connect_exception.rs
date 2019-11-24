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
use solace_semp_client::models::{MsgVpnAclProfileClientConnectExceptionsResponse, MsgVpnAclProfileClientConnectExceptionResponse, MsgVpnAclProfileClientConnectException};


impl Fetch<MsgVpnAclProfileClientConnectExceptionsResponse> for MsgVpnAclProfileClientConnectExceptionsResponse {

    fn fetch(msg_vpn_name: &str, acl_profile_name: &str, select_key: &str, select_value: &str, count: i32, cursor: &str, selector: &str, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<MsgVpnAclProfileClientConnectExceptionsResponse, &'static str> {
        let (wherev, selectv) = helpers::getwhere(select_key, select_value, selector);
        let request = apiclient
            .default_api()
            .get_msg_vpn_acl_profile_client_connect_exceptions(msg_vpn_name, acl_profile_name, count, cursor, wherev, selectv)
            .and_then(|acl| {
                futures::future::ok(acl)
            });

        core_run!(request, core)
    }
}

impl Provision<MsgVpnAclProfileClientConnectExceptionResponse> for MsgVpnAclProfileClientConnectExceptionResponse {

    fn provision_with_file(msg_vpn_name: &str, acl_profile_name: &str, file_name: &str, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<MsgVpnAclProfileClientConnectExceptionResponse, &'static str> {
        let deserialized = deserialize_file_into_type!(file_name, MsgVpnAclProfileClientConnectException);
        match deserialized {
            Some(mut body) => {
                if (&msg_vpn_name != &"") {
                    &body.set_msg_vpn_name(msg_vpn_name.to_owned());
                }
                let request = apiclient
                    .default_api()
                    .create_msg_vpn_acl_profile_client_connect_exception(
                        msg_vpn_name,
                        acl_profile_name,
                        body,
                        helpers::getselect("*")
                    );

                core_run!(request, core)

            }
            _ => unimplemented!()
        }
    }

}

mod test {
    extern crate rand;
    use crate::provision::Provision;
    use solace_semp_client::models::{MsgVpnQueue, MsgVpnResponse, MsgVpnAclProfileClientConnectExceptionResponse, MsgVpnAclProfileClientConnectException, MsgVpnAclProfileClientConnectExceptionsResponse, MsgVpnAclProfileResponse};
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
        let random_vpn = format!("acce_testvpn_{}", rng.gen_range(0, 10));
        println!("acce tests in tmp vpn: {}", &random_vpn);

        let (mut core, mut client) = solace_connect!();

        println!("acce delete testvpn");
        let d = MsgVpnResponse::delete(&random_vpn, "", "", &mut core, &client);

        println!("acce create vpn");
        let v = MsgVpnResponse::provision_with_file(&random_vpn,
                                                    "",
                                                    "test_yaml/acce/vpn.yaml", &mut core,
                                                    &client);
        match v {
            Ok(vpn) => {
                assert_eq!(vpn.data().unwrap().msg_vpn_name().unwrap(), &random_vpn);
            },
            Err(e) => {
                error!("acl cannot create testvpn");
            }
        }

        println!("acce provision acl");
        let a = MsgVpnAclProfileResponse::provision_with_file(
            &random_vpn,
            "myacl",
            "test_yaml/acce/acl.yaml", &mut core,
            &client
        );

        println!("acce provision verify acl");
        match a {
            Ok(acl) => {
                assert_eq!(acl.data().unwrap().acl_profile_name().unwrap(), "myacl");
            },
            Err(e) => {
                error!("acl could not be provisioned");
            }
        }

        println!("acce provision");
        let a = MsgVpnAclProfileClientConnectExceptionResponse::provision_with_file(
            &random_vpn,
            "myacl",
            "test_yaml/acce/acce.yaml", &mut core,
            &client
        );

        println!("acce provision verify");
        match a {
            Ok(acce) => {
                assert_eq!(acce.data().unwrap().acl_profile_name().unwrap(), "myacl");
            }
            Err(e) => {
                error!("acce could not be provisioned");
            }
        }

        println!("acce fetch");
        let fa = MsgVpnAclProfileClientConnectExceptionsResponse::fetch(
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

        println!("acce fetch verify");
        match fa {
            Ok(acls) => {
                assert_eq!(acls.data().unwrap().len(), 1);
            },
            Err(e) => {
                error!("acce fetch failed");
            }
        }

        // save single acl
//        println!("acl save");
//        let mut acl = MsgVpnAclProfile::new();
//        acl.set_acl_profile_name("tmpacl".to_owned());
//        acl.set_msg_vpn_name(random_vpn.clone());
//        MsgVpnAclProfile::save("tmp", &acl);
//        let deserialized = deserialize_file_into_type!(format!("tmp/{}/acl/tmpacl.yaml", random_vpn), MsgVpnAclProfile);
//        match deserialized {
//            Some(acl) => {
//                assert_eq!(acl.acl_profile_name().unwrap(), "tmpacl");
//            },
//            _ => {
//                error!("acl save error");
//            }
//        }
//
//        // save acls
//        println!("acl save response");
//        let acls = MsgVpnAclProfilesResponse::fetch(
//            &random_vpn,
//            "",
//            "aclProfileName",
//            "*",
//            10,
//            "",
//            "*",
//            &mut core,
//            &client
//        );
//
//        match acls {
//            Ok(acls) => {
//                MsgVpnAclProfilesResponse::save("tmp", &acls);
//                let default_acl = deserialize_file_into_type!(format!("tmp/{}/acl/default.yaml", random_vpn), MsgVpnAclProfile);
//                let myacl_acl = deserialize_file_into_type!(format!("tmp/{}/acl/myacl.yaml", random_vpn), MsgVpnAclProfile);
//                assert_eq!(default_acl.unwrap().acl_profile_name().unwrap(), "default");
//                assert_eq!(myacl_acl.unwrap().acl_profile_name().unwrap(), "myacl");
//            },
//            Err(e) => {
//                error!("save multiple acls failed");
//            }
//        }
//
//
//
//        println!("acl update");
//        let updated = MsgVpnAclProfileResponse::update(
//            &random_vpn,
//            "test_yaml/acl/update.yaml",
//            "",
//            &mut core,
//            &client
//        );
//        match updated {
//            Ok(acl) => {
//                assert_eq!(acl.data().unwrap().client_connect_default_action().unwrap(), &"disallow");
//            },
//            Err(e) => {
//                error!("acl update failed");
//            }
//        }


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