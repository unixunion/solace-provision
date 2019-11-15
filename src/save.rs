

use solace_semp_client::models::{MsgVpnsResponse, MsgVpnQueueSubscription, MsgVpnQueueSubscriptionsResponse, MsgVpnSequencedTopicsResponse, MsgVpnSequencedTopic, MsgVpnTopicEndpoint, MsgVpnTopicEndpointsResponse, MsgVpnAuthorizationGroup, MsgVpnAuthorizationGroupsResponse, MsgVpnBridgesResponse, MsgVpnBridge, MsgVpnBridgeRemoteMsgVpn, MsgVpnBridgeRemoteMsgVpnsResponse, MsgVpnReplayLogResponse, MsgVpnReplayLog, MsgVpnDmrBridge, MsgVpnDmrBridgesResponse};
use solace_semp_client::models::MsgVpn;
use serde::Serialize;
use std::path::Path;
use std::fs;
use std::fs::File;
use std::io::Write;
use solace_semp_client::models::MsgVpnQueuesResponse;
use solace_semp_client::models::MsgVpnQueue;
use solace_semp_client::models::MsgVpnAclProfilesResponse;
use solace_semp_client::models::MsgVpnAclProfile;
use solace_semp_client::models::MsgVpnClientProfile;
use solace_semp_client::models::MsgVpnClientUsername;
use solace_semp_client::models::MsgVpnClientProfilesResponse;
use solace_semp_client::models::MsgVpnClientUsernamesResponse;
extern crate sha1;

pub trait Save<T> {

    fn save(dir: &str, data: &T) -> Result<(), &'static str> where T: Serialize;

    fn save_in_dir(&self, dir: &str, subdir: &str, vpn_name: &Option<&String>, item_name: &Option<&String>) -> Result<(), &'static str> where Self: Serialize {

        let output_dir = dir;
        let mut t_vpn_name = "";
        let mut t_item_name = "";

        match vpn_name {
            Some(tvpn) => {
                t_vpn_name = tvpn;
                if !Path::new(&format!("{}/{}/{}", output_dir, tvpn, subdir)).exists() {
                    match fs::create_dir_all(format!("{}/{}/{}", output_dir, tvpn, subdir)) {
                        Ok(r) => {
                            debug!("Created dir");
                        },
                        Err(e) => {
                            error!("error creatuing dir");
                            panic!("error creating dir");
                        },
                        _ => unimplemented!()
                    }

                } else {
                    debug!("dir exists");
                }
            },
            _ => unimplemented!()
        }

        match item_name {
            Some(titem) => {
                let target_file = &format!("{}/{}/{}/{}.yaml", output_dir, t_vpn_name, subdir, titem);
                let mut f = File::create(target_file);
                match f {
                    Ok(mut _f) => {

                        let serialized_item = serde_yaml::to_string(self);
                        match serialized_item {
                            Ok(item) => {
                                _f.write(item.as_bytes());
                            },
                            Err(e) => {
                                error!("error serializing");
                                panic!("error serializing");
                            }
                        }
                    },
                    Err(e) => {
                        error!("{}", format!("error saving {}, error={}", target_file, e));
                        panic!("error saving file");
                    }
                }
                Ok(())
            },
            _ => unimplemented!()
        }
    }
}

impl Save<MsgVpn> for MsgVpn {
    fn save(dir: &str, data: &MsgVpn) -> Result<(), &'static str> where MsgVpn: Serialize {
        let vpn_name = data.msg_vpn_name();
        let item_name = data.msg_vpn_name();
        debug!("save vpn: {:?}, {:?}", vpn_name, item_name);
        data.save_in_dir(dir, "vpn", &vpn_name, &item_name);
        Ok(())
    }
}

impl Save<MsgVpnQueue> for MsgVpnQueue {
    fn save(dir: &str, data: &MsgVpnQueue) -> Result<(), &'static str> where MsgVpnQueue: Serialize {
        let vpn_name = data.msg_vpn_name();
        let item_name = data.queue_name();
        debug!("save queue: {:?}, {:?}", vpn_name, item_name);
        data.save_in_dir(dir, "queue", &vpn_name, &item_name);
        Ok(())
    }
}

