
mod tests {

    use solace_semp_client::models::MsgVpn;
    use crate::update::Update;
    use solace_semp_client::models::MsgVpnQueue;

    #[test]
    fn it_works() {}
}


use std::process;
use solace_semp_client::apis::client::APIClient;
use solace_semp_client::models::{MsgVpn, MsgVpnQueueSubscriptionResponse, MsgVpnSequencedTopicResponse, MsgVpnTopicEndpointResponse, MsgVpnTopicEndpointsResponse, MsgVpnAuthorizationGroupResponse, MsgVpnAuthorizationGroup, MsgVpnAuthorizationGroupsResponse, MsgVpnBridgeResponse, MsgVpnBridgesResponse, MsgVpnBridgeRemoteMsgVpnsResponse, MsgVpnBridgeRemoteMsgVpn, MsgVpnBridgeRemoteMsgVpnResponse, MsgVpnReplayLogResponse, MsgVpnAclProfilePublishException, MsgVpnAclProfileSubscribeException, MsgVpnAclProfilePublishExceptionResponse, MsgVpnAclProfileSubscribeExceptionResponse, SempMetaOnlyResponse};
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
    fn update(msg_vpn: &str, sub_item: &str, file_name: &str, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<T, &'static str> {
        unimplemented!()
    }

    // change the enabled state fo a object
    fn enabled(identifier_1: &str, identifier_2: &str, selector: Vec<&str>, state: bool, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<T, &'static str> {
        unimplemented!()
    }

    // ingress
    fn ingress(msg_vpn: &str, item_name: &str, state: bool, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<T, &'static str> {
        unimplemented!()
    }
    fn egress(msg_vpn: &str, item_name: &str, state: bool, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<T, &'static str> {
        unimplemented!()
    }
    // delete object
    fn delete(msg_vpn: &str, item_name: &str, sub_identifier: &str, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<SempMetaOnlyResponse, &'static str> {
        unimplemented!()
    }
    fn delete_by_sub_item(msg_vpn: &str, item_name: &str, sub_identifier: &str, sub_sub_identifier: &str, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<SempMetaOnlyResponse, &'static str> {
        unimplemented!()
    }
}

