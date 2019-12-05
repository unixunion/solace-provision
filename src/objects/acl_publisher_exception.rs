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
use solace_semp_client::models::{MsgVpnAclProfilePublishExceptionsResponse, MsgVpnAclProfilePublishException, MsgVpnAclProfilePublishExceptionResponse, SempMetaOnlyResponse};
use percent_encoding::{utf8_percent_encode, NON_ALPHANUMERIC};

// FETCH ACL publish exceptions

impl Fetch<MsgVpnAclProfilePublishExceptionsResponse> for MsgVpnAclProfilePublishExceptionsResponse {
    fn fetch(msg_vpn_name: &str, acl_profile_name: &str, select_key: &str, select_value: &str, count: i32, cursor: &str, selector: &str, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<MsgVpnAclProfilePublishExceptionsResponse, &'static str> {
        let (wherev, selectv) = helpers::getwhere(select_key, select_value, selector);
        let request = apiclient
            .default_api()
            .get_msg_vpn_acl_profile_publish_exceptions(msg_vpn_name, acl_profile_name, count, cursor, wherev, selectv)
            .and_then(|acl| {
                futures::future::ok(acl)
            });
        core_run!(request, core)
    }
}

// provision ACL publish exception

impl Provision<MsgVpnAclProfilePublishExceptionResponse> for MsgVpnAclProfilePublishExceptionResponse {
    fn provision_with_file(unused_1: &str, unused_2: &str, file_name: &str, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<MsgVpnAclProfilePublishExceptionResponse, &'static str> {
        let file = std::fs::File::open(file_name).unwrap();
        let deserialized: Option<MsgVpnAclProfilePublishException> = serde_yaml::from_reader(file).unwrap();
        match deserialized {
            Some(mut item) => {
//                item.set_msg_vpn_name(msg_vpn_name.to_owned());
//                let acl_profile_name = &*item.acl_profile_name().cloned().unwrap();
                let request = apiclient
                    .default_api()
                    .create_msg_vpn_acl_profile_publish_exception(
                        &*item.msg_vpn_name().cloned().unwrap(),
                        &*item.acl_profile_name().cloned().unwrap(),
                        item,
                        helpers::getselect("*"));

                core_run!(request, core)
            }
            _ => unimplemented!()
        }
    }

}


//save ACL publish exception

impl Save<MsgVpnAclProfilePublishException> for MsgVpnAclProfilePublishException {
    fn save(dir: &str, data: &MsgVpnAclProfilePublishException) -> Result<(), &'static str> where MsgVpnAclProfilePublishException: Serialize {
        let mut hasher = sha1::Sha1::new();
        hasher.update(data.publish_exception_topic().unwrap().as_bytes());
        let s = hasher.digest().to_string();
        let topic_hash = Option::from(&s);
        let vpn_name = data.msg_vpn_name();
        let acl_profile_name = data.acl_profile_name();
        debug!("save acl-publish exception: {:?}, {:?}", vpn_name, acl_profile_name);
        data.save_in_dir(dir, &format!("acl/{}/publish-exceptions", &acl_profile_name.unwrap()), &vpn_name, &topic_hash);
        Ok(())
    }
}

// save ACL publish exceptions response

impl Save<MsgVpnAclProfilePublishExceptionsResponse> for MsgVpnAclProfilePublishExceptionsResponse {
    fn save(dir: &str, data: &MsgVpnAclProfilePublishExceptionsResponse) -> Result<(), &'static str> where MsgVpnAclProfilePublishExceptionsResponse: Serialize {
        match data.data() {
            Some(acls) => {
                for acl in acls {
                    match MsgVpnAclProfilePublishException::save(dir, acl) {
                        Ok(t) => debug!("success saving"),
                        Err(e) => error!("error writing: {:?}", e)
                    }

                }
                Ok(())
            },
            _ => {
                error!("no acls");
                Err("no acls")
            }
        }
    }
}

// update ACL publish exception
impl Update<MsgVpnAclProfilePublishExceptionResponse> for MsgVpnAclProfilePublishExceptionResponse {

    fn delete_by_sub_item(msg_vpn: &str, acl_profile_name: &str, topic_syntax: &str, publish_exception_topic: &str, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<SempMetaOnlyResponse, &'static str> {
        let request = apiclient.default_api().delete_msg_vpn_acl_profile_publish_exception(
            msg_vpn,
            acl_profile_name,
            topic_syntax,
            &*utf8_percent_encode(publish_exception_topic, NON_ALPHANUMERIC).to_string());
        core_run_meta!(request, core)
    }
}

mod test {
    extern crate rand;

    use crate::provision::Provision;
    use solace_semp_client::models::{MsgVpnQueue, MsgVpnResponse, MsgVpnAclProfileClientConnectExceptionResponse, MsgVpnAclProfileClientConnectException, MsgVpnAclProfileClientConnectExceptionsResponse, MsgVpnAclProfileResponse, MsgVpnAclProfilePublishExceptionResponse, MsgVpnAclProfilePublishExceptionsResponse};
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
        let test_vpn = "testvpn";

        println!("ape delete testvpn");
        let d = MsgVpnResponse::delete(&test_vpn, "", "", &mut core, &client);

        println!("ape create vpn");
        let v = MsgVpnResponse::provision_with_file(
            "",
            "",
            "test_yaml/ape/vpn.yaml", &mut core,
            &client);

        match v {
            Ok(vpn) => {
                assert_eq!(vpn.data().unwrap().msg_vpn_name().unwrap(), &test_vpn);
            },
            Err(e) => {
                error!("ape cannot create testvpn");
            }
        }

        println!("ape provision acl");
        let a = MsgVpnAclProfileResponse::provision_with_file(
            "",
            "",
            "test_yaml/ape/acl.yaml",
            &mut core,
            &client
        );

        println!("ape provision verify acl");
        match a {
            Ok(acl) => {
                assert_eq!(acl.data().unwrap().acl_profile_name().unwrap(), "myacl");
            },
            Err(e) => {
                error!("ape acl could not be provisioned");
            }
        }

        println!("ape provision");
        let a = MsgVpnAclProfilePublishExceptionResponse::provision_with_file(
            "",
            "",
            "test_yaml/ape/ape.yaml", &mut core,
            &client
        );

        println!("ape provision verify");
        match a {
            Ok(acce) => {
                assert_eq!(acce.data().unwrap().acl_profile_name().unwrap(), "myacl");
            }
            Err(e) => {
                error!("ape could not be provisioned");
            }
        }

        println!("ape fetch");
        let fa = MsgVpnAclProfilePublishExceptionsResponse::fetch(
            &test_vpn,
            acl_name,
            "aclProfileName",
            acl_name,
            10,
            "",
            "*",
            &mut core,
            &client
        );

        println!("ape fetch verify");
        match fa {
            Ok(acls) => {
                assert_eq!(acls.data().unwrap().len(), 1);
            },
            Err(e) => {
                error!("ape fetch failed");
            }
        }


        println!("ape acl delete");
        let da = MsgVpnAclProfileResponse::delete(&test_vpn, acl_name, "", &mut core, &client);
        match da {
            Ok(resp) => {
                assert_eq!(resp.meta().response_code(), &200);
            },
            Err(e) => {
                error!("ape acl delete failed");
            }
        }


    }

}