
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
use solace_semp_client::models::{MsgVpn, MsgVpnTopicEndpointsResponse, MsgVpnSequencedTopic, MsgVpnQueueSubscriptionsResponse, MsgVpnSequencedTopicsResponse, MsgVpnAuthorizationGroup, MsgVpnAuthorizationGroupsResponse, MsgVpnBridgeRemoteMsgVpnsResponse, MsgVpnBridgeRemoteSubscriptionsResponse, MsgVpnBridgesResponse, MsgVpnBridgeRemoteSubscriptionResponse, MsgVpnBridgeTlsTrustedCommonNamesResponse, AboutApiResponse, MsgVpnReplayLogsResponse, MsgVpnReplayLogResponse, MsgVpnDmrBridgesResponse, MsgVpnDmrBridgeResponse, DmrClustersResponse, DmrClusterResponse};
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

}


// fetch about
impl Fetch<AboutApiResponse> for AboutApiResponse {
    fn fetch(in_vpn: &str, sub_item: &str, select_key: &str, select_value: &str, count: i32, cursor: &str, selector: &str, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<AboutApiResponse, &'static str> {
        let request = apiclient
            .about_api()
            .get_about_api()
            .and_then(|about| {
                debug!("{:?}", about);
                futures::future::ok(about)
            });

        match core.run(request) {
            Ok(response) => {
                let t = format!("{}", serde_yaml::to_string(&response.data().unwrap()).unwrap());
                println!("{}", &t);
                Ok(response)
            },
            Err(e) => {
                error!("error fetching: {:?}", e);
                panic!("fetch error: {:?}", e);
                Err("fetch error")
            }
        }
    }
}

// fetch multple msgvpnsresponse
impl Fetch<MsgVpnsResponse> for MsgVpnsResponse {

    fn fetch(in_vpn: &str, name: &str, select_key: &str, select_value: &str, count: i32, cursor: &str, selector: &str, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<MsgVpnsResponse, &'static str> {
        let (wherev, selectv) = helpers::getwhere(select_key, select_value, selector);
        let request = apiclient
            .msg_vpn_api()
            .get_msg_vpns(count, cursor, wherev, selectv)
            .and_then(|vpn| {
                debug!("{:?}", vpn);
                futures::future::ok(vpn)
            });

//        info!("{}", request);
//        core_run(request)
//        Self::core_run(request, core)

        match core.run(request) {
            Ok(response) => {
                let t = format!("{}", serde_yaml::to_string(&response.data().unwrap()).unwrap());
                println!("{}", &t);
                Ok(response)
            },
            Err(e) => {
                error!("error fetching: {:?}", e);
                panic!("fetch error: {:?}", e);
                Err("fetch error")
            }
        }

    }

}

impl Fetch<MsgVpnQueuesResponse> for MsgVpnQueuesResponse {

    fn fetch(in_vpn: &str, name: &str, select_key: &str, select_value: &str, count: i32, cursor: &str, selector: &str, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<MsgVpnQueuesResponse, &'static str> {
        let (wherev, selectv) = helpers::getwhere(select_key, select_value, selector);
        let request = apiclient
            .msg_vpn_api()
            .get_msg_vpn_queues(in_vpn, count, cursor, wherev, selectv)
            .and_then(|vpn| {
                futures::future::ok(vpn)
            });

        match core.run(request) {
            Ok(response) => {
                println!("{}",format!("{}", serde_yaml::to_string(&response.data().unwrap()).unwrap()));
                Ok(response)
            },
            Err(e) => {
                error!("error fetching: {:?}", e);
                panic!("fetch error: {:?}", e);
                Err("fetch error")
            }
        }

    }
}

impl Fetch<MsgVpnAclProfilesResponse> for MsgVpnAclProfilesResponse {

    fn fetch(in_vpn: &str, name: &str, select_key: &str, select_value: &str, count: i32, cursor: &str, selector: &str, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<MsgVpnAclProfilesResponse, &'static str> {
        let (wherev, selectv) = helpers::getwhere(select_key, select_value, selector);
        let request = apiclient
            .msg_vpn_api()
            .get_msg_vpn_acl_profiles(in_vpn, count, cursor, wherev, selectv)
            .and_then(|acl| {
                futures::future::ok(acl)
            });

        match core.run(request) {
            Ok(response) => {
                println!("{}",format!("{}", serde_yaml::to_string(&response.data().unwrap()).unwrap()));
                Ok(response)
            },
            Err(e) => {
                error!("error fetching: {:?}", e);
                panic!("fetch error: {:?}", e);
                Err("fetch error")
            }
        }

    }
}



