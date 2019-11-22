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
use solace_semp_client::models::{MsgVpnDmrBridgesResponse, MsgVpnDmrBridge, MsgVpnDmrBridgeResponse};

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

        core_run!(request, core)

    }
}

// provision

impl Provision<MsgVpnDmrBridgeResponse> for MsgVpnDmrBridgeResponse {
    fn provision_with_file(in_vpn: &str, unused_name: &str, file_name: &str, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<MsgVpnDmrBridgeResponse, &'static str> {
        let file = std::fs::File::open(file_name).unwrap();
        let deserialized: Option<MsgVpnDmrBridge> = serde_yaml::from_reader(file).unwrap();
        match deserialized {
            Some(mut item) => {
                item.set_msg_vpn_name(in_vpn.to_owned());
                let request = apiclient
                    .default_api()
                    .create_msg_vpn_dmr_bridge(in_vpn, item, helpers::getselect("*"));
                core_run!(request, core)

            }
            _ => unimplemented!()
        }
    }
}

// save

impl Save<MsgVpnDmrBridge> for MsgVpnDmrBridge {
    fn save(dir: &str, data: &MsgVpnDmrBridge) -> Result<(), &'static str> where MsgVpnDmrBridge: Serialize {
        let vpn_name = data.msg_vpn_name();
        let mut item_name =  data.msg_vpn_name().unwrap().clone();
        item_name.push_str("-");
        item_name.push_str(data.remote_msg_vpn_name().unwrap());
        let item_name = Some(&item_name);
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

// update

impl Update<MsgVpnDmrBridgeResponse> for MsgVpnDmrBridgeResponse {

}