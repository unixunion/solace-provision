
//mod tests {
//
//    use solace_semp_client::models::{MsgVpn, MsgVpnResponse, MsgVpnQueueResponse, MsgVpnAclProfileResponse, MsgVpnClientProfileResponse, MsgVpnClientUsernameResponse, MsgVpnQueueSubscriptionResponse, MsgVpnSequencedTopicResponse, MsgVpnTopicEndpointResponse, MsgVpnAuthorizationGroupResponse, MsgVpnBridgeResponse, MsgVpnBridgeRemoteMsgVpnResponse, MsgVpnBridgeRemoteSubscriptionResponse, MsgVpnReplayLog, MsgVpnReplayLogResponse, MsgVpnAclProfilePublishException};
//    use crate::provision::Provision;
//    use solace_semp_client::models::MsgVpnQueue;
//    use tokio_core::reactor::Core;
//    use hyper::client::HttpConnector;
//    use native_tls::TlsConnector;
//    use hyper::Client;
//    use crate::helpers;
//    use solace_semp_client::apis::configuration::Configuration;
//    use solace_semp_client::apis::client::APIClient;
//    use std::error::Error;
//
//    use crate::update::Update;
//
//    //-> Result<(), Box<Error>>
//    #[test]
//    fn provision() {
//        println!("provision tests");
//
//        // configure the http client
//        let mut core = Core::new().unwrap();
//        let handle = core.handle();
//
//        let mut http = HttpConnector::new(4, &handle);
//        http.enforce_http(false);
//
//        let mut tls = TlsConnector::builder().unwrap();
//
//        let hyperclient = Client::configure()
//            .connector(hyper_tls::HttpsConnector::from((http, tls.build().unwrap()))).build(&handle);
//
//        let auth = helpers::gencred("admin".to_owned(), "admin".to_owned());
//
//        // the configuration for the APIClient
//        let mut configuration = Configuration {
//            base_path: "http://localhost:8081/SEMP/v2/config".to_owned(),
//            user_agent: Some("solace-provision".to_owned()),
//            client: hyperclient,
//            basic_auth: Some(auth),
//            oauth_access_token: None,
//            api_key: None,
//        };
//
//        let client = APIClient::new(configuration);
//
//        println!("create vpn");
//
//        let v = MsgVpnResponse::provision_with_file("testvpn",
//                                                    "",
//                                                    "examples/vpn.yaml", &mut core,
//                                                    &client);
//
//        println!("create queue");
//
//        let q = MsgVpnQueueResponse::provision_with_file("testvpn",
//                                                         " queue1",
//                                                         "examples/queue1.yaml", &mut core,
//                                                         &client);
//
//        println!("create acl");
//
//        let a = MsgVpnAclProfileResponse::provision_with_file("testvpn",
//                                                              "myacl",
//                                                              "examples/acl.yaml", &mut core,
//                                                              &client);
//
//        println!("create acl publish exception");
//        let ape = MsgVpnAclProfilePublishException::provision_with_file("testvpn",
//                                                              "myacl",
//                                                              "examples/acl-pub-exception.yaml", &mut core,
//                                                              &client);
//
//
//
//
//        println!("create client profile");
//
//        let cp = MsgVpnClientProfileResponse::provision_with_file("testvpn",
//                                                                  "myclientprofile",
//                                                                  "examples/client-profile.yaml",
//                                                                  &mut core, &client);
//
//
//        println!("create client username");
//
//        let cu = MsgVpnClientUsernameResponse::provision_with_file("testvpn",
//                                                                   "myusername",
//                                                                   "examples/client-username.yaml",
//                                                                   &mut core, &client);
//
//
//        println!("create queue subscription");
//        let qs = MsgVpnQueueSubscriptionResponse::provision_with_file("testvpn",
//                                                                      "queue1",
//                                                                      "examples/queue-subscription.yaml",
//                                                                      &mut core, &client);
//
//        println!("create sequenced topic");
//        let st = MsgVpnSequencedTopicResponse::provision_with_file("testvpn",
//                                                                   "",
//                                                                   "examples/sequenced-topic.yaml",
//                                                                   &mut core, &client);
//
//        println!("create topic endpoint");
//        let te = MsgVpnTopicEndpointResponse::provision_with_file("testvpn",
//                                                                  "",
//                                                                  "examples/topicendpoint.yaml",
//                                                                  &mut core, &client);
//
//        println!("create auth group");
//        let ag = MsgVpnAuthorizationGroupResponse::provision_with_file("testvpn",
//                                                                       "",
//                                                                       "examples/authgroup.yaml",
//                                                                       &mut core, &client);
//
//        println!("create bridge");
//        let bp = MsgVpnBridgeResponse::provision_with_file("testvpn",
//                                                           "mybridge",
//                                                           "examples/bridge-primary.yaml", &mut core,
//                                                           &client);
//
//        println!("create remote bridge");
//        let br = MsgVpnBridgeRemoteMsgVpnResponse::provision_with_file("testvpn",
//                                                                       "mybridge",
//                                                                       "examples/bridge-remote-primary.yaml",
//                                                                       &mut core, &client);
//
//        println!("create remote bridge subscription");
//        let rbs = MsgVpnBridgeRemoteSubscriptionResponse::provision_with_file("testvpn",
//                                                                              "mybridge",
//                                                                              "examples/bridge-remote-subscription.yaml",
//                                                                              &mut core, &client);
//
//        println!("create replay-log");
//        let rpl = MsgVpnReplayLogResponse::provision_with_file("testvpn",
//                                                               "myreplaylog",
//                                                               "examples/replay.yaml",
//                                                               &mut core, &client);
//
//
//        match v {
//            Ok(vpn) => {
//                assert_eq!(vpn.data().unwrap().msg_vpn_name().unwrap(), "testvpn");
//            },
//            Err(e) => {
//                error!("cannot test");
//            }
//        }
//
//        match q {
//            Ok(queue) => {
//                assert_eq!(queue.data().unwrap().queue_name().unwrap(), "queue1");
//            },
//            Err(e) => {
//                error!("cannot test");
//            }
//        }
//
//
//    }
//}



