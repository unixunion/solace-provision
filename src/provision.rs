
mod tests {

    use solace_semp_client::models::{MsgVpn, MsgVpnResponse, MsgVpnQueueResponse, MsgVpnAclProfileResponse, MsgVpnClientProfileResponse, MsgVpnClientUsernameResponse, MsgVpnQueueSubscriptionResponse, MsgVpnSequencedTopicResponse, MsgVpnTopicEndpointResponse, MsgVpnAuthorizationGroupResponse, MsgVpnBridgeResponse, MsgVpnBridgeRemoteMsgVpnResponse, MsgVpnBridgeRemoteSubscriptionResponse};
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

    //-> Result<(), Box<Error>>
    #[test]
    fn provision() {
        println!("provision tests");

        // configure the http client
        let mut core = Core::new().unwrap();
        let handle = core.handle();

        let mut http = HttpConnector::new(4, &handle);
        http.enforce_http(false);

        let mut tls = TlsConnector::builder().unwrap();

        let hyperclient = Client::configure()
            .connector(hyper_tls::HttpsConnector::from((http, tls.build().unwrap()))).build(&handle);

        let auth = helpers::gencred("admin".to_owned(), "admin".to_owned());

        // the configuration for the APIClient
        let mut configuration = Configuration {
            base_path: "http://localhost:8081/SEMP/v2/config".to_owned(),
            user_agent: Some("solace-provision".to_owned()),
            client: hyperclient,
            basic_auth: Some(auth),
            oauth_access_token: None,
            api_key: None,
        };

        let client = APIClient::new(configuration);

        println!("create vpn");

        let v = MsgVpnResponse::provision("testvpn",
                                          "",
                                          "examples/vpn.yaml", &mut core,
                                          &client);

        println!("create queue");

        let q = MsgVpnQueueResponse::provision("testvpn",
                                               " queue1",
                                               "examples/queue1.yaml", &mut core,
                                               &client);

        println!("create acl");

        let a = MsgVpnAclProfileResponse::provision("testvpn",
                                                    "myacl",
                                                    "examples/acl.yaml", &mut core,
                                                    &client);

        println!("create client profile");

        let cp = MsgVpnClientProfileResponse::provision("testvpn",
                                                        "myclientprofile",
                                                        "examples/client-profile.yaml",
                                                        &mut core, &client);


        println!("create client username");

        let cu = MsgVpnClientUsernameResponse::provision("testvpn",
                                                         "myusername",
                                                         "examples/client-username.yaml",
                                                         &mut core, &client);


        println!("create queue subscription");
        let qs = MsgVpnQueueSubscriptionResponse::provision("testvpn",
                                                            "queue1",
                                                            "examples/queue-subscription.yaml",
                                                            &mut core, &client);

        println!("create sequenced topic");
        let st = MsgVpnSequencedTopicResponse::provision("testvpn",
                                                         "",
                                                         "examples/sequenced-topic.yaml",
                                                         &mut core, &client);

        println!("create topic endpoint");
        let te = MsgVpnTopicEndpointResponse::provision("testvpn",
                                                        "",
                                                        "examples/topicendpoint.yaml",
                                                        &mut core, &client);

        println!("create auth group");
        let ag = MsgVpnAuthorizationGroupResponse::provision("testvpn",
                                                             "",
                                                             "examples/authgroup.yaml",
                                                             &mut core, &client);

        println!("create bridge");
        let bp = MsgVpnBridgeResponse::provision("testvpn",
                                                 "mybridge",
                                                 "examples/bridge-primary.yaml", &mut core,
                                                 &client);

        println!("create remote bridge");
        let br = MsgVpnBridgeRemoteMsgVpnResponse::provision("testvpn",
                                                             "mybridge",
                                                             "examples/bridge-remote-primary.yaml",
                                                             &mut core, &client);

        println!("create remote bridge subscription");
        let rbs = MsgVpnBridgeRemoteSubscriptionResponse::provision("testvpn",
                                                            "mybridge",
                                                            "examples/bridge-remote-subscription.yaml",
                                                            &mut core, &client);


        match v {
            Ok(vpn) => {
                assert_eq!(vpn.data().unwrap().msg_vpn_name().unwrap(), "testvpn");
            },
            Err(e) => {
                error!("cannot test");
            }
        }

        match q {
            Ok(queue) => {
                assert_eq!(queue.data().unwrap().queue_name().unwrap(), "queue1");
            },
            Err(e) => {
                error!("cannot test");
            }
        }


    }
}



