
mod tests {

    use solace_semp_client::models::MsgVpn;
    use crate::update::Update;
    use solace_semp_client::models::MsgVpnQueue;

    #[test]
    fn it_works() {}
}


use std::process;
use solace_semp_client::apis::client::APIClient;
use solace_semp_client::models::{MsgVpn, MsgVpnQueueSubscriptionResponse, MsgVpnSequencedTopicResponse, MsgVpnTopicEndpointResponse, MsgVpnTopicEndpointsResponse, MsgVpnAuthorizationGroupResponse, MsgVpnAuthorizationGroup, MsgVpnAuthorizationGroupsResponse, MsgVpnBridgeResponse, MsgVpnBridgesResponse, MsgVpnBridgeRemoteMsgVpnsResponse, MsgVpnBridgeRemoteMsgVpn, MsgVpnBridgeRemoteMsgVpnResponse};
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
use log::{info, warn, error, debug};
use serde::Deserialize;
use std::io::Error;
use solace_semp_client::models::MsgVpnResponse;
use crate::fetch;
use solace_semp_client::models::MsgVpnsResponse;
use std::mem::size_of;
use crate::fetch::Fetch;
use std::process::exit;
use solace_semp_client::models::MsgVpnQueueResponse;
use solace_semp_client::models::MsgVpnQueuesResponse;
use solace_semp_client::models::MsgVpnAclProfileResponse;
use solace_semp_client::models::MsgVpnAclProfilesResponse;
use solace_semp_client::models::MsgVpnClientProfileResponse;
use solace_semp_client::models::MsgVpnClientProfile;
use solace_semp_client::models::MsgVpnClientProfilesResponse;
use solace_semp_client::models::MsgVpnClientUsernameResponse;
use solace_semp_client::models::MsgVpnClientUsername;
use solace_semp_client::models::MsgVpnClientUsernamesResponse;
use crate::helpers::getselect;

