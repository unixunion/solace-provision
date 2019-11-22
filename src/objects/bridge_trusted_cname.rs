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
use solace_semp_client::models::MsgVpnBridgeTlsTrustedCommonNamesResponse;

impl Fetch<MsgVpnBridgeTlsTrustedCommonNamesResponse> for MsgVpnBridgeTlsTrustedCommonNamesResponse {
    fn fetch(in_vpn: &str, bridge_name: &str, common_name: &str, bridge_virtual_router: &str, count: i32, cursor: &str, selector: &str, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<MsgVpnBridgeTlsTrustedCommonNamesResponse, &'static str> {
        let (wherev, mut selectv) = helpers::getwhere("tlsTrustedCommonName", common_name, selector);

        let request = apiclient
            .default_api()
            .get_msg_vpn_bridge_tls_trusted_common_names(in_vpn, bridge_name, bridge_virtual_router, wherev, selectv)
            .and_then(|item| {
                futures::future::ok(item)
            });

        core_run!(request, core)

    }
}