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

    fn provision_with_file(unused_1: &str, unused_2: &str, file_name: &str, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<MsgVpnBridgeResponse, &'static str> {
        let file = std::fs::File::open(file_name).unwrap();
        let deserialized: Option<MsgVpnBridge> = serde_yaml::from_reader(file).unwrap();
        match deserialized {
            Some(mut item) => {
//                item.set_msg_vpn_name(in_vpn.to_owned());
                let request = apiclient
                    .default_api()
                    .create_msg_vpn_bridge(
                        &*item.msg_vpn_name().cloned().unwrap(),
                        item,
                        helpers::getselect("*"));
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

    fn update(unused_1: &str, file_name: &str, unused_2: &str, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<MsgVpnBridgeResponse, &'static str> {

        let deserialized = deserialize_file_into_type!(file_name, MsgVpnBridge);

        match deserialized {
            Some(mut item) => {
//                item.set_msg_vpn_name(msg_vpn.to_owned());
                let msg_vpn_name = item.msg_vpn_name().cloned();
                let item_name = item.bridge_name().cloned();
                let bridge_virtual_router = item.bridge_virtual_router().cloned();
                let request = apiclient
                    .default_api()
                    .update_msg_vpn_bridge(&*msg_vpn_name.unwrap(),
                                           &*item_name.unwrap(),
                                           &*bridge_virtual_router.unwrap(),
                                           item,
                                           helpers::getselect("*"));
                core_run!(request, core)

            }
            _ => unimplemented!()
        }
    }

    fn enabled(msg_vpn_name: &str, bridge_name: &str, selector: Vec<&str>, state: bool, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<MsgVpnBridgeResponse, &'static str> {
        println!("retrieving current bridge from appliance");
        let mut item = MsgVpnBridgesResponse::fetch(
            msg_vpn_name,
            bridge_name,
            "bridgeName",
            bridge_name,
            10,
            "",
            "",
            core,
            apiclient)?;

        // a mutable vec of temp items
        let mut titem = item.data().unwrap().clone();

        // ensure only 1 match
        if titem.len() == 1 {
            info!("changing enabled state to: {}", state.to_string());
            let mut x = titem.pop().unwrap();
            let virtual_router = &*x.bridge_virtual_router().cloned().unwrap();
            x.set_enabled(state);
            let request = apiclient.default_api().update_msg_vpn_bridge(msg_vpn_name,
                                                                        bridge_name,
                                                                        virtual_router,
                                                                        x,
                                                                        helpers::getselect("*"));

            core_run!(request, core)

        } else {
            error!("error, did not find exactly one item matching query");
            process::exit(126);
        }

    }

    fn delete(msg_vpn: &str, brige_name: &str, virtual_router: &str, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<SempMetaOnlyResponse, &'static str> {

        let request = apiclient.default_api().delete_msg_vpn_bridge(msg_vpn, brige_name, virtual_router);
        core_run_meta!(request, core)

    }
}

mod tests {
    use crate::provision::Provision;
    use solace_semp_client::models::{MsgVpnQueue, MsgVpnResponse, MsgVpnAclProfileClientConnectExceptionResponse, MsgVpnAclProfileClientConnectException, MsgVpnAclProfileClientConnectExceptionsResponse, MsgVpnAclProfileResponse, MsgVpnAclProfilePublishExceptionResponse, MsgVpnAclProfilePublishExceptionsResponse, MsgVpnAclProfileSubscribeExceptionResponse, MsgVpnAclProfileSubscribeExceptionsResponse, MsgVpnAclProfileSubscribeException, MsgVpnClientProfileResponse, MsgVpnAuthorizationGroupResponse, MsgVpnAuthorizationGroupsResponse, MsgVpnAuthorizationGroup, MsgVpnBridge, MsgVpnBridgeResponse, MsgVpnBridgesResponse};
    use tokio_core::reactor::Core;
    use hyper::client::HttpConnector;
    use native_tls::TlsConnector;
    use hyper::Client;
    use crate::helpers;
    use solace_semp_client::apis::configuration::Configuration;
    use solace_semp_client::apis::client::APIClient;
    use std::error::Error;
    use crate::update::Update;
    use crate::fetch::Fetch;
    use crate::save::Save;
    use futures::future::err;

    #[test]
    fn provision() {

        let (mut core, mut client) = solace_connect!();

        let test_vpn = "testvpn";

        println!("bridge delete testvpn");
        let d = MsgVpnResponse::delete(&test_vpn, "", "", &mut core, &client);

        println!("bridge create vpn");
        let v = MsgVpnResponse::provision_with_file(
            "",
            "",
            "test_yaml/bridge/vpn.yaml",
            &mut core,
            &client);

        match v {
            Ok(vpn) => assert_eq!(vpn.data().unwrap().msg_vpn_name().unwrap(), &test_vpn),
            Err(e) => error!("bridge cannot create testvpn")
        }

        // provision from file
        let b = MsgVpnBridgeResponse::provision_with_file(
            "",
            "",
            "test_yaml/bridge/bridge.yaml",
            &mut core,
            &client
        );

        match b {
            Ok(bridge) => assert_eq!(bridge.data().unwrap().bridge_name().unwrap(), "mybridge"),
            Err(e) => error!("bridge cannot provision from file")
        }

        let b2 = MsgVpnBridgeResponse::provision_with_file(
            "",
            "",
            "test_yaml/bridge/bridge2.yaml",
            &mut core,
            &client
        );


        // fetch
        let fb = MsgVpnBridgesResponse::fetch(
            &test_vpn,
            "*",
            "bridgeName",
            "*",
            10,
            "",
            "*",
            &mut core,
            &client
        );
        match fb {
            Ok(bridge) => {
                assert_eq!(bridge.data().unwrap().len(), 2);
                MsgVpnBridgesResponse::save("tmp/bridges", &bridge);
                let c = deserialize_file_into_type!("tmp/bridges/testvpn/bridge/mybridge.yaml", MsgVpnBridge);
                assert_eq!(c.unwrap().bridge_name().unwrap(), "mybridge");
                let c = deserialize_file_into_type!("tmp/bridges/testvpn/bridge/mybridge2.yaml", MsgVpnBridge);
                assert_eq!(c.unwrap().bridge_name().unwrap(), "mybridge2");
            },
            Err(e) => error!("bridge unable to fetch bridge")
        }



        // save single
        let mut bridge = MsgVpnBridge::new();
        bridge.set_bridge_name("tmpbridge".to_owned());
        bridge.set_bridge_virtual_router("primary".to_owned());
        bridge.set_msg_vpn_name("testvpn".to_owned());
        MsgVpnBridge::save(
            "tmp/bridge",
            &bridge
        );
        let deserialized = deserialize_file_into_type!(format!("tmp/bridge/{}/bridge/tmpbridge.yaml", test_vpn), MsgVpnBridge);
        match deserialized {
            Some(a) => {
                assert_eq!(a.bridge_name().unwrap(), "tmpbridge");
            }
            _ => {
                error!("bridge save error");
            }
        }






        // delete
        let db = MsgVpnBridgeResponse::delete(
            &test_vpn,
            "mybridge",
            "primary",
            &mut core,
            &client
        );
        match db {
            Ok(resp) => assert_eq!(resp.meta().response_code(), &200),
            Err(e) => error!("bridge delete failed")
        }

        // delete bridge2
        let db = MsgVpnBridgeResponse::delete(
            &test_vpn,
            "mybridge2",
            "primary",
            &mut core,
            &client
        );


        println!("bridge delete test vpn");
        let d = MsgVpnResponse::delete(&test_vpn, "", "", &mut core, &client);
    }

}