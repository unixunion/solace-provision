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
use clap::ArgMatches;
use std::borrow::Cow;
use crate::commandlineparser::CommandLineParser;

//impl Fetch<DmrClusterResponse> for DmrClusterResponse {
//    fn fetch(unused_1: &str, dmr_cluster_name: &str, select_key: &str, select_value: &str, count: i32, cursor: &str, selector: &str, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<DmrClusterResponse, &'static str> {
//        let (wherev, mut selectv) = helpers::getwhere(select_key, select_value, selector);
//        let request = apiclient
//            .default_api()
//            .get_dmr_cluster(dmr_cluster_name, selectv)
//            .and_then(|item| {
//                futures::future::ok(item)
//            });
//
//        core_run!(request, core)
//
//    }
//}

// fetch all clusters by a key, where key is one of
//
impl Fetch<DmrClustersResponse> for DmrClustersResponse {
    fn fetch(unused_1: &str, unused_2: &str, select_key: &str, select_value: &str, count: i32, cursor: &str, selector: &str, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<DmrClustersResponse, &'static str> {
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

    fn update(unused_0: &str, unused_1: &str, file_name: &str, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<DmrClusterResponse, &'static str> {
//        info!("updating dmr-cluster: {} from file", unused_0);
        let deserialized = deserialize_file_into_type!(file_name, DmrCluster);

        match deserialized {
            Some(mut item) => {

                // TODO FIXME macro this override mechanism

//                let mut referenced_dmr_cluster_name = dmr_cluster_name;
//                let file_referenced_dmr_cluster_name = item.dmr_cluster_name().unwrap().clone();
//
//                // if name is overridden, set the same in the body,
//                // else set the referenced to the one from the body
//                if (&dmr_cluster_name != &"") {
//                    info!("overriding name to :{}", dmr_cluster_name);
//                    &item.set_dmr_cluster_name(dmr_cluster_name.to_owned());
//                } else {
//                    info!("using name from file");
//                    referenced_dmr_cluster_name = &*file_referenced_dmr_cluster_name; //body.msg_vpn_name().unwrap().clone().as_str();
//                }

                let request = apiclient
                    .default_api()
                    .update_dmr_cluster(&*item.dmr_cluster_name().cloned().unwrap(), item, helpers::getselect("*"));
                core_run!(request, core)

            }
            _ => unimplemented!()
        }
    }


    fn delete(cluster_name: &str, unused_1: &str, unused_2: &str, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<SempMetaOnlyResponse, &'static str> {
        let request = apiclient
            .default_api()
            .delete_dmr_cluster(cluster_name);
        core_run_meta!(request, core)

    }
}



mod tests {
    use solace_semp_client::models::{MsgVpn, MsgVpnResponse, MsgVpnsResponse, DmrClusterResponse, DmrCluster, DmrClustersResponse};
    use crate::provision::Provision;
    use solace_semp_client::models::MsgVpnQueue;
    use tokio_core::reactor::Core;
    use hyper::client::HttpConnector;
    use native_tls::TlsConnector;
    use hyper::Client;
    use crate::helpers;
    use solace_semp_client::apis::configuration::Configuration;
    use solace_semp_client::apis::client::APIClient;
    use std::error::Error;
    use crate::update::Update;
    use crate::fetch::Fetch;
    use crate::save::Save;

    //-> Result<(), Box<Error>>
    #[test]
    fn provision() {
        println!("dmr-cluster tests");

        let (mut core, mut client) = solace_connect!();

        println!("delete dmr-cluster");
        let d = DmrClusterResponse::delete("testdmr", "", "", &mut core, &client);

        println!("create dmr-cluster");
        let v = DmrClusterResponse::provision_with_file(
            "",
            "",
            "test_yaml/dmr_cluster/dmr-cluster.yaml", &mut core,
            &client
        );
        match v {
            Ok(vpn) => {
                assert_eq!(vpn.data().unwrap().dmr_cluster_name().unwrap(), "testdmr");
            },
            Err(e) => {
                error!("cannot test");
            }
        }

        println!("fetch dmr-cluster");
        let f = DmrClustersResponse::fetch(
            "",
            "",
            "dmrClusterName",
            "testdmr",
            10, "",
            "*",
            &mut core,
            &client
        );
        match f {
            Ok(dmr) => {
                assert_eq!(dmr.data().unwrap().len(), 1);
            }
            Err(e) => {
                error!("cannot test")
            }
        }

        println!("update dmr");
        let u = DmrClusterResponse::update(
            "testdmr",
            "",
            "test_yaml/dmr_cluster/update.yaml",
            &mut core,
            &client
        );
        match u {
            Ok(dmr) => {
                assert_eq!(dmr.data().unwrap().authentication_client_cert_enabled().unwrap(), &false);
            }
            Err(e) => {
                error!("cannot test");
            }
        }

        println!("save dmr");
        let mut dmr = DmrCluster::new();
        dmr.set_dmr_cluster_name("foo".to_owned());
        DmrCluster::save("tmp", &dmr);
        let deserialized = deserialize_file_into_type!("tmp/global/dmr-cluster/foo/dmr-cluster.yaml", DmrCluster);
        match deserialized {
            Some(vpn) => {
                assert_eq!(vpn.dmr_cluster_name().unwrap(), "foo");
            },
            _ => {
                error!("cannot save dmr");
            }
        }

        println!("save dmr response");
        let f = DmrClustersResponse::fetch(
            "",
            "",
            "dmrClusterName",
            "testdmr",
            10,
            "",
            "*",
            &mut core,
            &client
        );

        match f {
            Ok(dmrs) => {
                DmrClustersResponse::save("tmp", &dmrs);
                let dmr_ff = deserialize_file_into_type!("tmp/global/dmr-cluster/testdmr/dmr-cluster.yaml", DmrCluster);
                assert_eq!(dmr_ff.unwrap().dmr_cluster_name().unwrap(), "testdmr");
            },
            Err(e) => {
                error!("unable to save dmrs response");
            }
        }
//
//        println!("disable vpn");
//        let d = MsgVpnResponse::enabled("testvpn", "", vec![], false, &mut core, &client);
//        match d {
//            Ok(vpn) => {
//                assert_eq!(vpn.data().unwrap().enabled().unwrap(), &false);
//            },
//            Err(e) => {
//                error!("error in disable test");
//            }
//        }

        println!("delete dmr");
        let d = DmrClusterResponse::delete("testdmr", "", "", &mut core, &client);

    }
}