// shared base trait for all solace update-able objects
pub trait Update<T> {
    // update a object, shutting it down first if shutdown is true
    fn update(msg_vpn: &str, sub_item: &str, file_name: &str, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<(), &'static str>;
    // change the enabled state fo a object
    fn enabled(msg_vpn: &str, item_name: &str, selector: Vec<&str>, state: bool, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<(), &'static str>;
    // ingress
    fn ingress(msg_vpn: &str, item_name: &str, state: bool, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<(), &'static str>;
    fn egress(msg_vpn: &str, item_name: &str, state: bool, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<(), &'static str>;
    // delete object
    fn delete(msg_vpn: &str, item_name: &str, sub_identifier: &str, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<(), &'static str>;
}

impl Update<MsgVpnResponse> for MsgVpnResponse {

    fn update(msg_vpn: &str, file_name: &str, sub_item: &str, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<(), &'static str> {
        info!("updating message-vpn: {} from file", msg_vpn);
        let file = std::fs::File::open(file_name).unwrap();
        let deserialized: Option<MsgVpn> = serde_yaml::from_reader(file).unwrap();

        match deserialized {
            Some(mut item) => {
                item.set_msg_vpn_name(msg_vpn.to_owned());
                let request = apiclient
                    .default_api()
                    .update_msg_vpn(msg_vpn, item, getselect("*"));
                match core.run(request) {
                    Ok(response) => {
                        info!("{}",format!("{}", serde_yaml::to_string(&response.data().unwrap()).unwrap()));
                        Ok(())
                    },
                    Err(e) => {
                        error!("update error: {:?}", e);
                        process::exit(126);
                        Err("update error")
                    }
                }
            }
            _ => unimplemented!()
        }
    }

    fn enabled(msg_vpn: &str, item_name: &str, selector: Vec<&str>, state: bool, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<(), &'static str> {
        info!("changing enabled state to: {:?} for message-vpn: {}", state, msg_vpn);
        let mut vpn = MsgVpnsResponse::fetch(item_name, item_name, "msgVpnName",item_name, 10, "", "", core, apiclient)?;

        let mut tvpn = vpn.data().unwrap().clone();
        if tvpn.len() == 1 {
            info!("changing enabled state to: {}", state.to_string());
            let mut x = tvpn.pop().unwrap();
            x.set_enabled(state);
            let r = core.run(apiclient.default_api().update_msg_vpn(msg_vpn, x, getselect("*")));
            match r {
                Ok(t) => info!("state successfully changed to {:?}", state),
                Err(e) => {
                    error!("error changing enabled state for vpn: {}, {:?}", item_name, e);
                    exit(126);
//                    Err("err")
                }
            }
        } else {
            error!("error, did not find exactly one item matching query");
            exit(126);
//            Err("woops")
        }


        Ok(())

    }

    fn ingress(msg_vpn: &str, item_name: &str, state: bool, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<(), &'static str> {
        unimplemented!()
    }

    fn egress(msg_vpn: &str, item_name: &str, state: bool, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<(), &'static str> {
        unimplemented!()
    }

    fn delete(msg_vpn: &str, item_name: &str, sub_identifier: &str,  core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<(), &'static str> {
        let t = apiclient.default_api().delete_msg_vpn(item_name);
        match core.run(t) {
            Ok(vpn) => {
                info!("vpn deleted");
                Ok(())
            },
            Err(e) => {
                error!("unable to delete vpn: {:?}", e);
//                process::exit(126);
                Err("unable to delete vpn")
            }
        }
    }

}

impl Update<MsgVpnQueueResponse> for MsgVpnQueueResponse {

    fn update(vpn_name: &str, file_name: &str, sub_item: &str, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<(), &'static str> {
        let file = std::fs::File::open(file_name).unwrap();
        let deserialized: Option<MsgVpnQueue> = serde_yaml::from_reader(file).unwrap();

        match deserialized {
            Some(mut item) => {
                item.set_msg_vpn_name(vpn_name.to_owned());
                let item_name = item.queue_name().cloned();
                let request = apiclient
                    .default_api()
                    .update_msg_vpn_queue(vpn_name, &*item_name.unwrap(), item, getselect("*"));
                match core.run(request) {
                    Ok(response) => {
                        info!("{}",format!("{}", serde_yaml::to_string(&response.data().unwrap()).unwrap()));
                        Ok(())
                    },
                    Err(e) => {
                        error!("update error: {:?}", e);
                        process::exit(126);
                        Err("update error")
                    }
                }
            }
            _ => unimplemented!()
        }
    }

    fn enabled(vpn_name: &str, item_name: &str, selector: Vec<&str>, state: bool, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<(), &'static str> {
        unimplemented!()
    }

    fn ingress(msg_vpn: &str, item_name: &str, state: bool, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<(), &'static str> {
        info!("retrieving current queue from appliance");
        let mut item = MsgVpnQueuesResponse::fetch(msg_vpn, item_name, "queueName",item_name, 10, "", "", core, apiclient)?;
        let mut titem = item.data().unwrap().clone();

        if titem.len() == 1 {
            info!("changing ingress state to: {}", state.to_string());
            let mut x = titem.pop().unwrap();
            x.set_ingress_enabled(state);
            let r = core.run(apiclient.default_api().update_msg_vpn_queue(msg_vpn, item_name, x, getselect("*")));
            match r {
                Ok(t) => info!("ingress successfully changed to {:?}", state),
                Err(e) => {
                    error!("error changing ingress state for vpn: {}, {:?}", item_name, e);
                    exit(126);
                }
            }

        } else {
            error!("error, did not find exactly one item matching query");
            process::exit(126);
        }

        Ok(())
    }

    fn egress(msg_vpn: &str, item_name: &str, state: bool, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<(), &'static str> {
        info!("retrieving current queue from appliance");
        let mut item = MsgVpnQueuesResponse::fetch(msg_vpn, item_name, "queueName",item_name, 10, "", "", core, apiclient)?;
        let mut titem = item.data().unwrap().clone();

        if titem.len() == 1 {
            info!("changing egress state to: {}", state.to_string());
            let mut x = titem.pop().unwrap();
            x.set_egress_enabled(state);
            let r = core.run(apiclient.default_api().update_msg_vpn_queue(msg_vpn, item_name, x, getselect("*")));
            match r {
                Ok(t) => info!("egress successfully changed to {:?}", state),
                Err(e) => {
                    error!("error changing egress state for vpn: {}, {:?}", item_name, e);
                    exit(126);
                }
            }

        } else {
            error!("error, did not find exactly one item matching query");
            process::exit(126);
        }

        Ok(())
    }

    fn delete(msg_vpn: &str, item_name: &str, sub_identifier: &str, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<(), &'static str> {
        let t = apiclient.default_api().delete_msg_vpn_queue(msg_vpn, item_name);
        match core.run(t) {
            Ok(vpn) => {
                info!("queue deleted");
                Ok(())
            },
            Err(e) => {
                error!("unable to delete queue: {:?}", e);
//                process::exit(126);
                Err("unable to delete queue")
            }
        }
    }
}

impl Update<MsgVpnAclProfileResponse> for MsgVpnAclProfileResponse {

    fn update(msg_vpn: &str, file_name: &str, sub_item: &str, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<(), &'static str> {
        let file = std::fs::File::open(file_name).unwrap();
        let deserialized: Option<MsgVpnAclProfile> = serde_yaml::from_reader(file).unwrap();

        match deserialized {
            Some(mut item) => {
                item.set_msg_vpn_name(msg_vpn.to_owned());
                let item_name = item.acl_profile_name().cloned();
                let request = apiclient
                    .default_api()
                    .update_msg_vpn_acl_profile(msg_vpn, &*item_name.unwrap(), item, getselect("*"));
                match core.run(request) {
                    Ok(response) => {
                        info!("{}",format!("{}", serde_yaml::to_string(&response.data().unwrap()).unwrap()));
                        Ok(())
                    },
                    Err(e) => {
                        error!("update error: {:?}", e);
                        process::exit(126);
                        Err("update error")
                    }
                }
            }
            _ => unimplemented!()
        }
    }

    fn enabled(msg_vpn: &str, item_name: &str, selector: Vec<&str>, state: bool, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<(), &'static str> {
        unimplemented!()
    }

    fn ingress(msg_vpn: &str, item_name: &str, state: bool, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<(), &'static str> {
        unimplemented!()
    }

    fn egress(msg_vpn: &str, item_name: &str, state: bool, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<(), &'static str> {
        unimplemented!()
    }

    fn delete(msg_vpn: &str, item_name: &str, sub_identifier: &str, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<(), &'static str> {
        let t = apiclient.default_api().delete_msg_vpn_acl_profile(msg_vpn, item_name);
        match core.run(t) {
            Ok(vpn) => {
                info!("acl deleted");
                Ok(())
            },
            Err(e) => {
                error!("unable to delete acl: {:?}", e);
//                process::exit(126);
                Err("unable to delete acl")
            }
        }
    }
}


// client-profile

impl Update<MsgVpnClientProfileResponse> for MsgVpnClientProfileResponse {

    fn update(msg_vpn: &str, file_name: &str, sub_item: &str, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<(), &'static str> {
        let file = std::fs::File::open(file_name).unwrap();
        let deserialized: Option<MsgVpnClientProfile> = serde_yaml::from_reader(file).unwrap();

        match deserialized {
            Some(mut item) => {
                item.set_msg_vpn_name(msg_vpn.to_owned());
                let item_name = item.client_profile_name().cloned();
                let request = apiclient
                    .default_api()
                    .update_msg_vpn_client_profile(msg_vpn, &*item_name.unwrap(), item, getselect("*"));
                match core.run(request) {
                    Ok(response) => {
                        info!("{}",format!("{}", serde_yaml::to_string(&response.data().unwrap()).unwrap()));
                        Ok(())
                    },
                    Err(e) => {
                        error!("update error: {:?}", e);
                        process::exit(126);
                        Err("update error")
                    }
                }
            }
            _ => unimplemented!()
        }
    }

    fn enabled(msg_vpn: &str, item_name: &str, selector: Vec<&str>, state: bool, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<(), &'static str> {
        unimplemented!()
    }

    fn ingress(msg_vpn: &str, item_name: &str, state: bool, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<(), &'static str> {
        unimplemented!()
    }

    fn egress(msg_vpn: &str, item_name: &str, state: bool, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<(), &'static str> {
        unimplemented!()
    }

    fn delete(msg_vpn: &str, item_name: &str, sub_identifier: &str, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<(), &'static str> {
        let t = apiclient.default_api().delete_msg_vpn_client_profile(msg_vpn, item_name);
        match core.run(t) {
            Ok(vpn) => {
                info!("client-profile deleted");
                Ok(())
            },
            Err(e) => {
                error!("unable to delete client-profile: {:?}", e);
//                process::exit(126);
                Err("unable to delete client-profile")
            }
        }
    }
}



impl Update<MsgVpnClientUsernameResponse> for MsgVpnClientUsernameResponse {

    fn update(msg_vpn: &str, file_name: &str, sub_item: &str, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<(), &'static str> {
        let file = std::fs::File::open(file_name).unwrap();
        let deserialized: Option<MsgVpnClientUsername> = serde_yaml::from_reader(file).unwrap();

        match deserialized {
            Some(mut item) => {
                item.set_msg_vpn_name(msg_vpn.to_owned());
                let item_name = item.client_username().cloned();
                let request = apiclient
                    .default_api()
                    .update_msg_vpn_client_username(msg_vpn, &*item_name.unwrap(), item, getselect("*"));
                match core.run(request) {
                    Ok(response) => {
                        info!("{}",format!("{}", serde_yaml::to_string(&response.data().unwrap()).unwrap()));
                        Ok(())
                    },
                    Err(e) => {
                        error!("update error: {:?}", e);
                        process::exit(126);
                        Err("update error")
                    }
                }
            }
            _ => unimplemented!()
        }
    }

    fn enabled(msg_vpn: &str, client_username: &str, selector: Vec<&str>, state: bool, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<(), &'static str> {
        println!("retrieving current client-username from appliance");
        let mut item = MsgVpnClientUsernamesResponse::fetch(msg_vpn, client_username, "clientUsername",client_username, 10, "", "", core, apiclient)?;
        let mut titem = item.data().unwrap().clone();

        if titem.len() == 1 {
            println!("changing enabled state to: {}", state.to_string());
            let mut x = titem.pop().unwrap();
//            let client_username = x.client_username().clone().unwrap();
            x.set_enabled(state);
            // this sets the password to None, so its not sent back to the appliance
            x.reset_password();
            let r = core.run(apiclient.default_api().update_msg_vpn_client_username(msg_vpn, client_username, x, getselect("*")));
            match r {
                Ok(t) => info!("state successfully changed to {:?}", state),
                Err(e) => {
                    error!("error changing enabled state for client-username: {}, {:?}", client_username, e);
                    exit(126);
                }
            }

        } else {
            error!("error, did not find exactly one item matching query");
            process::exit(126);
        }

        Ok(())
    }

    fn ingress(msg_vpn: &str, item_name: &str, state: bool, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<(), &'static str> {
        unimplemented!()
    }

    fn egress(msg_vpn: &str, item_name: &str, state: bool, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<(), &'static str> {
        unimplemented!()
    }

    fn delete(msg_vpn: &str, item_name: &str, sub_identifier: &str, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<(), &'static str> {
        let t = apiclient.default_api().delete_msg_vpn_client_username(msg_vpn, item_name);
        match core.run(t) {
            Ok(vpn) => {
                info!("client-username deleted");
                Ok(())
            },
            Err(e) => {
                error!("unable to delete client-username: {:?}", e);
//                process::exit(126);
                Err("unable to delete client-username")
            }
        }
    }
}

// queue subscription

impl Update<MsgVpnQueueSubscriptionResponse> for MsgVpnQueueSubscriptionResponse {

    fn update(msg_vpn: &str, file_name: &str, sub_item: &str, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<(), &'static str> {
        unimplemented!()
    }

    fn enabled(msg_vpn: &str, item_name: &str, selector: Vec<&str>, state: bool, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<(), &'static str> {
        unimplemented!()
    }

    fn ingress(msg_vpn: &str, item_name: &str, state: bool, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<(), &'static str> {
        unimplemented!()
    }

    fn egress(msg_vpn: &str, item_name: &str, state: bool, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<(), &'static str> {
        unimplemented!()
    }

    fn delete(msg_vpn: &str, queue_name: &str, subscription_topic: &str, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<(), &'static str> {
        info!("deleting: {}", subscription_topic);
        let t = apiclient.default_api().delete_msg_vpn_queue_subscription(msg_vpn, queue_name, subscription_topic);
        match core.run(t) {
            Ok(vpn) => {
                info!("queue-subscription deleted");
                Ok(())
            },
            Err(e) => {
                error!("unable to delete queue-subscription: {:?}", e);
//                process::exit(126);
                Err("unable to delete queue-subscription")
            }
        }
    }
}



impl Update<MsgVpnSequencedTopicResponse> for MsgVpnSequencedTopicResponse {
    fn update(msg_vpn: &str, file_name: &str, sub_item: &str, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<(), &'static str> {
        unimplemented!()
    }

    fn enabled(msg_vpn: &str, item_name: &str, selector: Vec<&str>, state: bool, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<(), &'static str> {
        unimplemented!()
    }

    fn ingress(msg_vpn: &str, item_name: &str, state: bool, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<(), &'static str> {
        unimplemented!()
    }

    fn egress(msg_vpn: &str, item_name: &str, state: bool, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<(), &'static str> {
        unimplemented!()
    }

    fn delete(msg_vpn: &str, item_name: &str, sub_identifier: &str, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<(), &'static str> {
        info!("deleting: {}", sub_identifier);
        let t = apiclient.default_api().delete_msg_vpn_sequenced_topic(msg_vpn, item_name);
        match core.run(t) {
            Ok(vpn) => {
                info!("sequence-topic deleted");
                Ok(())
            },
            Err(e) => {
                error!("unable to delete sequence-topic: {:?}", e);
//                process::exit(126);
                Err("unable to delete sequence-topic")
            }
        }
    }
}


// topic endpoint

impl Update<MsgVpnTopicEndpointResponse> for MsgVpnTopicEndpointResponse {

    fn update(msg_vpn: &str, file_name: &str, sub_item: &str, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<(), &'static str> {
        unimplemented!()
    }

    fn enabled(msg_vpn: &str, item_name: &str, selector: Vec<&str>, state: bool, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<(), &'static str> {
        unimplemented!()
    }

    fn ingress(msg_vpn: &str, item_name: &str, state: bool, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<(), &'static str> {
        info!("retrieving current topic endpoint from appliance");
        let mut item = MsgVpnTopicEndpointsResponse::fetch(msg_vpn, item_name, "topicEndpointName",item_name, 10, "", "", core, apiclient)?;
        let mut titem = item.data().unwrap().clone();

        if titem.len() == 1 {
            info!("changing ingress state to: {}", state.to_string());
            let mut x = titem.pop().unwrap();
            x.set_ingress_enabled(state);
            let r = core.run(apiclient.default_api().update_msg_vpn_topic_endpoint(msg_vpn, item_name, x, getselect("*")));
            match r {
                Ok(t) => info!("ingress successfully changed to {:?}", state),
                Err(e) => {
                    error!("error changing ingress state for vpn: {}, {:?}", item_name, e);
                    exit(126);
                }
            }

        } else {
            error!("error, did not find exactly one item matching query");
            process::exit(126);
        }

        Ok(())
    }

    fn egress(msg_vpn: &str, item_name: &str, state: bool, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<(), &'static str> {
        info!("retrieving current queue from appliance");
        let mut item = MsgVpnTopicEndpointsResponse::fetch(msg_vpn, item_name, "topicEndpointName",item_name, 10, "", "", core, apiclient)?;
        let mut titem = item.data().unwrap().clone();

        if titem.len() == 1 {
            info!("changing egress state to: {}", state.to_string());
            let mut x = titem.pop().unwrap();
            x.set_egress_enabled(state);
            let r = core.run(apiclient.default_api().update_msg_vpn_topic_endpoint(msg_vpn, item_name, x, getselect("*")));
            match r {
                Ok(t) => info!("egress successfully changed to {:?}", state),
                Err(e) => {
                    error!("error changing egress state for vpn: {}, {:?}", item_name, e);
                    exit(126);
                }
            }

        } else {
            error!("error, did not find exactly one item matching query");
            process::exit(126);
        }

        Ok(())
    }

    fn delete(msg_vpn: &str, item_name: &str, sub_identifier: &str, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<(), &'static str> {
        info!("deleting: {}", sub_identifier);
        let t = apiclient.default_api().delete_msg_vpn_topic_endpoint(msg_vpn, item_name);
        match core.run(t) {
            Ok(vpn) => {
                info!("topic-endpoint deleted");
                Ok(())
            },
            Err(e) => {
                error!("unable to delete topic-endpoint: {:?}", e);
//                process::exit(126);
                Err("unable to delete topic-endpoint")
            }
        }
    }
}


// authorization group

impl Update<MsgVpnAuthorizationGroupResponse> for MsgVpnAuthorizationGroupResponse {

    fn update(msg_vpn: &str, file_name: &str, sub_item: &str, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<(), &'static str> {

        let file = std::fs::File::open(file_name).unwrap();
        let deserialized: Option<MsgVpnAuthorizationGroup> = serde_yaml::from_reader(file).unwrap();

        match deserialized {
            Some(mut item) => {
                item.set_msg_vpn_name(msg_vpn.to_owned());
                let item_name = item.authorization_group_name().cloned();
                let request = apiclient
                    .default_api()
                    .update_msg_vpn_authorization_group(msg_vpn, &*item_name.unwrap(), item, getselect("*"));
                match core.run(request) {
                    Ok(response) => {
                        info!("{}",format!("{}", serde_yaml::to_string(&response.data().unwrap()).unwrap()));
                        Ok(())
                    },
                    Err(e) => {
                        error!("update error: {:?}", e);
                        process::exit(126);
                        Err("update error")
                    }
                }
            }
            _ => unimplemented!()
        }
    }

    fn enabled(msg_vpn: &str, item_name: &str, selector: Vec<&str>, state: bool, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<(), &'static str> {
        println!("retrieving current authorization-group from appliance");
        let mut item = MsgVpnAuthorizationGroupsResponse::fetch(msg_vpn, item_name, "authorizationGroupName",item_name, 10, "", "", core, apiclient)?;
        let mut titem = item.data().unwrap().clone();

        if titem.len() == 1 {
            println!("changing enabled state to: {}", state.to_string());
            let mut x = titem.pop().unwrap();
            x.set_enabled(state);
            let r = core.run(apiclient.default_api().update_msg_vpn_authorization_group(msg_vpn, item_name, x, getselect("*")));
            match r {
                Ok(t) => info!("state successfully changed to {:?}", state),
                Err(e) => {
                    error!("error changing enabled state for authorization-group: {}, {:?}", item_name, e);
                    exit(126);
                }
            }

        } else {
            error!("error, did not find exactly one item matching query");
            process::exit(126);
        }

        Ok(())
    }

    fn ingress(msg_vpn: &str, item_name: &str, state: bool, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<(), &'static str> {
        unimplemented!()
    }

    fn egress(msg_vpn: &str, item_name: &str, state: bool, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<(), &'static str> {
        unimplemented!()
    }

    fn delete(msg_vpn: &str, item_name: &str, sub_identifier: &str, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<(), &'static str> {
        let t = apiclient.default_api().delete_msg_vpn_authorization_group(msg_vpn, item_name);
        match core.run(t) {
            Ok(vpn) => {
                info!("authorization-group deleted");
                Ok(())
            },
            Err(e) => {
                error!("unable to delete authorization-group: {:?}", e);
//                process::exit(126);
                Err("unable to delete authorization-group")
            }
        }
    }

}


// bridge

impl Update<MsgVpnBridgeResponse> for MsgVpnBridgeResponse {

    fn update(msg_vpn: &str, file_name: &str, sub_item: &str, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<(), &'static str> {

        let file = std::fs::File::open(file_name).unwrap();
        let deserialized: Option<MsgVpnBridge> = serde_yaml::from_reader(file).unwrap();

        match deserialized {
            Some(mut item) => {
                item.set_msg_vpn_name(msg_vpn.to_owned());
                let item_name = item.bridge_name().cloned();
                let bridge_virtual_router = item.bridge_virtual_router().cloned();
                let request = apiclient
                    .default_api()
                    .update_msg_vpn_bridge(msg_vpn,
                                           &*item_name.unwrap(),
                                           &*bridge_virtual_router.unwrap(),
                                           item,
                                           getselect("*"));
                match core.run(request) {
                    Ok(response) => {
                        info!("{}",format!("{}", serde_yaml::to_string(&response.data().unwrap()).unwrap()));
                        Ok(())
                    },
                    Err(e) => {
                        error!("update error: {:?}", e);
                        process::exit(126);
                        Err("update error")
                    }
                }
            }
            _ => unimplemented!()
        }
    }

    fn enabled(msg_vpn: &str, item_name: &str, selector: Vec<&str>, state: bool, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<(), &'static str> {
        println!("retrieving current bridge from appliance");
        let mut item = MsgVpnBridgesResponse::fetch(msg_vpn,
                                                    item_name,
                                                    "bridgeName",
                                                    item_name,
                                                    10,
                                                    "",
                                                    "",
                                                    core,
                                                    apiclient)?;
        let mut titem = item.data().unwrap().clone();

        if titem.len() == 1 {
            println!("changing enabled state to: {}", state.to_string());
            let mut x = titem.pop().unwrap();
            let virtual_router = &*x.bridge_virtual_router().cloned().unwrap();
            x.set_enabled(state);
            let r = core.run(apiclient.default_api().update_msg_vpn_bridge(msg_vpn,
                                                                           item_name,
                                                                           virtual_router,
                                                                           x,
                                                                           getselect("*")));
            match r {
                Ok(t) => info!("state successfully changed to {:?}", state),
                Err(e) => {
                    error!("error changing enabled state for authorization-group: {}, {:?}", item_name, e);
                    exit(126);
                }
            }

        } else {
            error!("error, did not find exactly one item matching query");
            process::exit(126);
        }

        Ok(())
    }

    fn ingress(msg_vpn: &str, item_name: &str, state: bool, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<(), &'static str> {
        unimplemented!()
    }

    fn egress(msg_vpn: &str, item_name: &str, state: bool, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<(), &'static str> {
        unimplemented!()
    }

    fn delete(msg_vpn: &str, brige_name: &str, virtual_router: &str, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<(), &'static str> {

        let t = apiclient.default_api().delete_msg_vpn_bridge(msg_vpn, brige_name, virtual_router);
        match core.run(t) {
            Ok(vpn) => {
                info!("bridge deleted");
                Ok(())
            },
            Err(e) => {
                error!("unable to delete bridge: {:?}", e);
                process::exit(126);
                Err("unable to delete bridge")
            }
        }
    }
}

// bridge-remote

impl Update<MsgVpnBridgeRemoteMsgVpnResponse> for MsgVpnBridgeRemoteMsgVpnResponse {

    fn update(msg_vpn: &str, file_name: &str, remote_vpn_name: &str, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<(), &'static str> {

        let file = std::fs::File::open(file_name).unwrap();
        let deserialized: Option<MsgVpnBridgeRemoteMsgVpn> = serde_yaml::from_reader(file).unwrap();

        match deserialized {
            Some(mut item) => {
                item.set_msg_vpn_name(msg_vpn.to_owned());
                let item_name = item.bridge_name().cloned();
                let bridge_virtual_router = item.bridge_virtual_router().cloned();
                let remote_vpn_name = item.remote_msg_vpn_name().cloned();
                let remote_vpn_location = item.remote_msg_vpn_location().cloned();
                let remote_msg_vpn_interface = item.remote_msg_vpn_interface().cloned();
                let request = apiclient
                    .default_api()
                    .update_msg_vpn_bridge_remote_msg_vpn(msg_vpn,
                                                          &*item_name.unwrap(),
                                                          &*bridge_virtual_router.unwrap(),
                                                          &*remote_vpn_name.unwrap(),
                                                          &*remote_vpn_location.unwrap(),
                                                            &*remote_msg_vpn_interface.unwrap(),
                                                            item,
                                                            getselect("*")
                    );
                match core.run(request) {
                    Ok(response) => {
                        info!("{}",format!("{}", serde_yaml::to_string(&response.data().unwrap()).unwrap()));
                        Ok(())
                    },
                    Err(e) => {
                        error!("update error: {:?}", e);
                        process::exit(126);
                        Err("update error")
                    }
                }
            }
            _ => unimplemented!()
        }
    }

    fn enabled(msg_vpn: &str, bridge_name: &str, selector: Vec<&str>, state: bool, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<(), &'static str> {
        println!("retrieving remote-bridge-vpn from appliance");
        let mut item = MsgVpnBridgeRemoteMsgVpnsResponse::fetch(msg_vpn,
                                                                bridge_name,
                                                                selector[0],
                                                                bridge_name,
                                                                10,
                                                                "",
                                                                "",
                                                                core,
                                                                apiclient)?;
        let mut titem = item.data().unwrap().clone();

        if titem.len() == 1 {
            println!("changing enabled state to: {}", state.to_string());
            let mut x = titem.pop().unwrap();
            let virtual_router = &*x.bridge_virtual_router().cloned().unwrap();
            let remote_msg_vpn_name = &*x.remote_msg_vpn_name().cloned().unwrap();
            let remote_location = &*x.remote_msg_vpn_location().cloned().unwrap();
            let remote_interface = &*x.remote_msg_vpn_interface().cloned().unwrap();
            x.set_enabled(state);
            let r = core.run(apiclient.default_api().update_msg_vpn_bridge_remote_msg_vpn(
                msg_vpn,
                bridge_name,
                virtual_router,
                remote_msg_vpn_name,
                remote_location,
                remote_interface,
                x,
                getselect("*")
            ));
            match r {
                Ok(t) => info!("state successfully changed to {:?}", state),
                Err(e) => {
                    error!("error changing enabled state for authorization-group: {}, {:?}", bridge_name, e);
                    exit(126);
                }
            }

        } else {
            error!("error, did not find exactly one item matching query");
            process::exit(126);
        }

        Ok(())
    }

    fn ingress(msg_vpn: &str, item_name: &str, state: bool, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<(), &'static str> {
        unimplemented!()
    }

    fn egress(msg_vpn: &str, item_name: &str, state: bool, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<(), &'static str> {
        unimplemented!()
    }

    fn delete(msg_vpn: &str, bridge_name: &str, virtual_router: &str, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<(), &'static str> {

        let mut item = MsgVpnBridgeRemoteMsgVpnsResponse::fetch(msg_vpn,
                                                                bridge_name,
                                                                virtual_router,
                                                                bridge_name,
                                                                10,
                                                                "",
                                                                "",
                                                                core,
                                                                apiclient)?;


        let mut titem = item.data().unwrap().clone();

        if titem.len() == 1 {
            let mut x = titem.pop().unwrap();
            let virtual_router = &*x.bridge_virtual_router().cloned().unwrap();
            let remote_bridge_name = &*x.remote_msg_vpn_name().cloned().unwrap();
            let remote_bridge_location = &*x.remote_msg_vpn_location().cloned().unwrap();
            let remote_interface = &*x.remote_msg_vpn_interface().cloned().unwrap();
            let t = apiclient.default_api().delete_msg_vpn_bridge_remote_msg_vpn(
                msg_vpn,
                bridge_name,
                virtual_router,
                remote_bridge_name,
                remote_bridge_location,
                remote_interface
            );
            match core.run(t) {
                Ok(vpn) => {
                    info!("bridge deleted");
                    Ok(())
                },
                Err(e) => {
                    error!("unable to delete bridge: {:?}", e);
                    process::exit(126);
                    Err("unable to delete bridge")
                }
            }
        } else {
            error!("error, did not find exactly one item matching query");
            process::exit(126);
        }

    }
}