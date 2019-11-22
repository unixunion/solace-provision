use crate::fetch::Fetch;
use solace_semp_client::models::{MsgVpnQueuesResponse, MsgVpnQueueResponse, MsgVpnQueue, SempMetaOnlyResponse};
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


// fetch queues
impl Fetch<MsgVpnQueuesResponse> for MsgVpnQueuesResponse {

    fn fetch(in_vpn: &str, name: &str, select_key: &str, select_value: &str, count: i32, cursor: &str, selector: &str, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<MsgVpnQueuesResponse, &'static str> {
        let (wherev, selectv) = helpers::getwhere(select_key, select_value, selector);
        let request = apiclient
            .msg_vpn_api()
            .get_msg_vpn_queues(in_vpn, count, cursor, wherev, selectv)
            .and_then(|vpn| {
                futures::future::ok(vpn)
            });

        core_run!(request, core)

    }
}

// provision queue

impl Provision<MsgVpnQueueResponse> for MsgVpnQueueResponse {

    fn provision_with_file(in_vpn: &str, item_name: &str, file_name: &str, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<MsgVpnQueueResponse, &'static str> {
        let file = std::fs::File::open(file_name).unwrap();
        let deserialized: Option<MsgVpnQueue> = serde_yaml::from_reader(file).unwrap();
        match deserialized {
            Some(mut item) => {
                item.set_msg_vpn_name(in_vpn.to_owned());
                let request = apiclient
                    .default_api()
                    .create_msg_vpn_queue(in_vpn, item, helpers::getselect("*"));
                core_run!(request, core)

            }
            _ => unimplemented!()
        }
    }

}

// save for single queue
impl Save<MsgVpnQueue> for MsgVpnQueue {
    fn save(dir: &str, data: &MsgVpnQueue) -> Result<(), &'static str> where MsgVpnQueue: Serialize {
        let vpn_name = data.msg_vpn_name();
        let item_name = data.queue_name();
        debug!("save queue: {:?}, {:?}", vpn_name, item_name);
        data.save_in_dir(dir, "queue", &vpn_name, &item_name);
        Ok(())
    }
}

// save for queues response
impl Save<MsgVpnQueuesResponse> for MsgVpnQueuesResponse {
    fn save(dir: &str, data: &MsgVpnQueuesResponse) -> Result<(), &'static str> where MsgVpnQueuesResponse: Serialize {
        match data.data() {
            Some(items) => {
                for item in items {
                    match MsgVpnQueue::save(dir, item) {
                        Ok(t) => debug!("success saving"),
                        Err(e) => error!("error writing: {:?}", e)
                    }

                }
                Ok(())
            },
            _ => {
                error!("no queues");
                Err("no queues")
            }
        }
    }
}

// update queue

impl Update<MsgVpnQueueResponse> for MsgVpnQueueResponse {

    fn update(vpn_name: &str, file_name: &str, sub_item: &str, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<MsgVpnQueueResponse, &'static str> {
        let file = std::fs::File::open(file_name).unwrap();
        let deserialized: Option<MsgVpnQueue> = serde_yaml::from_reader(file).unwrap();

        match deserialized {
            Some(mut item) => {
                item.set_msg_vpn_name(vpn_name.to_owned());
                let item_name = item.queue_name().cloned();
                let request = apiclient
                    .default_api()
                    .update_msg_vpn_queue(vpn_name, &*item_name.unwrap(), item, helpers::getselect("*"));
                core_run!(request, core)

            }
            _ => unimplemented!()
        }
    }

    fn ingress(msg_vpn: &str, item_name: &str, state: bool, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<MsgVpnQueueResponse, &'static str> {
        info!("retrieving current queue from appliance");
        let mut item = MsgVpnQueuesResponse::fetch(msg_vpn, item_name, "queueName",item_name, 10, "", "", core, apiclient)?;
        let mut titem = item.data().unwrap().clone();

        if titem.len() == 1 {
            info!("changing ingress state to: {}", state.to_string());
            let mut x = titem.pop().unwrap();
            x.set_ingress_enabled(state);
            let request = apiclient.default_api().update_msg_vpn_queue(msg_vpn, item_name, x, helpers::getselect("*"));
            core_run!(request, core)

        } else {
            error!("error, did not find exactly one item matching query");
            exit(126);
        }

//        Ok(())
    }

    fn egress(msg_vpn: &str, item_name: &str, state: bool, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<MsgVpnQueueResponse, &'static str> {
        info!("retrieving current queue from appliance");
        let mut item = MsgVpnQueuesResponse::fetch(msg_vpn, item_name, "queueName",item_name, 10, "", "", core, apiclient)?;
        let mut titem = item.data().unwrap().clone();

        if titem.len() == 1 {
            info!("changing egress state to: {}", state.to_string());
            let mut x = titem.pop().unwrap();
            x.set_egress_enabled(state);
            let request = apiclient.default_api().update_msg_vpn_queue(msg_vpn, item_name, x, helpers::getselect("*"));
            core_run!(request, core)

        } else {
            error!("error, did not find exactly one item matching query");
            exit(126);
        }

//        Ok(())
    }

    fn delete(msg_vpn: &str, item_name: &str, sub_identifier: &str, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<SempMetaOnlyResponse, &'static str> {
        let request = apiclient
            .default_api()
            .delete_msg_vpn_queue(msg_vpn, item_name);
        core_run_meta!(request, core)

    }
}