impl Save<MsgVpnAclProfile> for MsgVpnAclProfile {
    fn save(dir: &str, data: &MsgVpnAclProfile) -> Result<(), &'static str> where MsgVpnAclProfile: Serialize {
        let vpn_name = data.msg_vpn_name();
        let item_name = data.acl_profile_name();
        debug!("save acl: {:?}, {:?}", vpn_name, item_name);
        data.save_in_dir(dir, "acl", &vpn_name, &item_name);
        Ok(())
    }
}

impl Save<MsgVpnClientProfile> for MsgVpnClientProfile {
    fn save(dir: &str, data: &MsgVpnClientProfile) -> Result<(), &'static str> where MsgVpnClientProfile: Serialize {
        let vpn_name = data.msg_vpn_name();
        let item_name = data.client_profile_name();
        debug!("save client-profile: {:?}, {:?}", vpn_name, item_name);
        data.save_in_dir(dir, "client-profile", &vpn_name, &item_name);
        Ok(())
    }
}

impl Save<MsgVpnClientUsername> for MsgVpnClientUsername {
    fn save(dir: &str, data: &MsgVpnClientUsername) -> Result<(), &'static str> where MsgVpnClientUsername: Serialize {
        let vpn_name = data.msg_vpn_name();
        let item_name = data.client_username();
        debug!("save client-username: {:?}, {:?}", vpn_name, item_name);
        data.save_in_dir(dir, "client-username", &vpn_name, &item_name);
        Ok(())
    }
}


impl Save<MsgVpnsResponse> for MsgVpnsResponse {
    fn save(dir: &str, data: &MsgVpnsResponse) -> Result<(), &'static str> where MsgVpnsResponse: Serialize {
        match data.data() {
            Some(vpns) => {
                for vpn in vpns {
                    match MsgVpn::save(dir, vpn) {
                        Ok(t) => debug!("success saving"),
                        Err(e) => error!("error writing: {:?}", e)
                    }

                }
                Ok(())
            },
            _ => {
                error!("no vpns");
                Err("no vpns")
            }
        }
    }
}

impl Save<MsgVpnQueuesResponse> for MsgVpnQueuesResponse {
    fn save(dir: &str, data: &MsgVpnQueuesResponse) -> Result<(), &'static str> where MsgVpnQueuesResponse: Serialize {
        match data.data() {
            Some(items) => {
                for item in items {
                    match MsgVpnQueue::save(dir, item) {
                        Ok(t) => debug!("success saving"),
                        Err(e) => error!("error writing: {:?}", e)
                    }

                }
                Ok(())
            },
            _ => {
                error!("no queues");
                Err("no queues")
            }
        }
    }
}

impl Save<MsgVpnAclProfilesResponse> for MsgVpnAclProfilesResponse {
    fn save(dir: &str, data: &MsgVpnAclProfilesResponse) -> Result<(), &'static str> where MsgVpnAclProfilesResponse: Serialize {
        match data.data() {
            Some(items) => {
                for item in items {
                    match MsgVpnAclProfile::save(dir, item) {
                        Ok(t) => debug!("success saving"),
                        Err(e) => error!("error writing: {:?}", e)
                    }

                }
                Ok(())
            },
            _ => {
                error!("no acls");
                Err("no acl")
            }
        }
    }
}


impl Save<MsgVpnClientProfilesResponse> for MsgVpnClientProfilesResponse {
    fn save(dir: &str, data: &MsgVpnClientProfilesResponse) -> Result<(), &'static str> where MsgVpnClientProfilesResponse: Serialize {
        match data.data() {
            Some(items) => {
                for item in items {
                    match MsgVpnClientProfile::save(dir, item) {
                        Ok(t) => debug!("success saving"),
                        Err(e) => error!("error writing: {:?}", e)
                    }

                }
                Ok(())
            },
            _ => {
                error!("no profiles");
                Err("no profiles")
            }
        }
    }
}

