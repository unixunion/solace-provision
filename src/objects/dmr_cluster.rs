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
use solace_semp_client::models::{DmrClusterResponse, DmrClustersResponse, DmrCluster, SempMetaOnlyResponse};

impl Fetch<DmrClusterResponse> for DmrClusterResponse {
    fn fetch(in_vpn: &str, dmr_cluster_name: &str, select_key: &str, select_value: &str, count: i32, cursor: &str, selector: &str, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<DmrClusterResponse, &'static str> {
        let (wherev, mut selectv) = helpers::getwhere(select_key, select_value, selector);
        let request = apiclient
            .default_api()
            .get_dmr_cluster(dmr_cluster_name, selectv)
            .and_then(|item| {
                futures::future::ok(item)
            });

        core_run!(request, core)

    }
}


// fetch all clusters by a key, where key is one of
//
impl Fetch<DmrClustersResponse> for DmrClustersResponse {
    fn fetch(in_vpn: &str, dmr_cluster_name: &str, select_key: &str, select_value: &str, count: i32, cursor: &str, selector: &str, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<DmrClustersResponse, &'static str> {
        let (wherev, mut selectv) = helpers::getwhere(select_key, select_value, selector);
        let request = apiclient
            .dmr_cluster_api()
            .get_dmr_clusters(count, cursor, wherev, selectv)
            .and_then(|item| {
                futures::future::ok(item)
            });

        core_run!(request, core)

    }
}

impl Provision<DmrClusterResponse> for DmrClusterResponse {
    fn provision_with_file(unused_1: &str, unused_2: &str, file_name: &str, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<DmrClusterResponse, &'static str> {
        let file = std::fs::File::open(file_name).unwrap();
        let deserialized: Option<DmrCluster> = serde_yaml::from_reader(file).unwrap();
        match deserialized {
            Some(mut item) => {
//                item.set_dmr_cluster_name(item_name.to_owned());
                let request = apiclient
                    .default_api()
                    .create_dmr_cluster(item, helpers::getselect("*"));
                core_run!(request, core)

            }
            _ => unimplemented!()
        }
    }
}


impl Save<DmrClustersResponse> for DmrClustersResponse {
    fn save(dir: &str, data: &DmrClustersResponse) -> Result<(), &'static str> where DmrClustersResponse: Serialize {
        match data.data() {
            Some(items) => {
                for item in items {
                    match DmrCluster::save(dir, item) {
                        Ok(t) => debug!("success saving"),
                        Err(e) => error!("error writing: {:?}", e)
                    }
                }
                Ok(())
            },
            _ => {
                error!("no dmr-bridges");
                Err("no dmr-bridges")
            }
        }
    }
}

impl Save<DmrCluster> for DmrCluster {
    fn save(dir: &str, data: &DmrCluster) -> Result<(), &'static str> where DmrCluster: Serialize {
        let name = &String::from("global");
        let node_name =  Some(name);
        let mut item_name =  data.dmr_cluster_name().unwrap().clone();
        let item_name = Some(&item_name);
        debug!("save dmr-cluster: {:?}, {:?}", node_name, item_name);
        data.save_in_dir(dir, &format!("dmr-cluster/{}", &item_name.unwrap().clone()), &node_name, &Some(&"dmr-cluster".to_owned()));
        Ok(())
    }
}

impl Update<DmrClusterResponse> for DmrClusterResponse {

    fn update(dmr_cluster_name: &str, sub_item: &str, file_name: &str, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<DmrClusterResponse, &'static str> {
        info!("updating dmr-cluster: {} from file", dmr_cluster_name);
        let deserialized = deserialize_file_into_type!(file_name, DmrCluster);

        match deserialized {
            Some(mut item) => {

                // TODO FIXME macro this override mechanism
                let mut referenced_dmr_cluster_name = dmr_cluster_name;
                let file_referenced_dmr_cluster_name = item.dmr_cluster_name().unwrap().clone();

                // if name is overridden, set the same in the body,
                // else set the referenced to the one from the body
                if (&dmr_cluster_name != &"") {
                    info!("overriding name to :{}", dmr_cluster_name);
                    &item.set_dmr_cluster_name(dmr_cluster_name.to_owned());
                } else {
                    info!("using name from file");
                    referenced_dmr_cluster_name = &*file_referenced_dmr_cluster_name; //body.msg_vpn_name().unwrap().clone().as_str();
                }

                let request = apiclient
                    .default_api()
                    .update_dmr_cluster(referenced_dmr_cluster_name, item, helpers::getselect("*"));
                core_run!(request, core)

            }
            _ => unimplemented!()
        }
    }


    fn delete(cluster_name: &str, item_name: &str, sub_identifier: &str, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<SempMetaOnlyResponse, &'static str> {
        let request = apiclient
            .default_api()
            .delete_dmr_cluster(cluster_name);
        core_run_meta!(request, core)

    }
}
