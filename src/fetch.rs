use solace_semp_client::apis::client::APIClient;
use solace_semp_client::models::{MsgVpn, MsgVpnTopicEndpointsResponse, MsgVpnSequencedTopic, MsgVpnQueueSubscriptionsResponse, MsgVpnSequencedTopicsResponse, MsgVpnAuthorizationGroup, MsgVpnAuthorizationGroupsResponse, MsgVpnBridgeRemoteMsgVpnsResponse, MsgVpnBridgeRemoteSubscriptionsResponse, MsgVpnBridgesResponse, MsgVpnBridgeRemoteSubscriptionResponse, MsgVpnBridgeTlsTrustedCommonNamesResponse, AboutApiResponse, MsgVpnReplayLogsResponse, MsgVpnReplayLogResponse, MsgVpnDmrBridgesResponse, MsgVpnDmrBridgeResponse, DmrClustersResponse, DmrClusterResponse, DmrClusterLinksResponse, DmrClusterLinkRemoteAddressesResponse, MsgVpnAclProfilePublishExceptionsResponse, MsgVpnAclProfileSubscribeExceptionsResponse, MsgVpnAclProfileClientConnectExceptionsResponse};
use tokio_core::reactor::Core;
use hyper_tls::HttpsConnector;
use hyper::client::HttpConnector;
use solace_semp_client::models::MsgVpnBridge;
use solace_semp_client::models::MsgVpnQueue;
use solace_semp_client::models::MsgVpnAclProfile;
use serde::Serialize;
use colored::Colorize;
use futures::{Future};
use futures::future::{AndThen, FutureResult};
use std::fs::File;
use serde::Deserialize;
use crate::helpers;
use solace_semp_client::models::MsgVpnsResponse;
use solace_semp_client::apis::Error;
use solace_semp_client::models::MsgVpnResponse;
use solace_semp_client::models::MsgVpnQueuesResponse;
use solace_semp_client::models::MsgVpnAclProfilesResponse;
use solace_semp_client::models::MsgVpnClientProfilesResponse;
use solace_semp_client::models::MsgVpnClientUsernamesResponse;
use std::process::exit;
use log::{info, warn, error, debug};
use std::path::Path;
use std::fs;
use std::io::Write;
use serde_json::Value;


// shared base trait for all solace fetch-able objects
pub trait Fetch<T> {

    fn fetch(in_vpn: &str, sub_item: &str, select_key: &str, select_value: &str, count: i32,  cursor: &str, selector: &str, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<T, &'static str>;

}