impl Fetch<MsgVpnClientProfilesResponse> for MsgVpnClientProfilesResponse {

    fn fetch(in_vpn: &str, name: &str, select_key: &str, select_value: &str, count: i32, cursor: &str, selector: &str, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<MsgVpnClientProfilesResponse, &'static str> {
        let (wherev, selectv) = helpers::getwhere(select_key, select_value, selector);
        let request = apiclient
            .msg_vpn_api()
            .get_msg_vpn_client_profiles(in_vpn, count, cursor, wherev, selectv)
            .and_then(|acl| {
                futures::future::ok(acl)
            });

        match core.run(request) {
            Ok(response) => {
                println!("{}",format!("{}", serde_yaml::to_string(&response.data().unwrap()).unwrap()));
                Ok(response)
            },
            Err(e) => {
                error!("error fetching: {:?}", e);
                panic!("fetch error: {:?}", e);
                Err("fetch error")
            }
        }

    }
}


impl Fetch<MsgVpnClientUsernamesResponse> for MsgVpnClientUsernamesResponse {

    fn fetch(in_vpn: &str, name: &str, select_key: &str, select_value: &str, count: i32, cursor: &str, selector: &str, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<MsgVpnClientUsernamesResponse, &'static str> {
        let (wherev, selectv) = helpers::getwhere(select_key, select_value, selector);
        let request = apiclient
            .msg_vpn_api()
            .get_msg_vpn_client_usernames(in_vpn, count, cursor, wherev, selectv)
            .and_then(|cu| {
                debug!("{:?}", cu);
                futures::future::ok(cu)
            });

        match core.run(request) {
            Ok(response) => {
                println!("{}",format!("{}", serde_yaml::to_string(&response.data().unwrap()).unwrap()));
                Ok(response)
            },
            Err(e) => {
                error!("error fetching: {:?}", e);
                panic!("fetch error: {:?}", e);
                Err("fetch error")
            }
        }

    }
}


impl Fetch<MsgVpnQueueSubscriptionsResponse> for MsgVpnQueueSubscriptionsResponse {
    fn fetch(in_vpn: &str, queue_name: &str, select_key: &str, select_value: &str, count: i32, cursor: &str, selector: &str, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<MsgVpnQueueSubscriptionsResponse, &'static str> {
        let (wherev, mut selectv) = helpers::getwhere(select_key, select_value, selector);

        let request = apiclient
            .default_api()
            .get_msg_vpn_queue_subscriptions(in_vpn, queue_name, count, cursor, wherev, selectv)
            .and_then(|item| {
                futures::future::ok(item)
            });

        match core.run(request) {
            Ok(response) => {
                println!("{}",format!("{}", serde_yaml::to_string(&response.data().unwrap()).unwrap()));
                Ok(response)
            },
            Err(e) => {
                error!("error fetching: {:?}", e);
                panic!("fetch error: {:?}", e);
                Err("fetch error")
            }
        }

    }
}


impl Fetch<MsgVpnSequencedTopicsResponse> for MsgVpnSequencedTopicsResponse {
    fn fetch(in_vpn: &str, sub_item: &str, select_key: &str, select_value: &str ,count: i32, cursor: &str, selector: &str, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<MsgVpnSequencedTopicsResponse, &'static str> {
        let (wherev, mut selectv) = helpers::getwhere(select_key, select_value, selector);

        let request = apiclient
            .default_api()
            .get_msg_vpn_sequenced_topics(in_vpn, count, cursor,  wherev, selectv)
            .and_then(|item| {
                futures::future::ok(item)
            });

        match core.run(request) {
            Ok(response) => {
                println!("{}",format!("{}", serde_yaml::to_string(&response.data().unwrap()).unwrap()));
                Ok(response)
            },
            Err(e) => {
                error!("error fetching: {:?}", e);
                panic!("fetch error: {:?}", e);
                Err("fetch error")
            }
        }
    }
}


// topic endpoint

impl Fetch<MsgVpnTopicEndpointsResponse> for MsgVpnTopicEndpointsResponse {
    fn fetch(in_vpn: &str, sub_item: &str, select_key: &str, select_value: &str, count: i32, cursor: &str, selector: &str, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<MsgVpnTopicEndpointsResponse, &'static str> {
        let (wherev, mut selectv) = helpers::getwhere(select_key, select_value, selector);

        let request = apiclient
            .topic_endpoint_api()
            .get_msg_vpn_topic_endpoints(in_vpn, count, cursor,  wherev, selectv)
            .and_then(|item| {
                futures::future::ok(item)
            });

        match core.run(request) {
            Ok(response) => {
                println!("{}",format!("{}", serde_yaml::to_string(&response.data().unwrap()).unwrap()));
                Ok(response)
            },
            Err(e) => {
                error!("error fetching: {:?}", e);
                panic!("fetch error: {:?}", e);
                Err("fetch error")
            }
        }
    }
}


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

        match core.run(request) {
            Ok(response) => {
                println!("{}",format!("{}", serde_yaml::to_string(&response.data().unwrap()).unwrap()));
                Ok(response)
            },
            Err(e) => {
                error!("error fetching: {:?}", e);
                panic!("fetch error: {:?}", e);
                Err("fetch error")
            }
        }
    }
}

