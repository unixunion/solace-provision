//
//mod tests {
//
//    use solace_semp_client::models::MsgVpn;
//    use solace_semp_client::models::MsgVpnQueue;
//    use crate::fetch;
//    use solace_semp_client::models::MsgVpnsResponse;
//
//    #[test]
//    fn it_works() {
//        // create a new vpn, then test if our new traits and functions are bound
//
//
//
//    }
//}
//
//
//
//use solace_semp_client::apis::client::APIClient;
//use solace_semp_client::models::MsgVpn;
//use tokio_core::reactor::Core;
//use hyper_tls::HttpsConnector;
//use hyper::client::HttpConnector;
//use solace_semp_client::models::MsgVpnBridge;
//use solace_semp_client::models::MsgVpnQueue;
//use solace_semp_client::models::MsgVpnAclProfile;
//use serde::Serialize;
//use colored::Colorize;
//use futures::{Future};
//use futures::future::AndThen;
//use std::fs::File;
//use serde::Deserialize;
//use crate::helpers;
//use crate::fetch;
//use solace_semp_client::models::MsgVpnsResponse;
//use solace_semp_client::apis::Error;
//use solace_semp_client::models::MsgVpnResponse;
//use solace_semp_client::models::MsgVpnQueuesResponse;
//use solace_semp_client::models::MsgVpnAclProfilesResponse;
//use solace_semp_client::models::MsgVpnClientProfilesResponse;
//use solace_semp_client::models::MsgVpnClientUsernamesResponse;
//
//// shared base trait for all solace fetch-able objects
//pub trait Provisionable<T> {
//    fn process_args(in_vpn: &str, item_name: &str, count: i32, cursor: &str, selector: &str,
//                    core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<(), &'static str>;
//
//}
//
//
//// fetch multple msgvpnsresponse
//impl Provisionable<MsgVpn> for MsgVpn {
//    fn process_args(in_vpn: &str, item_name: &str, count: i32, cursor: &str, selector: &str, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<(), &'static str> {
//        unimplemented!()
//    }
//}
//
//impl Provisionable<MsgVpnQueue> for MsgVpnQueue {
//    fn process_args(in_vpn: &str, item_name: &str, count: i32, cursor: &str, selector: &str, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<(), &'static str> {
//        unimplemented!()
//    }
//}
//
//impl Provisionable<MsgVpnAclProfilesResponse> for MsgVpnAclProfilesResponse {
//    fn process_args(in_vpn: &str, item_name: &str, count: i32, cursor: &str, selector: &str, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<(), &'static str> {
//        unimplemented!()
//    }
//}
//
//impl Provisionable<MsgVpnClientProfilesResponse> for MsgVpnClientProfilesResponse {
//    fn process_args(in_vpn: &str, item_name: &str, count: i32, cursor: &str, selector: &str, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<(), &'static str> {
//        unimplemented!()
//    }
//}
//
//impl Provisionable<MsgVpnClientUsernamesResponse> for MsgVpnClientUsernamesResponse {
//    fn process_args(in_vpn: &str, item_name: &str, count: i32, cursor: &str, selector: &str, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<(), &'static str> {
//        unimplemented!()
//    }
//}
//
//
//
