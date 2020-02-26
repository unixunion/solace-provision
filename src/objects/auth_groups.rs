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

impl Fetch<MsgVpnAuthorizationGroupsResponse> for MsgVpnAuthorizationGroupsResponse {
    /// fetch multiple
    fn fetch(msg_vpn_name: &str, unused_1: &str, select_key: &str, select_value: &str, count: i32, cursor: &str, selector: &str, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<MsgVpnAuthorizationGroupsResponse, &'static str> {
        let (wherev, mut selectv) = helpers::getwhere(select_key, select_value, selector);

        let request = apiclient
            .authorization_group_api()
            .get_msg_vpn_authorization_groups(msg_vpn_name, count, cursor, wherev, selectv)
            .and_then(|item| {
                futures::future::ok(item)
            });
        core_run!(request, core)
    }
}

impl Provision<MsgVpnAuthorizationGroupResponse> for MsgVpnAuthorizationGroupResponse {
    /// provision from file
    fn provision_with_file(unused_1: &str, unused_2: &str, file_name: &str, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<MsgVpnAuthorizationGroupResponse, &'static str> {

        let deserialized = deserialize_file_into_type!(file_name, MsgVpnAuthorizationGroup);

        match deserialized {
            Some(mut item) => {
                let request = apiclient
                    .default_api()
                    .create_msg_vpn_authorization_group(
                        &*item.msg_vpn_name().cloned().unwrap(),
                        item,
                        helpers::getselect("*"));
                core_run!(request, core)
            }
            _ => unimplemented!()
        }
    }

}

impl Save<MsgVpnAuthorizationGroup> for MsgVpnAuthorizationGroup {
    /// save individual
    fn save(dir: &str, data: &MsgVpnAuthorizationGroup) -> Result<(), &'static str> where MsgVpnAuthorizationGroup: Serialize {
        let vpn_name = data.msg_vpn_name();
        let item_name = data.authorization_group_name();

        debug!("save authorization-group: {:?}, {:?}", vpn_name, item_name);
        data.save_in_dir(dir, "authorization-group", &vpn_name, &item_name);
        Ok(())
    }
}

impl Save<MsgVpnAuthorizationGroupsResponse> for MsgVpnAuthorizationGroupsResponse {
    /// save multiple
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

impl Update<MsgVpnAuthorizationGroupResponse> for MsgVpnAuthorizationGroupResponse {

    fn update(unused_1: &str, file_name: &str, unused_2: &str, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<MsgVpnAuthorizationGroupResponse, &'static str> {

        let deserialized = deserialize_file_into_type!(file_name, MsgVpnAuthorizationGroup);

        match deserialized {
            Some(mut item) => {
                let request = apiclient
                    .default_api()
                    .update_msg_vpn_authorization_group(
                        &*item.msg_vpn_name().cloned().unwrap(),
                        &*item.authorization_group_name().cloned().unwrap(),
                        item,
                        helpers::getselect("*"));
                core_run!(request, core)
            }
            _ => unimplemented!()
        }
    }

    fn enabled(msg_vpn: &str, auth_group_name: &str, selector: Vec<&str>, state: bool, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<MsgVpnAuthorizationGroupResponse, &'static str> {
        println!("retrieving current authorization-group from appliance");
        let mut item = MsgVpnAuthorizationGroupsResponse::fetch(msg_vpn, auth_group_name, "authorizationGroupName", auth_group_name, 10, "", "", core, apiclient)?;
        let mut titem = item.data().unwrap().clone();

        if titem.len() == 1 {
            println!("changing enabled state to: {}", state.to_string());
            let mut x = titem.pop().unwrap();
            x.set_enabled(state);
            let request = apiclient.default_api().update_msg_vpn_authorization_group(msg_vpn, auth_group_name, x, helpers::getselect("*"));
            core_run!(request, core)

        } else {
            error!("error, did not find exactly one item matching query");
            Err("unable to change enabled state")
        }

    }

    fn delete(msg_vpn_name: &str, auth_group_name: &str, unused_1: &str, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<SempMetaOnlyResponse, &'static str> {
        let request = apiclient.default_api().delete_msg_vpn_authorization_group(msg_vpn_name, auth_group_name);
        core_run_meta!(request, core)
    }

}

mod tests {
    extern crate rand;

    use crate::provision::Provision;
    use solace_semp_client::models::{MsgVpnQueue, MsgVpnResponse, MsgVpnAclProfileClientConnectExceptionResponse, MsgVpnAclProfileClientConnectException, MsgVpnAclProfileClientConnectExceptionsResponse, MsgVpnAclProfileResponse, MsgVpnAclProfilePublishExceptionResponse, MsgVpnAclProfilePublishExceptionsResponse, MsgVpnAclProfileSubscribeExceptionResponse, MsgVpnAclProfileSubscribeExceptionsResponse, MsgVpnAclProfileSubscribeException, MsgVpnClientProfileResponse, MsgVpnAuthorizationGroupResponse, MsgVpnAuthorizationGroupsResponse, MsgVpnAuthorizationGroup};
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

        println!("ag delete testvpn");
        let d = MsgVpnResponse::delete(&test_vpn, "", "", &mut core, &client);

