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
use solace_semp_client::models::{MsgVpnTopicEndpointsResponse, MsgVpnTopicEndpointResponse, MsgVpnTopicEndpoint};
use std::process;

// fetch topic endpoint
impl Fetch<MsgVpnTopicEndpointsResponse> for MsgVpnTopicEndpointsResponse {
    fn fetch(in_vpn: &str, sub_item: &str, select_key: &str, select_value: &str, count: i32, cursor: &str, selector: &str, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<MsgVpnTopicEndpointsResponse, &'static str> {
        let (wherev, mut selectv) = helpers::getwhere(select_key, select_value, selector);

        let request = apiclient
            .topic_endpoint_api()
            .get_msg_vpn_topic_endpoints(in_vpn, count, cursor,  wherev, selectv)
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


// provision topic endpoint
impl Provision<MsgVpnTopicEndpointResponse> for MsgVpnTopicEndpointResponse {

    fn provision_with_file(in_vpn: &str, item_name: &str, file_name: &str, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<MsgVpnTopicEndpointResponse, &'static str> {
        let file = std::fs::File::open(file_name).unwrap();
        let deserialized: Option<MsgVpnTopicEndpoint> = serde_yaml::from_reader(file).unwrap();
        match deserialized {
            Some(mut item) => {
                item.set_msg_vpn_name(in_vpn.to_owned());
                let request = apiclient
                    .default_api()
                    .create_msg_vpn_topic_endpoint(in_vpn, item, helpers::getselect("*"));
                match core.run(request) {
                    Ok(response) => {
                        info!("{}",format!("{}", serde_yaml::to_string(&response.data().unwrap()).unwrap()));
                        Ok(response)
                    },
                    Err(e) => {
                        println!("provision error: {:?}", e);
                        exit(126);
                        Err("provision error")
                    }
                }
            }
            _ => unimplemented!()
        }
    }

}

// save Topic Endpoint

impl Save<MsgVpnTopicEndpoint> for MsgVpnTopicEndpoint {
    fn save(dir: &str, data: &MsgVpnTopicEndpoint) -> Result<(), &'static str> where MsgVpnTopicEndpoint: Serialize {
        let vpn_name = data.msg_vpn_name();
        let item_name = data.topic_endpoint_name();

        debug!("save topic-endpoint: {:?}, {:?}", vpn_name, item_name);
        data.save_in_dir(dir, "topic-endpoint", &vpn_name, &item_name);
        Ok(())
    }
}



impl Save<MsgVpnTopicEndpointsResponse> for MsgVpnTopicEndpointsResponse {
    fn save(dir: &str, data: &MsgVpnTopicEndpointsResponse) -> Result<(), &'static str> where MsgVpnTopicEndpointsResponse: Serialize {
        match data.data() {
            Some(items) => {
                for item in items {
                    match MsgVpnTopicEndpoint::save(dir, item) {
                        Ok(t) => debug!("success saving"),
                        Err(e) => error!("error writing: {:?}", e)
                    }
                }
                Ok(())
            },
            _ => {
                error!("no users");
                Err("no users")
            }
        }
    }
}

// update
// topic endpoint

impl Update<MsgVpnTopicEndpointResponse> for MsgVpnTopicEndpointResponse {

    fn ingress(msg_vpn: &str, item_name: &str, state: bool, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<(), &'static str> {
        info!("retrieving current topic endpoint from appliance");
        let mut item = MsgVpnTopicEndpointsResponse::fetch(msg_vpn, item_name, "topicEndpointName",item_name, 10, "", "", core, apiclient)?;
        let mut titem = item.data().unwrap().clone();

        if titem.len() == 1 {
            info!("changing ingress state to: {}", state.to_string());
            let mut x = titem.pop().unwrap();
            x.set_ingress_enabled(state);
            let r = core.run(apiclient.default_api().update_msg_vpn_topic_endpoint(msg_vpn, item_name, x, helpers::getselect("*")));
            match r {
                Ok(t) => info!("ingress successfully changed to {:?}", state),
                Err(e) => {
                    error!("error changing ingress state for vpn: {}, {:?}", item_name, e);
                    exit(126);
                }
            }

        } else {
            error!("error, did not find exactly one item matching query");
            process::exit(126);
        }

        Ok(())
    }

    fn egress(msg_vpn: &str, item_name: &str, state: bool, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<(), &'static str> {
        info!("retrieving current queue from appliance");
        let mut item = MsgVpnTopicEndpointsResponse::fetch(msg_vpn, item_name, "topicEndpointName",item_name, 10, "", "", core, apiclient)?;
        let mut titem = item.data().unwrap().clone();

        if titem.len() == 1 {
            info!("changing egress state to: {}", state.to_string());
            let mut x = titem.pop().unwrap();
            x.set_egress_enabled(state);
            let r = core.run(apiclient.default_api().update_msg_vpn_topic_endpoint(msg_vpn, item_name, x, helpers::getselect("*")));
            match r {
                Ok(t) => info!("egress successfully changed to {:?}", state),
                Err(e) => {
                    error!("error changing egress state for vpn: {}, {:?}", item_name, e);
                    exit(126);
                }
            }

        } else {
            error!("error, did not find exactly one item matching query");
            process::exit(126);
        }

        Ok(())
    }

    fn delete(msg_vpn: &str, item_name: &str, sub_identifier: &str, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<(), &'static str> {
        info!("deleting: {}", sub_identifier);
        let t = apiclient.default_api().delete_msg_vpn_topic_endpoint(msg_vpn, item_name);
        match core.run(t) {
            Ok(vpn) => {
                info!("topic-endpoint deleted");
                Ok(())
            },
            Err(e) => {
                error!("unable to delete topic-endpoint: {:?}", e);
                Err("unable to delete topic-endpoint")
            }
        }
    }
}