use solace_semp_client::apis::client::APIClient;
use solace_semp_client::models::{MsgVpn, MsgVpnQueueSubscription, MsgVpnQueueSubscriptionsResponse, MsgVpnQueueSubscriptionResponse, MsgVpnSequencedTopicResponse, MsgVpnSequencedTopic, MsgVpnTopicEndpointsResponse, MsgVpnTopicEndpointResponse, MsgVpnTopicEndpoint, MsgVpnAuthorizationGroupResponse, MsgVpnAuthorizationGroup, MsgVpnBridgeResponse, MsgVpnBridgeRemoteMsgVpnResponse, MsgVpnBridgeRemoteMsgVpn, MsgVpnBridgeRemoteSubscriptionResponse, MsgVpnBridgeRemoteSubscription};
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
    fn provision(in_vpn: &str, item_name: &str, file_name: &str, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<T, &'static str>;
}

impl Provision<MsgVpnResponse> for MsgVpnResponse {

    fn provision(in_vpn: &str, item_name: &str, file_name: &str, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<MsgVpnResponse, &'static str> {
        let file = std::fs::File::open(file_name).unwrap();
        let deserialized: Option<MsgVpn> = serde_yaml::from_reader(file).unwrap();
        match deserialized {
            Some(mut item) => {
                item.set_msg_vpn_name(in_vpn.to_owned());
                let request = apiclient
                    .default_api()
                    .create_msg_vpn(item, getselect("*"));
                match core.run(request) {
                    Ok(response) => {
                        info!("{}",format!("{}", serde_yaml::to_string(&response.data().unwrap()).unwrap()));
                        Ok(response)
                    },
                    Err(e) => {
                        println!("provision error: {:?}", e);
                        exit(126);
                        Err("provision error")
                    }
                }
            }
            _ => unimplemented!()
        }
    }
}

impl Provision<MsgVpnQueueResponse> for MsgVpnQueueResponse {

    fn provision(in_vpn: &str, item_name: &str, file_name: &str, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<MsgVpnQueueResponse, &'static str> {
        let file = std::fs::File::open(file_name).unwrap();
        let deserialized: Option<MsgVpnQueue> = serde_yaml::from_reader(file).unwrap();
        match deserialized {
            Some(mut item) => {
                item.set_msg_vpn_name(in_vpn.to_owned());
                let request = apiclient
                    .default_api()
                    .create_msg_vpn_queue(in_vpn, item, getselect("*"));
                match core.run(request) {
                    Ok(response) => {
                        info!("{}",format!("{}", serde_yaml::to_string(&response.data().unwrap()).unwrap()));
                        Ok(response)
                    },
                    Err(e) => {
                        println!("provision error: {:?}", e);
                        exit(126);
                        Err("provision error")
                    }
                }
            }
            _ => unimplemented!()
        }
    }
}

impl Provision<MsgVpnAclProfileResponse> for MsgVpnAclProfileResponse {

    fn provision(in_vpn: &str, item_name: &str, file_name: &str, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<MsgVpnAclProfileResponse, &'static str> {
        let file = std::fs::File::open(file_name).unwrap();
        let deserialized: Option<MsgVpnAclProfile> = serde_yaml::from_reader(file).unwrap();
        match deserialized {
            Some(mut item) => {
                item.set_msg_vpn_name(in_vpn.to_owned());
                let request = apiclient
                    .default_api()
                    .create_msg_vpn_acl_profile(in_vpn, item, getselect("*"));
                match core.run(request) {
                    Ok(response) => {
                        info!("{}",format!("{}", serde_yaml::to_string(&response.data().unwrap()).unwrap()));
                        Ok(response)
                    },
                    Err(e) => {
                        println!("provision error: {:?}", e);
                        exit(126);
                        Err("provision error")
                    }
                }
            }
            _ => unimplemented!()
        }
    }
}

impl Provision<MsgVpnClientProfileResponse> for MsgVpnClientProfileResponse {

    fn provision(in_vpn: &str, item_name: &str, file_name: &str, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<MsgVpnClientProfileResponse, &'static str> {
        let file = std::fs::File::open(file_name).unwrap();
        let deserialized: Option<MsgVpnClientProfile> = serde_yaml::from_reader(file).unwrap();
        match deserialized {
            Some(mut item) => {
                item.set_msg_vpn_name(in_vpn.to_owned());
                let request = apiclient
                    .default_api()
                    .create_msg_vpn_client_profile(in_vpn, item, getselect("*"));
                match core.run(request) {
                    Ok(response) => {
                        info!("{}",format!("{}", serde_yaml::to_string(&response.data().unwrap()).unwrap()));
                        Ok(response)
                    },
                    Err(e) => {
                        println!("provision error: {:?}", e);
                        exit(126);
                        Err("provision error")
                    }
                }
            }
            _ => unimplemented!()
        }
    }
}

