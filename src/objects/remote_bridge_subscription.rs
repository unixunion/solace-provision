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
use solace_semp_client::models::{MsgVpnBridgeRemoteSubscriptionsResponse, MsgVpnBridgeRemoteSubscriptionResponse, MsgVpnBridgeRemoteSubscription};

impl Fetch<MsgVpnBridgeRemoteSubscriptionsResponse> for MsgVpnBridgeRemoteSubscriptionsResponse {
    fn fetch(in_vpn: &str, bridge_name: &str, remote_subscription_topic: &str, bridge_virtual_router: &str, count: i32, cursor: &str, selector: &str, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<MsgVpnBridgeRemoteSubscriptionsResponse, &'static str> {
        let (wherev, mut selectv) = helpers::getwhere("remoteSubscriptionTopic", remote_subscription_topic, selector);

        let request = apiclient
            .default_api()
            .get_msg_vpn_bridge_remote_subscriptions(in_vpn, bridge_name, bridge_virtual_router, count, cursor, wherev, selectv)
            .and_then(|item| {
                futures::future::ok(item)
            });
        core_run!(request, core)

    }
}

impl Provision<MsgVpnBridgeRemoteSubscriptionResponse> for MsgVpnBridgeRemoteSubscriptionResponse {

    fn provision_with_file(in_vpn: &str, bridge_name: &str, file_name: &str, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<MsgVpnBridgeRemoteSubscriptionResponse, &'static str> {
        let file = std::fs::File::open(file_name).unwrap();
        let deserialized: Option<MsgVpnBridgeRemoteSubscription> = serde_yaml::from_reader(file).unwrap();
        match deserialized {
            Some(mut item) => {
                item.set_msg_vpn_name(in_vpn.to_owned());
                let virtual_router = &*item.bridge_virtual_router().cloned().unwrap();
                let request = apiclient
                    .default_api()
                    .create_msg_vpn_bridge_remote_subscription(in_vpn, bridge_name, virtual_router, item, helpers::getselect("*"));
                core_run!(request, core)

            }
            _ => unimplemented!()
        }
    }
}

impl Save<MsgVpnBridgeRemoteSubscriptionsResponse> for MsgVpnBridgeRemoteSubscriptionsResponse {
}


impl Update<MsgVpnBridgeRemoteSubscriptionResponse> for MsgVpnBridgeRemoteSubscriptionResponse {
}
