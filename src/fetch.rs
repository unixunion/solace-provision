
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

    #[test]
    fn it_works() {
        // create a new vpn, then test if our new traits and functions are bound
        let mut core = Core::new().unwrap();
        let handle = core.handle();
        let hyperclient = Client::configure()
            .connector(hyper_tls::HttpsConnector::new(4, &handle)
                .unwrap()
            )
            .build(&handle);
        let c = SPClientConnection::new("https://localhost:8080/SEMP/v2/config", "admin", "admin", hyperclient);
        let client = APIClient::new(c.configuration);

        match MsgVpnQueuesResponse::fetch("default", "*", "default", 10, "", "*", &mut core, &client) {
            Ok(i) => {
                assert_eq!(&200, i.meta().response_code());
            },
            Err(e) =>{
                error!("error: {}", e);
                panic!("error: {}", e);
            }
        }

    }
}

use solace_semp_client::apis::client::APIClient;
use solace_semp_client::models::{MsgVpn, MsgVpnTopicEndpointsResponse, MsgVpnSequencedTopic, MsgVpnQueueSubscriptionsResponse};
use tokio_core::reactor::Core;
use hyper_tls::HttpsConnector;
use hyper::client::HttpConnector;
use solace_semp_client::models::MsgVpnBridge;
use solace_semp_client::models::MsgVpnQueue;
use solace_semp_client::models::MsgVpnAclProfile;
use serde::Serialize;
use colored::Colorize;
use futures::{Future};
use futures::future::AndThen;
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

// shared base trait for all solace fetch-able objects
pub trait Fetch<T> {

    fn fetch(in_vpn: &str, sub_item: &str, sub_identifier: &str, count: i32,  cursor: &str, selector: &str, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<T, &'static str>;

}


// fetch multple msgvpnsresponse
impl Fetch<MsgVpnsResponse> for MsgVpnsResponse {

    fn fetch(in_vpn: &str, name: &str, sub_identifier: &str, count: i32, cursor: &str, selector: &str, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<MsgVpnsResponse, &'static str> {
        let (wherev, selectv) = helpers::getwhere("msgVpnName", name, selector);
        let request = apiclient
            .msg_vpn_api()
            .get_msg_vpns(count, cursor, wherev, selectv)
            .and_then(|vpn| {
                debug!("{:?}", vpn);
                futures::future::ok(vpn)
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

impl Fetch<MsgVpnQueuesResponse> for MsgVpnQueuesResponse {

    fn fetch(in_vpn: &str, name: &str, sub_identifier: &str, count: i32, cursor: &str, selector: &str, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<MsgVpnQueuesResponse, &'static str> {
        let (wherev, selectv) = helpers::getwhere("queueName", name, selector);
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
                panic!("fetch error");
                Err("fetch error")
            }
        }

    }
}

impl Fetch<MsgVpnAclProfilesResponse> for MsgVpnAclProfilesResponse {

    fn fetch(in_vpn: &str, name: &str, sub_identifier: &str, count: i32, cursor: &str, selector: &str, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<MsgVpnAclProfilesResponse, &'static str> {
        let (wherev, selectv) = helpers::getwhere("aclProfileName", name, selector);
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
                panic!("fetch error");
                Err("fetch error")
            }
        }

    }
}



impl Fetch<MsgVpnClientProfilesResponse> for MsgVpnClientProfilesResponse {

    fn fetch(in_vpn: &str, name: &str, sub_identifier: &str, count: i32, cursor: &str, selector: &str, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<MsgVpnClientProfilesResponse, &'static str> {
        let (wherev, selectv) = helpers::getwhere("clientProfileName", name, selector);
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
                panic!("fetch error");
                Err("fetch error")
            }
        }

    }
}


impl Fetch<MsgVpnClientUsernamesResponse> for MsgVpnClientUsernamesResponse {

    fn fetch(in_vpn: &str, name: &str, sub_identifier: &str, count: i32, cursor: &str, selector: &str, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<MsgVpnClientUsernamesResponse, &'static str> {
        let (wherev, selectv) = helpers::getwhere("clientUsername", name, selector);
        let request = apiclient
            .msg_vpn_api()
            .get_msg_vpn_client_usernames(in_vpn, count, cursor, wherev, selectv)
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
                panic!("fetch error");
                Err("fetch error")
            }
        }

    }
}


impl Fetch<MsgVpnQueueSubscriptionsResponse> for MsgVpnQueueSubscriptionsResponse {
    fn fetch(in_vpn: &str, name: &str, sub_identifier: &str, count: i32, cursor: &str, selector: &str, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<MsgVpnQueueSubscriptionsResponse, &'static str> {
        let (wherev, mut selectv) = helpers::getwhere("queueName", name, selector);

//        let mut request;

//        if sub_identifier == "" {
//        let whereitem = format!("{}=={}", "subscriptionTopic", sub_identifier);
//        wherev.push(whereitem);
//        info!("seklect vector {:?}", selectv);
        let request = apiclient
            .default_api()
            .get_msg_vpn_queue_subscriptions(in_vpn, name, count, cursor, wherev, selectv)
            .and_then(|item| {
                futures::future::ok(item)
            });
//        } else {
//
//            let request = apiclient
//                .default_api()
//                .get_msg_vpn_queue_subscription(in_vpn, name, sub_identifier, selectv)
//                .and_then(|item| {
//                    futures::future::ok(item)
//                });
//        }

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