impl Provision<MsgVpnClientUsernameResponse> for MsgVpnClientUsernameResponse {

    fn provision(in_vpn: &str, item_name: &str, file_name: &str, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<MsgVpnClientUsernameResponse, &'static str> {
        let file = std::fs::File::open(file_name).unwrap();
        let deserialized: Option<MsgVpnClientUsername> = serde_yaml::from_reader(file).unwrap();
        match deserialized {
            Some(mut item) => {
                item.set_msg_vpn_name(in_vpn.to_owned());
                let request = apiclient
                    .default_api()
                    .create_msg_vpn_client_username(in_vpn, item, getselect("*"));
                match core.run(request) {
                    Ok(response) => {
                        info!("{}",format!("{}", serde_yaml::to_string(&response.data().unwrap()).unwrap()));
                        Ok(response)
                    },
                    Err(e) => {
                        println!("provision error: {:?}", e);
                        exit(126);
                        Err("provision error")
                    }
                }
            }
            _ => unimplemented!()
        }
    }
}

impl Provision<MsgVpnQueueSubscriptionResponse> for MsgVpnQueueSubscriptionResponse {

    fn provision(in_vpn: &str, queue_name: &str, file_name: &str, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<MsgVpnQueueSubscriptionResponse, &'static str> {
        let file = std::fs::File::open(file_name).unwrap();
        let deserialized: Option<MsgVpnQueueSubscription> = serde_yaml::from_reader(file).unwrap();
        match deserialized {
            Some(mut item) => {
                item.set_msg_vpn_name(in_vpn.to_owned());
                let request = apiclient
                    .default_api()
                    .create_msg_vpn_queue_subscription(in_vpn, queue_name, item, getselect("*"));
                match core.run(request) {
                    Ok(response) => {
                        info!("{}",format!("{}", serde_yaml::to_string(&response.data().unwrap()).unwrap()));
                        Ok(response)
                    },
                    Err(e) => {
                        println!("provision error: {:?}", e);
                        exit(126);
                        Err("provision error")
                    }
                }
            }
            _ => unimplemented!()
        }
    }
}

// sequenced topic
impl Provision<MsgVpnSequencedTopicResponse> for MsgVpnSequencedTopicResponse {

    fn provision(in_vpn: &str, item_name: &str, file_name: &str, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<MsgVpnSequencedTopicResponse, &'static str> {
        let file = std::fs::File::open(file_name).unwrap();
        let deserialized: Option<MsgVpnSequencedTopic> = serde_yaml::from_reader(file).unwrap();
        match deserialized {
            Some(mut item) => {
                item.set_msg_vpn_name(in_vpn.to_owned());
                let request = apiclient
                    .default_api()
                    .create_msg_vpn_sequenced_topic(in_vpn, item, getselect("*"));
                match core.run(request) {
                    Ok(response) => {
                        info!("{}",format!("{}", serde_yaml::to_string(&response.data().unwrap()).unwrap()));
                        Ok(response)
                    },
                    Err(e) => {
                        println!("provision error: {:?}", e);
                        exit(126);
                        Err("provision error")
                    }
                }
            }
            _ => unimplemented!()
        }
    }
}

// topic endpoint
impl Provision<MsgVpnTopicEndpointResponse> for MsgVpnTopicEndpointResponse {

    fn provision(in_vpn: &str, item_name: &str, file_name: &str, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<MsgVpnTopicEndpointResponse, &'static str> {
        let file = std::fs::File::open(file_name).unwrap();
        let deserialized: Option<MsgVpnTopicEndpoint> = serde_yaml::from_reader(file).unwrap();
        match deserialized {
            Some(mut item) => {
                item.set_msg_vpn_name(in_vpn.to_owned());
                let request = apiclient
                    .default_api()
                    .create_msg_vpn_topic_endpoint(in_vpn, item, getselect("*"));
                match core.run(request) {
                    Ok(response) => {
                        info!("{}",format!("{}", serde_yaml::to_string(&response.data().unwrap()).unwrap()));
                        Ok(response)
                    },
                    Err(e) => {
                        println!("provision error: {:?}", e);
                        exit(126);
                        Err("provision error")
                    }
                }
            }
            _ => unimplemented!()
        }
    }
}