// bridge

impl Fetch<MsgVpnBridgesResponse> for MsgVpnBridgesResponse {

    // select_key = bridgeName
    fn fetch(in_vpn: &str, bridge_name: &str, select_key: &str, select_value: &str, count: i32, cursor: &str, selector: &str, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<MsgVpnBridgesResponse, &'static str> {
        let (wherev, mut selectv) = helpers::getwhere(select_key, select_value, selector);

        let request = apiclient
            .bridge_api()
            .get_msg_vpn_bridges(in_vpn, count, cursor, wherev, selectv)
            .and_then(|item| {
                futures::future::ok(item)
            });

        match core.run(request) {
            Ok(response) => {
                println!("{}",format!("{}", serde_yaml::to_string(&response.data().unwrap()).unwrap()));
                Ok(response)
            },
            Err(e) => {
                error!("error fetching: {:?}", e);
                panic!("fetch error: {:?}", e);
                Err("fetch error")
            }
        }
    }
}


// remote bridge

impl Fetch<MsgVpnBridgeRemoteMsgVpnsResponse> for MsgVpnBridgeRemoteMsgVpnsResponse {
    fn fetch(in_vpn: &str, bridge_name: &str, bridge_virtual_router: &str, select_value: &str, count: i32, cursor: &str, selector: &str, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<MsgVpnBridgeRemoteMsgVpnsResponse, &'static str> {
        let (wherev, mut selectv) = helpers::getwhere("bridgeName", bridge_name, selector);

        let request = apiclient
            .default_api()
            .get_msg_vpn_bridge_remote_msg_vpns(in_vpn, bridge_name, bridge_virtual_router,  wherev, selectv)
            .and_then(|item| {
                futures::future::ok(item)
            });

        match core.run(request) {
            Ok(response) => {
                println!("{}",format!("{}", serde_yaml::to_string(&response.data().unwrap()).unwrap()));
                Ok(response)
            },
            Err(e) => {
                error!("error fetching: {:?}", e);
                panic!("fetch error: {:?}", e);
                Err("fetch error")
            }
        }
    }
}

// remote bridge subscriptions

impl Fetch<MsgVpnBridgeRemoteSubscriptionsResponse> for MsgVpnBridgeRemoteSubscriptionsResponse {
    fn fetch(in_vpn: &str, bridge_name: &str, remote_subscription_topic: &str, bridge_virtual_router: &str, count: i32, cursor: &str, selector: &str, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<MsgVpnBridgeRemoteSubscriptionsResponse, &'static str> {
        let (wherev, mut selectv) = helpers::getwhere("remoteSubscriptionTopic", remote_subscription_topic, selector);

        let request = apiclient
            .default_api()
            .get_msg_vpn_bridge_remote_subscriptions(in_vpn, bridge_name, bridge_virtual_router, count, cursor, wherev, selectv)
            .and_then(|item| {
                futures::future::ok(item)
            });

        match core.run(request) {
            Ok(response) => {
                println!("{}",format!("{}", serde_yaml::to_string(&response.data().unwrap()).unwrap()));
                Ok(response)
            },
            Err(e) => {
                error!("error fetching: {:?}", e);
                panic!("fetch error: {:?}", e);
                Err("fetch error")
            }
        }
    }
}

// bridge trusted common name

impl Fetch<MsgVpnBridgeTlsTrustedCommonNamesResponse> for MsgVpnBridgeTlsTrustedCommonNamesResponse {
    fn fetch(in_vpn: &str, bridge_name: &str, common_name: &str, bridge_virtual_router: &str, count: i32, cursor: &str, selector: &str, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<MsgVpnBridgeTlsTrustedCommonNamesResponse, &'static str> {
        let (wherev, mut selectv) = helpers::getwhere("tlsTrustedCommonName", common_name, selector);

        let request = apiclient
            .default_api()
            .get_msg_vpn_bridge_tls_trusted_common_names(in_vpn, bridge_name, bridge_virtual_router, wherev, selectv)
            .and_then(|item| {
                futures::future::ok(item)
            });

        match core.run(request) {
            Ok(response) => {
                println!("{}",format!("{}", serde_yaml::to_string(&response.data().unwrap()).unwrap()));
                Ok(response)
            },
            Err(e) => {
                error!("error fetching: {:?}", e);
                panic!("fetch error: {:?}", e);
                Err("fetch error")
            }
        }
    }
}

