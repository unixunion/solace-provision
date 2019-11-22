

use solace_semp_client::models::{MsgVpnsResponse, MsgVpnQueueSubscription, MsgVpnQueueSubscriptionsResponse, MsgVpnSequencedTopicsResponse, MsgVpnSequencedTopic, MsgVpnTopicEndpoint, MsgVpnTopicEndpointsResponse, MsgVpnAuthorizationGroup, MsgVpnAuthorizationGroupsResponse, MsgVpnBridgesResponse, MsgVpnBridge, MsgVpnBridgeRemoteMsgVpn, MsgVpnBridgeRemoteMsgVpnsResponse, MsgVpnReplayLogResponse, MsgVpnReplayLog, MsgVpnDmrBridge, MsgVpnDmrBridgesResponse, DmrClustersResponse, DmrCluster, DmrClusterLinksResponse, DmrClusterLink, DmrClusterLinkRemoteAddressesResponse, DmrClusterLinkRemoteAddress, MsgVpnAclProfilePublishException, MsgVpnAclProfilePublishExceptionsResponse, MsgVpnAclProfileSubscribeException, MsgVpnAclProfileSubscribeExceptionsResponse};
use solace_semp_client::models::MsgVpn;
use serde::Serialize;
use std::path::Path;
use std::{fs, ptr};
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
use std::ptr::null;

extern crate sha1;

pub trait Save<T> {

    fn save(dir: &str, data: &T) -> Result<(), &'static str> where T: Serialize {
        unimplemented!()
    }
//    fn save(dir: &str, data: &T) -> Result<(), &'static str> where T: Serialize {
//        match data.data() {
//            Some(items) => {
//                for item in items {
//                    match T::save(dir, item) {
//                        Ok(t) => debug!("success saving"),
//                        Err(e) => error!("error writing: {:?}", e)
//                    }
//                }
//                Ok(())
//            },
//            _ => {
//                error!("nothing to save");
//                Err("nothing to save")
//            }
//        }
//    }


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



// dmr-cluster


// dmr cluster link




// dmr cluster link remotes

impl Save<DmrClusterLinkRemoteAddressesResponse> for DmrClusterLinkRemoteAddressesResponse {
    fn save(dir: &str, data: &DmrClusterLinkRemoteAddressesResponse) -> Result<(), &'static str> where DmrClusterLinkRemoteAddressesResponse: Serialize {
        match data.data() {
            Some(items) => {
                for item in items {
                    match DmrClusterLinkRemoteAddress::save(dir, item) {
                        Ok(t) => debug!("success saving"),
                        Err(e) => error!("error writing: {:?}", e)
                    }
                }
                Ok(())
            },
            _ => {
                error!("no dmr cluster link remotes");
                Err("no dmr cluster link remotes")
            }
        }
    }
}

impl Save<DmrClusterLinkRemoteAddress> for DmrClusterLinkRemoteAddress {
    fn save(dir: &str, data: &DmrClusterLinkRemoteAddress) -> Result<(), &'static str> where DmrClusterLinkRemoteAddress: Serialize {
        let name = &String::from("global");
        let node_name =  Some(name);
        let mut item_name =  data.remote_node_name().unwrap().clone();
        let item_name = Some(&item_name);
        debug!("save dmr-cluster-link-remote: {:?}, {:?}", node_name, item_name);
        data.save_in_dir(dir, "dmr-cluster-link-remote", &node_name, &item_name);
        Ok(())
    }
}