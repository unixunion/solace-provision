
mod tests {

    use solace_semp_client::models::{MsgVpn, MsgVpnTopicEndpointsResponse, MsgVpnQueuesResponse};
    use solace_semp_client::models::MsgVpnQueue;
    use crate::fetch::Fetch;
    use solace_semp_client::models::MsgVpnsResponse;
    use tokio_core::reactor::Core;
    use hyper::Client;
    use solace_semp_client::apis::client::APIClient;
    use crate::clientconnection;
    use crate::clientconnection::SPClientConnection;
    use hyper::client::HttpConnector;
    use native_tls::{TlsConnector, Certificate};



//    #[test]
//    fn it_works() {
//        // create a new vpn, then test if our new traits and functions are bound
//        let mut core = Core::new().unwrap();
//        let handle = core.handle();
//        let mut http = HttpConnector::new(4, &handle);
//        http.enforce_http(false);
//
//        let mut tls = TlsConnector::builder().unwrap().build();
//
//        let hyperclient = Client::configure()
//            .connector(hyper_tls::HttpsConnector::from((http, tls.unwrap()))).build(&handle);
//
//
//        let c = SPClientConnection::new("https://localhost:8080/SEMP/v2/config", "admin", "admin", hyperclient);
//        let client = APIClient::new(c.configuration);
//
//
//        match MsgVpnQueuesResponse::fetch("default", "default", "default", 10, "", "*", &mut core, &client) {
//            Ok(i) => {
//                assert_eq!(&200, i.meta().response_code());
//            },
//            Err(e) =>{
//                error!("error: {}", e);
//                panic!("error: {}", e);
//            }
//        }
//
//    }
}

use solace_semp_client::apis::client::APIClient;
use solace_semp_client::models::{MsgVpn, MsgVpnTopicEndpointsResponse, MsgVpnSequencedTopic, MsgVpnQueueSubscriptionsResponse, MsgVpnSequencedTopicsResponse, MsgVpnAuthorizationGroup, MsgVpnAuthorizationGroupsResponse, MsgVpnBridgeRemoteMsgVpnsResponse, MsgVpnBridgeRemoteSubscriptionsResponse, MsgVpnBridgesResponse, MsgVpnBridgeRemoteSubscriptionResponse, MsgVpnBridgeTlsTrustedCommonNamesResponse, AboutApiResponse, MsgVpnReplayLogsResponse, MsgVpnReplayLogResponse, MsgVpnDmrBridgesResponse, MsgVpnDmrBridgeResponse, DmrClustersResponse, DmrClusterResponse, DmrClusterLinksResponse, DmrClusterLinkRemoteAddressesResponse, MsgVpnAclProfilePublishExceptionsResponse, MsgVpnAclProfileSubscribeExceptionsResponse, MsgVpnAclProfileClientConnectExceptionsResponse};
use tokio_core::reactor::Core;
use hyper_tls::HttpsConnector;
use hyper::client::HttpConnector;
use solace_semp_client::models::MsgVpnBridge;
use solace_semp_client::models::MsgVpnQueue;
use solace_semp_client::models::MsgVpnAclProfile;
use serde::Serialize;
use colored::Colorize;
use futures::{Future};
use futures::future::{AndThen, FutureResult};
use std::fs::File;
use serde::Deserialize;
use crate::helpers;
use solace_semp_client::models::MsgVpnsResponse;
use solace_semp_client::apis::Error;
use solace_semp_client::models::MsgVpnResponse;
use solace_semp_client::models::MsgVpnQueuesResponse;
use solace_semp_client::models::MsgVpnAclProfilesResponse;
use solace_semp_client::models::MsgVpnClientProfilesResponse;
use solace_semp_client::models::MsgVpnClientUsernamesResponse;
use std::process::exit;
use log::{info, warn, error, debug};
use std::path::Path;
use std::fs;
use std::io::Write;
use serde_json::Value;


// shared base trait for all solace fetch-able objects
pub trait Fetch<T> {

    fn fetch(in_vpn: &str, sub_item: &str, select_key: &str, select_value: &str, count: i32,  cursor: &str, selector: &str, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<T, &'static str>;

//    fn make_request(core: &mut Core) {
//
//    }

//    fn handle_response(request: AndThen<T, B, F>, core: &mut Core) -> Result<T, &'static str> {
//        match core.run(request) {
//            Ok(response) => {
//                let t = format!("{}", serde_yaml::to_string(&response.data().unwrap()).unwrap());
//                println!("{}", &t);
//                Ok(response)
//            },
//            Err(e) => {
//                error!("error fetching: {:?}", e);
//                panic!("fetch error: {:?}", e);
//                Err("fetch error")
//            }
//        }
//    }

}

