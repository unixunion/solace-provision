//#[macro_use]

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
use solace_semp_client::models::{DmrClusterLinksResponse, DmrClusterLinkResponse, DmrClusterLink};


impl Fetch<DmrClusterLinksResponse> for DmrClusterLinksResponse {
    fn fetch(cluster_name: &str, unused_sub_item: &str, select_key: &str, select_value: &str, count: i32, cursor: &str, selector: &str, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<DmrClusterLinksResponse, &'static str> {
        let (wherev, mut selectv) = helpers::getwhere(select_key, select_value, selector);
        let request = apiclient
            .default_api()
            .get_dmr_cluster_links(cluster_name, count, cursor, wherev, selectv)
            .and_then(|item| {
                futures::future::ok(item)
            });

        core_run!(request, core)

    }
}

impl Provision<DmrClusterLinkResponse> for DmrClusterLinkResponse {
    fn provision_with_file(unused_cluster_name: &str, unused_item_name: &str, file_name: &str, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<DmrClusterLinkResponse, &'static str> {
        let file = std::fs::File::open(file_name).unwrap();
        let deserialized: Option<DmrClusterLink> = serde_yaml::from_reader(file).unwrap();
        match deserialized {
            Some(mut item) => {
//                item.set_dmr_cluster_name(item_name.to_owned());
                let cluster_name_file = &*item.dmr_cluster_name().cloned().unwrap();
                let request = apiclient
                    .default_api()
                    .create_dmr_cluster_link(cluster_name_file, item, helpers::getselect("*"));
                core_run!(request, core)

            }
            _ => unimplemented!()
        }
    }
}

impl Save<DmrClusterLinksResponse> for DmrClusterLinksResponse {
    fn save(dir: &str, data: &DmrClusterLinksResponse) -> Result<(), &'static str> where DmrClusterLinksResponse: Serialize {
        match data.data() {
            Some(items) => {
                for item in items {
                    match DmrClusterLink::save(dir, item) {
                        Ok(t) => debug!("success saving"),
                        Err(e) => error!("error writing: {:?}", e)
                    }
                }
                Ok(())
            },
            _ => {
                error!("no dmr cluster links");
                Err("no dmr cluster links")
            }
        }
    }
}

impl Save<DmrClusterLink> for DmrClusterLink {
    fn save(dir: &str, data: &DmrClusterLink) -> Result<(), &'static str> where DmrClusterLink: Serialize {
        let name = &String::from("global");
        let node_name =  Some(name);
        let mut item_name =  data.remote_node_name().unwrap().clone();
        let item_name = Some(&item_name);
        let cluster_name = data.dmr_cluster_name().unwrap().clone();
        debug!("save dmr-cluster-link: {:?}, {:?}", node_name, item_name);
        data.save_in_dir(dir, &format!("dmr-cluster/{}/dmr-cluster-link", cluster_name), &node_name, &item_name);
        Ok(())
    }
}

impl Update<DmrClusterLinkResponse> for DmrClusterLinkResponse {

    fn update(unused_1: &str, unused_2: &str, file_name: &str, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<DmrClusterLinkResponse, &'static str> {

        let deserialized = deserialize_file_into_type!(file_name, DmrClusterLink);

        match deserialized {
            Some(mut item) => {

                let request = apiclient
                    .default_api()
                    .update_dmr_cluster_link(
                        &*item.dmr_cluster_name().cloned().unwrap(),
                        &*item.remote_node_name().cloned().unwrap(),
                        item,
                        helpers::getselect("*"));

                core_run!(request, core)

            }
            _ => unimplemented!()
        }
    }


    fn enabled(cluster_name: &str, remote_node_name: &str, selector: Vec<&str>, state: bool, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<DmrClusterLinkResponse, &'static str> {

        info!("changing enabled statet to {:?}", state);

        let mut cluster_link_response = DmrClusterLinksResponse::fetch(cluster_name, cluster_name, "remoteNodeName", remote_node_name, 10, "", "", core, apiclient)?;

        let mut cluster_link = cluster_link_response.data().unwrap().clone();

        if cluster_link.len() == 1 {
            let mut x = cluster_link.pop().unwrap();
            enabled!(x, state);
            let remote_node_name = &*x.remote_node_name().cloned().unwrap();

            let request = apiclient.default_api().update_dmr_cluster_link(cluster_name, remote_node_name, x, helpers::getselect("*"));
            core_run!(request, core)

        } else {
            error!("error, did not find exactly one item matching query");
            exit(126);
        }

    }
}

