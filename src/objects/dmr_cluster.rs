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
use solace_semp_client::models::{DmrClusterResponse, DmrClustersResponse, DmrCluster};

impl Fetch<DmrClusterResponse> for DmrClusterResponse {
    fn fetch(in_vpn: &str, dmr_cluster_name: &str, select_key: &str, select_value: &str, count: i32, cursor: &str, selector: &str, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<DmrClusterResponse, &'static str> {
        let (wherev, mut selectv) = helpers::getwhere(select_key, select_value, selector);
        let request = apiclient
            .default_api()
            .get_dmr_cluster(dmr_cluster_name, selectv)
            .and_then(|item| {
                futures::future::ok(item)
            });

        match core.run(request) {
            Ok(response) => {
                println!("{}",format!("{}", serde_yaml::to_string(&response.data().unwrap()).unwrap()));
                Ok(response)
            },
            Err(e) => {
                error!("error fetching: {:?}", e);
                panic!("fetch error: {:?}", e);
                Err("fetch error")
            }
        }
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

        match core.run(request) {
            Ok(response) => {
                println!("{}",format!("{}", serde_yaml::to_string(&response.data().unwrap()).unwrap()));
                Ok(response)
            },
            Err(e) => {
                error!("error fetching: {:?}", e);
                panic!("fetch error: {:?}", e);
                Err("fetch error")
            }
        }
    }
}

impl Provision<DmrClusterResponse> for DmrClusterResponse {
    fn provision_with_file(in_vpn: &str, item_name: &str, file_name: &str, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<DmrClusterResponse, &'static str> {
        let file = std::fs::File::open(file_name).unwrap();
        let deserialized: Option<DmrCluster> = serde_yaml::from_reader(file).unwrap();
        match deserialized {
            Some(mut item) => {
//                item.set_dmr_cluster_name(item_name.to_owned());
                let request = apiclient
                    .default_api()
                    .create_dmr_cluster(item, helpers::getselect("*"));
                match core.run(request) {
                    Ok(response) => {
                        info!("{}",format!("{}", serde_yaml::to_string(&response.data().unwrap()).unwrap()));
                        Ok(response)
                    },
                    Err(e) => {
                        error!("provision error: {:?}", e);
                        exit(126);
                        Err("provision error")
                    }
                }
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
        data.save_in_dir(dir, "dmr-cluster", &node_name, &item_name);
        Ok(())
    }
}

impl Update<DmrClusterResponse> for DmrClusterResponse {
    fn delete(cluster_name: &str, item_name: &str, sub_identifier: &str, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<(), &'static str> {
        let t = apiclient.default_api().delete_dmr_cluster(cluster_name);
        match core.run(t) {
            Ok(vpn) => {
                info!("dmr cluster deleted");
                Ok(())
            },
            Err(e) => {
                error!("unable to delete dmr cluster: {:?}", e);
                Err("unable to delete dmr cluster")
            }
        }
    }
}
