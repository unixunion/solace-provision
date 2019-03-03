
mod tests {

    use solace_semp_client::models::MsgVpn;
    use solace_semp_client::models::MsgVpnQueue;
    use crate::configure;

    #[test]
    fn it_works() {
        // create a new vpn, then test if our new traits and functions are bound



    }
}



use solace_semp_client::apis::client::APIClient;
use solace_semp_client::models::MsgVpn;
use tokio_core::reactor::Core;
use hyper_tls::HttpsConnector;
use hyper::client::HttpConnector;
use solace_semp_client::models::MsgVpnBridge;
use solace_semp_client::models::MsgVpnQueue;
use solace_semp_client::models::MsgVpnAclProfile;
use serde::Serialize;
use colored::Colorize;
use futures::{Future};
use futures::future::AndThen;
use std::fs::File;
use serde::Deserialize;
use crate::helpers;
use crate::fetch;
use solace_semp_client::models::MsgVpnsResponse;
use solace_semp_client::apis::Error;
use solace_semp_client::models::MsgVpnResponse;
use solace_semp_client::models::MsgVpnQueuesResponse;
use solace_semp_client::models::MsgVpnAclProfilesResponse;
use solace_semp_client::models::MsgVpnClientProfilesResponse;
use solace_semp_client::models::MsgVpnClientUsernamesResponse;

// shared base trait for all solace fetch-able objects
pub trait Configure<T> {
    fn set_name(&self, name: &str) -> Self;
}


// fetch multple msgvpnsresponse
impl Configure<MsgVpn> for MsgVpn {
    fn set_name(&self, name: &str) -> Self {
        unimplemented!()
    }
}

impl Configure<MsgVpnQueue> for MsgVpnQueue {
    fn set_name(&self, name: &str) -> Self {
        unimplemented!()
    }
}

impl Configure<MsgVpnAclProfile> for MsgVpnAclProfile {
    fn set_name(&self, name: &str) -> Self {
        unimplemented!()
    }
}

impl Configure<MsgVpnClientProfile> for MsgVpnClientProfile {
    fn set_name(&self, name: &str) -> Self {
        unimplemented!()
    }
}

impl Configure<MsgVpnClientUsername> for MsgVpnClientUsername {
    fn set_name(&self, name: &str) -> Self {
        unimplemented!()
    }
}