        println!("ag create vpn");
        let v = MsgVpnResponse::provision_with_file(
            "",
            "",
            "test_yaml/ase/vpn.yaml", &mut core,
            &client);

        match v {
            Ok(vpn) => {
                assert_eq!(vpn.data().unwrap().msg_vpn_name().unwrap(), &test_vpn);
            },
            Err(e) => {
                error!("ag cannot create testvpn");
            }
        }

        println!("ag provision acl");
        let a = MsgVpnAclProfileResponse::provision_with_file(
            "",
            "",
            "test_yaml/ase/acl.yaml",
            &mut core,
            &client
        );

        println!("ag provision verify acl");
        match a {
            Ok(acl) => {
                assert_eq!(acl.data().unwrap().acl_profile_name().unwrap(), "myacl");
            },
            Err(e) => {
                error!("ag acl could not be provisioned");
            }
        }

        println!("provision client-profile");
        let c = MsgVpnClientProfileResponse::provision_with_file(
            "",
            "",
            "test_yaml/ag/client-profile.yaml",
            &mut core,
            &client);
        match c {
            Ok(cp) => {
                assert_eq!(cp.data().unwrap().client_profile_name().unwrap(), "myclientprofile");
            },
            Err(e) => {
                error!("ag client-profile could not be provisioned");
            }
        }

        println!("provision ag");
        let ag = MsgVpnAuthorizationGroupResponse::provision_with_file(
            "",
            "",
            "test_yaml/ag/ag.yaml",
            &mut core,
            &client);
        match ag {
            Ok(a) => {
                assert_eq!(a.data().unwrap().authorization_group_name().unwrap(), "myauthgroup");
            },
            Err(e) => {
                error!("ag auth-group could not be provisioned");
            }
        }

        println!("ag fetch");
        let fag = MsgVpnAuthorizationGroupsResponse::fetch(
            &test_vpn,
            "",
            "authorizationGroupName",
            "myauthgroup",
            10,
            "",
            "*",
            &mut core,
            &client);
        match fag {
            Ok(ag) => {
                assert_eq!(ag.data().unwrap().len(), 1);
                MsgVpnAuthorizationGroupsResponse::save("tmp/ag", &ag);
                let c = deserialize_file_into_type!("tmp/ag/testvpn/authorization-group/myauthgroup.yaml", MsgVpnAuthorizationGroup);
                assert_eq!(c.unwrap().authorization_group_name().unwrap(), "myauthgroup");
            },
            Err(e) => {
                error!("ag auth-group could not be fetched");
            }
        }

        print!("ag update");
        let dag = MsgVpnAuthorizationGroupResponse::update(
            "",
            "test_yaml/ag/ag-update.yaml",
            "",
            &mut core,
            &client);
        match dag {
            Ok(ag) => {
                assert_eq!(ag.data().unwrap().enabled().unwrap(), &true);
            },
            Err(e) => {
                error!("ag auth-group could not be updated");
            }
        }

        println!("ag enable");
        let eag = MsgVpnAuthorizationGroupResponse::enabled(
            &test_vpn,
            "myauthgroup",
            vec![],
            false,
            &mut core,
            &client);
        match eag {
            Ok(ag) => {
                assert_eq!(ag.data().unwrap().enabled().unwrap(), &false);
            },
            Err(e) => {
                error!("ag auth-group could not be enabled");
            }
        }

        println!("ag save");
        let mut ag = MsgVpnAuthorizationGroup::new();
        ag.set_authorization_group_name("tmpag".to_owned());
        ag.set_acl_profile_name("tmpacl".to_owned());
        ag.set_msg_vpn_name(test_vpn.to_owned());
        MsgVpnAuthorizationGroup::save("tmp", &ag);

        let deserialized = deserialize_file_into_type!(format!("tmp/{}/authorization-group/tmpag.yaml", test_vpn), MsgVpnAuthorizationGroup);

        match deserialized {
            Some(a) => {
                assert_eq!(a.authorization_group_name().unwrap(), "tmpag");
            }
            _ => {
                error!("ag save error");
            }
        }


        println!("ag delete");
        let dag = MsgVpnAuthorizationGroupResponse::delete(
            &test_vpn,
            "myauthgroup",
            "",
            &mut core,
            &client);
        match dag {
            Ok(d) => assert_eq!(d.meta().response_code(), &200),
            Err(e) => error!("ag auth-group could not be enabled")
        }


        println!("ag acl delete");
        let da = MsgVpnAclProfileResponse::delete(&test_vpn, "myacl", "", &mut core, &client);
        match da {
            Ok(resp) => assert_eq!(resp.meta().response_code(), &200),
            Err(e) => error!("acl delete failed")
        }


        println!("ag client-profile delete");
        let da = MsgVpnClientProfileResponse::delete(
            &test_vpn,
            "myclientprofile",
            "",
            &mut core,
            &client);

        match da {
            Ok(resp) => assert_eq!(resp.meta().response_code(), &200),
            Err(e) => error!("ag acl delete failed")
        }

        println!("ag delete test vpn");
        let d = MsgVpnResponse::delete(&test_vpn, "", "", &mut core, &client);

    }
}