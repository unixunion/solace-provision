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
use solace_semp_client::models::{DmrClusterLinkRemoteAddressesResponse, DmrClusterLinkRemoteAddressResponse, DmrClusterLinkRemoteAddress};

impl Fetch<DmrClusterLinkRemoteAddressesResponse> for DmrClusterLinkRemoteAddressesResponse {
    fn fetch(cluster_name: &str, remote_node_name: &str, select_key: &str, select_value: &str, count: i32, cursor: &str, selector: &str, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<DmrClusterLinkRemoteAddressesResponse, &'static str> {
        let (wherev, mut selectv) = helpers::getwhere(select_key, select_value, selector);
        let request = apiclient
            .default_api()
            .get_dmr_cluster_link_remote_addresses(cluster_name, remote_node_name,  wherev, selectv)
            .and_then(|item| {
                futures::future::ok(item)
            });

        core_run!(request, core)

    }
}


impl Provision<DmrClusterLinkRemoteAddressResponse> for DmrClusterLinkRemoteAddressResponse {
    fn provision_with_file(unused_cluster_name: &str, unused_item_name: &str, file_name: &str, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<DmrClusterLinkRemoteAddressResponse, &'static str> {
        let file = std::fs::File::open(file_name).unwrap();
        let deserialized: Option<DmrClusterLinkRemoteAddress> = serde_yaml::from_reader(file).unwrap();
        match deserialized {
            Some(mut item) => {
//                item.set_dmr_cluster_name(item_name.to_owned());
                let cluster_name_file = &*item.dmr_cluster_name().cloned().unwrap();
                let remote_node_name = &*item.remote_node_name().cloned().unwrap();
                let request = apiclient
                    .default_api()
                    .create_dmr_cluster_link_remote_address(cluster_name_file, remote_node_name, item, helpers::getselect("*"));
                core_run!(request, core)

            }
            _ => unimplemented!()
        }
    }
}