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
use solace_semp_client::models::{MsgVpnBridgesResponse, MsgVpnBridgeResponse, MsgVpnBridge, SempMetaOnlyResponse};


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

        core_run!(request, core)

    }
}


impl Provision<MsgVpnBridgeResponse> for MsgVpnBridgeResponse {

    fn provision_with_file(in_vpn: &str, item_name: &str, file_name: &str, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<MsgVpnBridgeResponse, &'static str> {
        let file = std::fs::File::open(file_name).unwrap();
        let deserialized: Option<MsgVpnBridge> = serde_yaml::from_reader(file).unwrap();
        match deserialized {
            Some(mut item) => {
                item.set_msg_vpn_name(in_vpn.to_owned());
                let request = apiclient
                    .default_api()
                    .create_msg_vpn_bridge(in_vpn, item, helpers::getselect("*"));
                core_run!(request, core)

            }
            _ => unimplemented!()
        }
    }

}


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


// bridge

impl Update<MsgVpnBridgeResponse> for MsgVpnBridgeResponse {

    fn update(msg_vpn: &str, file_name: &str, sub_item: &str, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<MsgVpnBridgeResponse, &'static str> {

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
                                           helpers::getselect("*"));
                core_run!(request, core)

            }
            _ => unimplemented!()
        }
    }

    fn enabled(msg_vpn: &str, item_name: &str, selector: Vec<&str>, state: bool, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<MsgVpnBridgeResponse, &'static str> {
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
            let request = apiclient.default_api().update_msg_vpn_bridge(msg_vpn,
                                                                        item_name,
                                                                        virtual_router,
                                                                        x,
                                                                        helpers::getselect("*"));

            core_run!(request, core)

        } else {
            error!("error, did not find exactly one item matching query");
            process::exit(126);
        }

//        Ok(())
    }

    fn delete(msg_vpn: &str, brige_name: &str, virtual_router: &str, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<SempMetaOnlyResponse, &'static str> {

        let request = apiclient.default_api().delete_msg_vpn_bridge(msg_vpn, brige_name, virtual_router);
        core_run_meta!(request, core)

    }
}