// authorization group
impl Provision<MsgVpnAuthorizationGroupResponse> for MsgVpnAuthorizationGroupResponse {

    fn provision(in_vpn: &str, item_name: &str, file_name: &str, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<MsgVpnAuthorizationGroupResponse, &'static str> {
        let file = std::fs::File::open(file_name).unwrap();
        let deserialized: Option<MsgVpnAuthorizationGroup> = serde_yaml::from_reader(file).unwrap();
        match deserialized {
            Some(mut item) => {
                item.set_msg_vpn_name(in_vpn.to_owned());
                let request = apiclient
                    .default_api()
                    .create_msg_vpn_authorization_group(in_vpn, item, getselect("*"));
                match core.run(request) {
                    Ok(response) => {
                        info!("{}",format!("{}", serde_yaml::to_string(&response.data().unwrap()).unwrap()));
                        Ok(response)
                    },
                    Err(e) => {
                        println!("provision error: {:?}", e);
                        exit(126);
                        Err("provision error")
                    }
                }
            }
            _ => unimplemented!()
        }
    }
}

// bridge

impl Provision<MsgVpnBridgeResponse> for MsgVpnBridgeResponse {

    fn provision(in_vpn: &str, item_name: &str, file_name: &str, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<MsgVpnBridgeResponse, &'static str> {
        let file = std::fs::File::open(file_name).unwrap();
        let deserialized: Option<MsgVpnBridge> = serde_yaml::from_reader(file).unwrap();
        match deserialized {
            Some(mut item) => {
                item.set_msg_vpn_name(in_vpn.to_owned());
                let request = apiclient
                    .default_api()
                    .create_msg_vpn_bridge(in_vpn, item, getselect("*"));
                match core.run(request) {
                    Ok(response) => {
                        info!("{}",format!("{}", serde_yaml::to_string(&response.data().unwrap()).unwrap()));
                        Ok(response)
                    },
                    Err(e) => {
                        println!("provision error: {:?}", e);
                        exit(126);
                        Err("provision error")
                    }
                }
            }
            _ => unimplemented!()
        }
    }
}

// remote bridge

impl Provision<MsgVpnBridgeRemoteMsgVpnResponse> for MsgVpnBridgeRemoteMsgVpnResponse {

    fn provision(in_vpn: &str, bridge_name: &str, file_name: &str, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<MsgVpnBridgeRemoteMsgVpnResponse, &'static str> {
        let file = std::fs::File::open(file_name).unwrap();
        let deserialized: Option<MsgVpnBridgeRemoteMsgVpn> = serde_yaml::from_reader(file).unwrap();
        match deserialized {
            Some(mut item) => {
                item.set_msg_vpn_name(in_vpn.to_owned());
                let virtual_router = &*item.bridge_virtual_router().cloned().unwrap();
                let request = apiclient
                    .default_api()
                    .create_msg_vpn_bridge_remote_msg_vpn(in_vpn, bridge_name, virtual_router, item, getselect("*"));
                match core.run(request) {
                    Ok(response) => {
                        info!("{}",format!("{}", serde_yaml::to_string(&response.data().unwrap()).unwrap()));
                        Ok(response)
                    },
                    Err(e) => {
                        println!("provision error: {:?}", e);
                        exit(126);
                        Err("provision error")
                    }
                }
            }
            _ => unimplemented!()
        }
    }
}

// remote bridge subscriptions

impl Provision<MsgVpnBridgeRemoteSubscriptionResponse> for MsgVpnBridgeRemoteSubscriptionResponse {

    fn provision(in_vpn: &str, bridge_name: &str, file_name: &str, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<MsgVpnBridgeRemoteSubscriptionResponse, &'static str> {
        let file = std::fs::File::open(file_name).unwrap();
        let deserialized: Option<MsgVpnBridgeRemoteSubscription> = serde_yaml::from_reader(file).unwrap();
        match deserialized {
            Some(mut item) => {
                item.set_msg_vpn_name(in_vpn.to_owned());
                let virtual_router = &*item.bridge_virtual_router().cloned().unwrap();
                let request = apiclient
                    .default_api()
                    .create_msg_vpn_bridge_remote_subscription(in_vpn, bridge_name, virtual_router, item, getselect("*"));
                match core.run(request) {
                    Ok(response) => {
                        info!("{}",format!("{}", serde_yaml::to_string(&response.data().unwrap()).unwrap()));
                        Ok(response)
                    },
                    Err(e) => {
                        println!("provision error: {:?}", e);
                        exit(126);
                        Err("provision error")
                    }
                }
            }
            _ => unimplemented!()
        }
    }
}