impl Save<MsgVpnClientUsernamesResponse> for MsgVpnClientUsernamesResponse {
    fn save(dir: &str, data: &MsgVpnClientUsernamesResponse) -> Result<(), &'static str> where MsgVpnClientUsernamesResponse: Serialize {
        match data.data() {
            Some(items) => {
                for item in items {
                    match MsgVpnClientUsername::save(dir, item) {
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

// Queue Subscriptions


impl Save<MsgVpnQueueSubscription> for MsgVpnQueueSubscription {
    fn save(dir: &str, data: &MsgVpnQueueSubscription) -> Result<(), &'static str> where MsgVpnQueueSubscription: Serialize {
        let vpn_name = data.msg_vpn_name();
        let mut hasher = sha1::Sha1::new();
        match data.subscription_topic() {
            Some(i) => {
                hasher.update(i.as_bytes());
                let s = hasher.digest().to_string();
                let item_name = Option::from(&s);
                debug!("save queue-subscription: {:?}, {:?}", vpn_name, item_name);
                data.save_in_dir(dir, "queue-subscription", &vpn_name, &item_name);
                Ok(())
            },
            _ => {
                panic!("unable to get queue subscription from item")
            }
        }

    }
}


impl Save<MsgVpnQueueSubscriptionsResponse> for MsgVpnQueueSubscriptionsResponse {
    fn save(dir: &str, data: &MsgVpnQueueSubscriptionsResponse) -> Result<(), &'static str> where MsgVpnQueueSubscriptionsResponse: Serialize {
        match data.data() {
            Some(items) => {
                for item in items {
                    match MsgVpnQueueSubscription::save(dir, item) {
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



// Sequenced Topics

impl Save<MsgVpnSequencedTopic> for MsgVpnSequencedTopic {
    fn save(dir: &str, data: &MsgVpnSequencedTopic) -> Result<(), &'static str> where MsgVpnSequencedTopic: Serialize {
        let vpn_name = data.msg_vpn_name();
        let mut hasher = sha1::Sha1::new();
        match data.sequenced_topic() {
            Some(i) => {
                hasher.update(i.as_bytes());
                let s = hasher.digest().to_string();
                let item_name = Option::from(&s);
                debug!("save queue-subscription: {:?}, {:?}", vpn_name, item_name);
                data.save_in_dir(dir, "sequenced-topic", &vpn_name, &item_name);
                Ok(())
            },
            _ => {
                panic!("unable to get queue subscription from item")
            }
        }

    }
}


impl Save<MsgVpnSequencedTopicsResponse> for MsgVpnSequencedTopicsResponse {
    fn save(dir: &str, data: &MsgVpnSequencedTopicsResponse) -> Result<(), &'static str> where MsgVpnSequencedTopicsResponse: Serialize {
        match data.data() {
            Some(items) => {
                for item in items {
                    match MsgVpnSequencedTopic::save(dir, item) {
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


// Topic Endpoint

impl Save<MsgVpnTopicEndpoint> for MsgVpnTopicEndpoint {
    fn save(dir: &str, data: &MsgVpnTopicEndpoint) -> Result<(), &'static str> where MsgVpnTopicEndpoint: Serialize {
        let vpn_name = data.msg_vpn_name();
        let item_name = data.topic_endpoint_name();

        debug!("save topic-endpoint: {:?}, {:?}", vpn_name, item_name);
        data.save_in_dir(dir, "topic-endpoint", &vpn_name, &item_name);
        Ok(())
    }
}


impl Save<MsgVpnTopicEndpointsResponse> for MsgVpnTopicEndpointsResponse {
    fn save(dir: &str, data: &MsgVpnTopicEndpointsResponse) -> Result<(), &'static str> where MsgVpnTopicEndpointsResponse: Serialize {
        match data.data() {
            Some(items) => {
                for item in items {
                    match MsgVpnTopicEndpoint::save(dir, item) {
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

// authorization group

impl Save<MsgVpnAuthorizationGroup> for MsgVpnAuthorizationGroup {
    fn save(dir: &str, data: &MsgVpnAuthorizationGroup) -> Result<(), &'static str> where MsgVpnAuthorizationGroup: Serialize {
        let vpn_name = data.msg_vpn_name();
        let item_name = data.authorization_group_name();

        debug!("save authorization-group: {:?}, {:?}", vpn_name, item_name);
        data.save_in_dir(dir, "authorization-group", &vpn_name, &item_name);
        Ok(())
    }
}


impl Save<MsgVpnAuthorizationGroupsResponse> for MsgVpnAuthorizationGroupsResponse {
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

// bridge

impl Save<MsgVpnBridge> for MsgVpnBridge {
    fn save(dir: &str, data: &MsgVpnBridge) -> Result<(), &'static str> where MsgVpnBridge: Serialize {
        let vpn_name = data.msg_vpn_name();
        let item_name = data.bridge_name();
        debug!("save bridge: {:?}, {:?}", vpn_name, item_name);
        data.save_in_dir(dir, "bridge", &vpn_name, &item_name);
        Ok(())
    }
}

impl Save<MsgVpnBridgesResponse> for MsgVpnBridgesResponse {
    fn save(dir: &str, data: &MsgVpnBridgesResponse) -> Result<(), &'static str> where MsgVpnBridgesResponse: Serialize {
        match data.data() {
            Some(items) => {
                for item in items {
                    match MsgVpnBridge::save(dir, item) {
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

// bridge-remote

impl Save<MsgVpnBridgeRemoteMsgVpn> for MsgVpnBridgeRemoteMsgVpn {
    fn save(dir: &str, data: &MsgVpnBridgeRemoteMsgVpn) -> Result<(), &'static str> where MsgVpnBridgeRemoteMsgVpn: Serialize {
        let vpn_name = data.msg_vpn_name();
        let item_name = data.bridge_name();
        debug!("save bridge: {:?}, {:?}", vpn_name, item_name);
        data.save_in_dir(dir, "remote-bridge", &vpn_name, &item_name);
        Ok(())
    }
}

impl Save<MsgVpnBridgeRemoteMsgVpnsResponse> for MsgVpnBridgeRemoteMsgVpnsResponse {
    fn save(dir: &str, data: &MsgVpnBridgeRemoteMsgVpnsResponse) -> Result<(), &'static str> where MsgVpnBridgeRemoteMsgVpnsResponse: Serialize {
        match data.data() {
            Some(items) => {
                for item in items {
                    match MsgVpnBridgeRemoteMsgVpn::save(dir, item) {
                        Ok(t) => debug!("success saving"),
                        Err(e) => error!("error writing: {:?}", e)
                    }
                }
                Ok(())
            },
            _ => {
                error!("no bridge remote vpns");
                Err("no bridge remote vpns")
            }
        }
    }
}

// replay-log

impl Save<MsgVpnReplayLog> for MsgVpnReplayLog {
    fn save(dir: &str, data: &MsgVpnReplayLog) -> Result<(), &'static str> where MsgVpnReplayLog: Serialize {
        let vpn_name = data.msg_vpn_name();
        let item_name = data.replay_log_name();
        debug!("save replay-log: {:?}, {:?}", vpn_name, item_name);
        data.save_in_dir(dir, "replay-log", &vpn_name, &item_name);
        Ok(())
    }
}

// dmr-bridge

impl Save<MsgVpnDmrBridge> for MsgVpnDmrBridge {
    fn save(dir: &str, data: &MsgVpnDmrBridge) -> Result<(), &'static str> where MsgVpnDmrBridge: Serialize {
        let vpn_name = data.msg_vpn_name();
        let mut item_name =  data.msg_vpn_name().unwrap().clone();
        item_name.push_str("-");
        item_name.push_str(data.remote_msg_vpn_name().unwrap());
        let item_name = Some(&item_name);
//
//        Some(format!("{:?}-{:?}", data.msg_vpn_name(), data.remote_msg_vpn_name()));
        debug!("save dmr-bridge: {:?}, {:?}", vpn_name, item_name);
        data.save_in_dir(dir, "dmr-bridge", &vpn_name, &item_name);
        Ok(())
    }
}

impl Save<MsgVpnDmrBridgesResponse> for MsgVpnDmrBridgesResponse {
    fn save(dir: &str, data: &MsgVpnDmrBridgesResponse) -> Result<(), &'static str> where MsgVpnDmrBridgesResponse: Serialize {
        match data.data() {
            Some(items) => {
                for item in items {
                    match MsgVpnDmrBridge::save(dir, item) {
                        Ok(t) => debug!("success saving"),
                        Err(e) => error!("error writing: {:?}", e)
                    }
                }
                Ok(())
            },
            _ => {
                error!("no dmr-bridges");
                Err("no dmr-bridges")
            }
        }
    }
}