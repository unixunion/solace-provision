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


//                override_parameter!(msg_vpn_name, body.msg_vpn_name().unwrap().clone(), body.set_msg_vpn_name);
                // TODO FIXME macro this
                let mut referenced_msg_vpn_name = msg_vpn_name;
                let file_referenced_msg_vpn = body.msg_vpn_name().unwrap().clone();

                // if msg_vpn_name is overridden, set the same in the body,
                // else set the referenced to the one from the body
                if (&msg_vpn_name != &"") {
                    info!("overriding vpn name to :{}", msg_vpn_name);
                    &body.set_msg_vpn_name(msg_vpn_name.to_owned());
                } else {
                    info!("using vpn_name from file");
                    referenced_msg_vpn_name = &*file_referenced_msg_vpn; //body.msg_vpn_name().unwrap().clone().as_str();
                }

                // TODO FIXME macro this
                let mut referenced_acl_profile_name = acl_profile_name;
                let file_referenced_acl_profile = body.acl_profile_name().unwrap().clone();

                if (&acl_profile_name != &"") {
                    info!("overriding acl profile name to: {}", acl_profile_name);
                    &body.set_acl_profile_name(acl_profile_name.to_owned());
                } else {
                    info!("using acl profile name from file");
                    referenced_acl_profile_name = &*file_referenced_acl_profile;
                }
//                &item.set_acl_profile_name(acl_profile_name.to_owned());

                let request = apiclient
                    .default_api()
                    .create_msg_vpn_acl_profile_client_connect_exception(
                        referenced_msg_vpn_name,
                        referenced_acl_profile_name,
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

        let (mut core, mut client) = solace_connect!();

        let acl_name = "myacl";
        let random_vpn = "testvpn";

        println!("acce delete testvpn");
        let d = MsgVpnResponse::delete(&random_vpn, "", "", &mut core, &client);

        println!("acce create vpn");
        let v = MsgVpnResponse::provision_with_file(
            "",
            "",
            "test_yaml/acce/vpn.yaml", &mut core,
            &client);

        match v {
            Ok(vpn) => {
                assert_eq!(vpn.data().unwrap().msg_vpn_name().unwrap(), &random_vpn);
            },
            Err(e) => {
                error!("acce cannot create testvpn");
            }
        }

        println!("acce provision acl");
        let a = MsgVpnAclProfileResponse::provision_with_file(
            "",
            "",
            "test_yaml/acce/acl.yaml",
            &mut core,
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
            acl_name,
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
            acl_name,
            "aclProfileName",
            acl_name,
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


        println!("acce acl delete");
        let da = MsgVpnAclProfileResponse::delete(&random_vpn, acl_name, "", &mut core, &client);
        match da {
            Ok(resp) => {
                assert_eq!(resp.meta().response_code(), &200);
            },
            Err(e) => {
                error!("acce acl delete failed");
            }
        }


    }

}