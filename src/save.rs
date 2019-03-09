

use solace_semp_client::models::MsgVpnsResponse;
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
                    info!("dir exists");
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
        info!("save vpn: {:?}, {:?}", vpn_name, item_name);
        data.save_in_dir(dir, "vpn", &vpn_name, &item_name);
        Ok(())
    }
}

impl Save<MsgVpnQueue> for MsgVpnQueue {
    fn save(dir: &str, data: &MsgVpnQueue) -> Result<(), &'static str> where MsgVpnQueue: Serialize {
        let vpn_name = data.msg_vpn_name();
        let item_name = data.queue_name();
        info!("save queue: {:?}, {:?}", vpn_name, item_name);
        data.save_in_dir(dir, "queue", &vpn_name, &item_name);
        Ok(())
    }
}

impl Save<MsgVpnAclProfile> for MsgVpnAclProfile {
    fn save(dir: &str, data: &MsgVpnAclProfile) -> Result<(), &'static str> where MsgVpnAclProfile: Serialize {
        let vpn_name = data.msg_vpn_name();
        let item_name = data.acl_profile_name();
        info!("save acl: {:?}, {:?}", vpn_name, item_name);
        data.save_in_dir(dir, "acl", &vpn_name, &item_name);
        Ok(())
    }
}

impl Save<MsgVpnClientProfile> for MsgVpnClientProfile {
    fn save(dir: &str, data: &MsgVpnClientProfile) -> Result<(), &'static str> where MsgVpnClientProfile: Serialize {
        let vpn_name = data.msg_vpn_name();
        let item_name = data.client_profile_name();
        info!("save client-profile: {:?}, {:?}", vpn_name, item_name);
        data.save_in_dir(dir, "client-profile", &vpn_name, &item_name);
        Ok(())
    }
}

impl Save<MsgVpnClientUsername> for MsgVpnClientUsername {
    fn save(dir: &str, data: &MsgVpnClientUsername) -> Result<(), &'static str> where MsgVpnClientUsername: Serialize {
        let vpn_name = data.msg_vpn_name();
        let item_name = data.client_username();
        info!("save client-username: {:?}, {:?}", vpn_name, item_name);
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
                        Ok(t) => info!("success saving"),
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
                        Ok(t) => info!("success saving"),
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
                        Ok(t) => info!("success saving"),
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
                        Ok(t) => info!("success saving"),
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
                        Ok(t) => info!("success saving"),
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