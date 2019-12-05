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
use solace_semp_client::models::{MsgVpnBridgeRemoteMsgVpnsResponse, MsgVpnBridgeRemoteMsgVpn, MsgVpnBridgeRemoteMsgVpnResponse, SempMetaOnlyResponse};

// remote bridge

impl Fetch<MsgVpnBridgeRemoteMsgVpnsResponse> for MsgVpnBridgeRemoteMsgVpnsResponse {
    fn fetch(in_vpn: &str, bridge_name: &str, bridge_virtual_router: &str, unused_1: &str, count: i32, cursor: &str, selector: &str, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<MsgVpnBridgeRemoteMsgVpnsResponse, &'static str> {
        let (wherev, mut selectv) = helpers::getwhere("bridgeName", bridge_name, selector);

        let request = apiclient
            .default_api()
            .get_msg_vpn_bridge_remote_msg_vpns(in_vpn, bridge_name, bridge_virtual_router,  wherev, selectv)
            .and_then(|item| {
                futures::future::ok(item)
            });

        core_run!(request, core)

    }
}

impl Provision<MsgVpnBridgeRemoteMsgVpnResponse> for MsgVpnBridgeRemoteMsgVpnResponse {

    fn provision_with_file(unused_1: &str, unused_2: &str, file_name: &str, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<MsgVpnBridgeRemoteMsgVpnResponse, &'static str> {

        let deserialized = deserialize_file_into_type!(file_name, MsgVpnBridgeRemoteMsgVpn);

        match deserialized {
            Some(mut item) => {
//                item.set_msg_vpn_name(in_vpn.to_owned());
                let virtual_router = &*item.bridge_virtual_router().cloned().unwrap();
                let request = apiclient
                    .default_api()
                    .create_msg_vpn_bridge_remote_msg_vpn(&*item.msg_vpn_name().cloned().unwrap(), &*item.bridge_name().cloned().unwrap(), virtual_router, item, helpers::getselect("*"));
                core_run!(request, core)

            }
            _ => unimplemented!()
        }
    }
}

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

// bridge-remote

impl Update<MsgVpnBridgeRemoteMsgVpnResponse> for MsgVpnBridgeRemoteMsgVpnResponse {

    fn update(unused_1: &str, file_name: &str, unused_2: &str, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<MsgVpnBridgeRemoteMsgVpnResponse, &'static str> {

        let file = std::fs::File::open(file_name).unwrap();
        let deserialized: Option<MsgVpnBridgeRemoteMsgVpn> = serde_yaml::from_reader(file).unwrap();

        match deserialized {
            Some(mut item) => {
                let msg_vpn_name = item.msg_vpn_name().cloned();
                let item_name = item.bridge_name().cloned();
                let bridge_virtual_router = item.bridge_virtual_router().cloned();
                let remote_vpn_name = item.remote_msg_vpn_name().cloned();
                let remote_vpn_location = item.remote_msg_vpn_location().cloned();
                let remote_msg_vpn_interface = item.remote_msg_vpn_interface().cloned();
                let request = apiclient
                    .default_api()
                    .update_msg_vpn_bridge_remote_msg_vpn(
                        &*msg_vpn_name.unwrap(),
                        &*item_name.unwrap(),
                        &*bridge_virtual_router.unwrap(),
                        &*remote_vpn_name.unwrap(),
                        &*remote_vpn_location.unwrap(),
                        &*remote_msg_vpn_interface.unwrap_or("".to_owned()),
                        item,
                        helpers::getselect("*")
                    );

                core_run!(request, core)

            }
            _ => unimplemented!()
        }
    }

    fn enabled(msg_vpn: &str, bridge_name: &str, selector: Vec<&str>, state: bool, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<MsgVpnBridgeRemoteMsgVpnResponse, &'static str> {
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
            let remote_interface = &*x.remote_msg_vpn_interface().cloned().unwrap_or("".to_owned());
            x.set_enabled(state);

            let request = apiclient.default_api().update_msg_vpn_bridge_remote_msg_vpn(
                msg_vpn,
                bridge_name,
                virtual_router,
                remote_msg_vpn_name,
                remote_location,
                remote_interface,
                x,
                helpers::getselect("*"));

            core_run!(request, core)

        } else {
            error!("error, did not find exactly one item matching query");
            process::exit(126);
        }

//        Ok(())
    }

    fn delete(msg_vpn: &str, bridge_name: &str, virtual_router: &str, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<SempMetaOnlyResponse, &'static str> {

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
            let remote_interface = &*x.remote_msg_vpn_interface().cloned().unwrap_or("".to_owned());
            let request = apiclient
                .default_api()
                .delete_msg_vpn_bridge_remote_msg_vpn(
                    msg_vpn,
                    bridge_name,
                    virtual_router,
                    remote_bridge_name,
                    remote_bridge_location,
                    remote_interface);
            core_run_meta!(request, core)

        } else {
            error!("error, did not find exactly one item matching query");
            process::exit(126);
        }

    }
}