// replay log

impl Fetch<MsgVpnReplayLogResponse> for MsgVpnReplayLogResponse {
    fn fetch(in_vpn: &str, replay_log_name: &str, unused_1: &str, unused_2: &str, count: i32, cursor: &str, selector: &str, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<MsgVpnReplayLogResponse, &'static str> {
        let (wherev, mut selectv) = helpers::getwhere("replayLogName", replay_log_name, selector);
        let request = apiclient
            .default_api()
            .get_msg_vpn_replay_log(in_vpn, replay_log_name,  selectv)
            .and_then(|item| {
                futures::future::ok(item)
            });

        match core.run(request) {
            Ok(response) => {
                println!("{}",format!("{}", serde_yaml::to_string(&response.data().unwrap()).unwrap()));
                Ok(response)
            },
            Err(e) => {
                error!("error fetching: {:?}", e);
                panic!("fetch error: {:?}", e);
                Err("fetch error")
            }
        }
    }
}





// DMR Bridges

/**
    select key is one of: msgVpnName, remoteMsgVpnName, remoteNodeName
**/
///
/// #Argumentts
/// * `in_vpn` - the vpn name
/// * `sub_item` - unused
/// * `select_key` - one of msgVpnName, remoteMsgVpnName, remoteNodeName
/// * `select_value` - match value for the select_key
///
impl Fetch<MsgVpnDmrBridgesResponse> for MsgVpnDmrBridgesResponse {
    fn fetch(in_vpn: &str, unused_sub_item: &str, select_key: &str, select_value: &str, count: i32, cursor: &str, selector: &str, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<MsgVpnDmrBridgesResponse, &'static str> {
        let (wherev, mut selectv) = helpers::getwhere(select_key, select_value, selector);
        let request = apiclient
            .dmr_bridge_api()
            .get_msg_vpn_dmr_bridges(in_vpn, count,  cursor, wherev, selectv)
            .and_then(|item| {
                futures::future::ok(item)
            });

        match core.run(request) {
            Ok(response) => {
                println!("{}",format!("{}", serde_yaml::to_string(&response.data().unwrap()).unwrap()));
                Ok(response)
            },
            Err(e) => {
                error!("error fetching: {:?}", e);
                panic!("fetch error: {:?}", e);
                Err("fetch error")
            }
        }
    }
}


// DMR cluster

impl Fetch<DmrClusterResponse> for DmrClusterResponse {
    fn fetch(in_vpn: &str, dmr_cluster_name: &str, select_key: &str, select_value: &str, count: i32, cursor: &str, selector: &str, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<DmrClusterResponse, &'static str> {
        let (wherev, mut selectv) = helpers::getwhere(select_key, select_value, selector);
        let request = apiclient
            .default_api()
            .get_dmr_cluster(dmr_cluster_name, selectv)
            .and_then(|item| {
                futures::future::ok(item)
            });

        match core.run(request) {
            Ok(response) => {
                println!("{}",format!("{}", serde_yaml::to_string(&response.data().unwrap()).unwrap()));
                Ok(response)
            },
            Err(e) => {
                error!("error fetching: {:?}", e);
                panic!("fetch error: {:?}", e);
                Err("fetch error")
            }
        }
    }
}

// fetch all clusters by a key, where key is one of
//
impl Fetch<DmrClustersResponse> for DmrClustersResponse {
    fn fetch(in_vpn: &str, dmr_cluster_name: &str, select_key: &str, select_value: &str, count: i32, cursor: &str, selector: &str, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<DmrClustersResponse, &'static str> {
        let (wherev, mut selectv) = helpers::getwhere(select_key, select_value, selector);
        let request = apiclient
            .dmr_cluster_api()
            .get_dmr_clusters(count, cursor, wherev, selectv)
            .and_then(|item| {
                futures::future::ok(item)
            });

        match core.run(request) {
            Ok(response) => {
                println!("{}",format!("{}", serde_yaml::to_string(&response.data().unwrap()).unwrap()));
                Ok(response)
            },
            Err(e) => {
                error!("error fetching: {:?}", e);
                panic!("fetch error: {:?}", e);
                Err("fetch error")
            }
        }
    }
}