use solace_semp_client::apis::client::APIClient;
use solace_semp_client::models::{MsgVpn, MsgVpnQueueSubscription, MsgVpnQueueSubscriptionsResponse, MsgVpnQueueSubscriptionResponse, MsgVpnSequencedTopicResponse, MsgVpnSequencedTopic, MsgVpnTopicEndpointsResponse, MsgVpnTopicEndpointResponse, MsgVpnTopicEndpoint, MsgVpnAuthorizationGroupResponse, MsgVpnAuthorizationGroup, MsgVpnBridgeResponse, MsgVpnBridgeRemoteMsgVpnResponse, MsgVpnBridgeRemoteMsgVpn, MsgVpnBridgeRemoteSubscriptionResponse, MsgVpnBridgeRemoteSubscription, MsgVpnReplayLogResponse, MsgVpnReplayLogsResponse, MsgVpnReplayLog, MsgVpnDmrBridgeResponse, MsgVpnDmrBridge, DmrClusterResponse, DmrCluster, DmrClusterLinkResponse, DmrClusterLink, MsgVpnAclProfilePublishExceptionResponse, MsgVpnAclProfilePublishException, MsgVpnAclProfileSubscribeExceptionResponse, MsgVpnAclProfileSubscribeException};
use tokio_core::reactor::Core;
use hyper_tls::HttpsConnector;
use hyper::client::HttpConnector;
use solace_semp_client::models::MsgVpnBridge;
use solace_semp_client::models::MsgVpnQueue;
use solace_semp_client::models::MsgVpnAclProfile;
use serde::Serialize;
use colored::Colorize;
use futures::future::Future;
use futures::future::AndThen;
use std::fs::File;
use serde::Deserialize;
use std::io::Error;
use crate::helpers;
use std::io;
use solace_semp_client::models::MsgVpnResponse;
use solace_semp_client::models::MsgVpnQueueResponse;
use solace_semp_client::models::MsgVpnAclProfilesResponse;
use solace_semp_client::models::MsgVpnAclProfileResponse;
use solace_semp_client::models::MsgVpnClientProfileResponse;
use solace_semp_client::models::MsgVpnClientProfile;
use solace_semp_client::models::MsgVpnClientUsernameResponse;
use solace_semp_client::models::MsgVpnClientUsername;
use std::process::exit;
use crate::helpers::getselect;


pub trait Provision<T> {
    fn provision_with_file(in_vpn: &str, item_name: &str, file_name: &str, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<T, &'static str> {
        unimplemented!()
    }
    fn provision_item_subittem(in_vpn: &str, item_name: &str, second_item_name: &str, file_name: &str, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<T, &'static str> {
        unimplemented!()
    }
//    fn deserialize_file(file_name: &str) -> Result<Option<T>,&'static str> where T:Deserialize {
//        let file = std::fs::File::open(file_name).unwrap();
//        let deserialized: Option<T> = serde_yaml::from_reader(file).unwrap();
//        return Ok(deserialized);
//    }
}



// DMR cluster



// DMR cluster link

