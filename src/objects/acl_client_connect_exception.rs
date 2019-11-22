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
use solace_semp_client::models::MsgVpnAclProfileClientConnectExceptionsResponse;

// fetch ACL client connect exceptions

impl Fetch<MsgVpnAclProfileClientConnectExceptionsResponse> for MsgVpnAclProfileClientConnectExceptionsResponse {
    fn fetch(in_vpn: &str, acl_profile_name: &str, select_key: &str, select_value: &str, count: i32, cursor: &str, selector: &str, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<MsgVpnAclProfileClientConnectExceptionsResponse, &'static str> {
        let (wherev, selectv) = helpers::getwhere(select_key, select_value, selector);
        let request = apiclient
            .default_api()
            .get_msg_vpn_acl_profile_client_connect_exceptions(in_vpn, acl_profile_name, count, cursor, wherev, selectv)
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
