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
use solace_semp_client::models::{MsgVpnSequencedTopicsResponse, MsgVpnSequencedTopic, MsgVpnSequencedTopicResponse, SempMetaOnlyResponse};

// fetch sequenced topic
impl Fetch<MsgVpnSequencedTopicsResponse> for MsgVpnSequencedTopicsResponse {
    fn fetch(in_vpn: &str, sub_item: &str, select_key: &str, select_value: &str ,count: i32, cursor: &str, selector: &str, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<MsgVpnSequencedTopicsResponse, &'static str> {
        let (wherev, mut selectv) = helpers::getwhere(select_key, select_value, selector);

        let request = apiclient
            .default_api()
            .get_msg_vpn_sequenced_topics(in_vpn, count, cursor,  wherev, selectv)
            .and_then(|item| {
                futures::future::ok(item)
            });

        core_run!(request, core)

    }
}

// provision sequenced topic
impl Provision<MsgVpnSequencedTopicResponse> for MsgVpnSequencedTopicResponse {

    fn provision_with_file(in_vpn: &str, item_name: &str, file_name: &str, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<MsgVpnSequencedTopicResponse, &'static str> {
        let file = std::fs::File::open(file_name).unwrap();
        let deserialized: Option<MsgVpnSequencedTopic> = serde_yaml::from_reader(file).unwrap();
        match deserialized {
            Some(mut item) => {
                item.set_msg_vpn_name(in_vpn.to_owned());
                let request = apiclient
                    .default_api()
                    .create_msg_vpn_sequenced_topic(in_vpn, item, helpers::getselect("*"));
                core_run!(request, core)

            }
            _ => unimplemented!()
        }
    }
}

// save Sequenced Topic

impl Save<MsgVpnSequencedTopic> for MsgVpnSequencedTopic {
    fn save(dir: &str, data: &MsgVpnSequencedTopic) -> Result<(), &'static str> where MsgVpnSequencedTopic: Serialize {
        let vpn_name = data.msg_vpn_name();
        let mut hasher = sha1::Sha1::new();
        match data.sequenced_topic() {
            Some(i) => {
                hasher.update(i.as_bytes());
                let s = hasher.digest().to_string();
                let item_name = Option::from(&s);
                debug!("save queue-subscription: {:?}, {:?}", vpn_name, item_name);
                data.save_in_dir(dir, "sequenced-topic", &vpn_name, &item_name);
                Ok(())
            },
            _ => {
                panic!("unable to get queue subscription from item")
            }
        }

    }
}

// save sequenced topics response
impl Save<MsgVpnSequencedTopicsResponse> for MsgVpnSequencedTopicsResponse {
    fn save(dir: &str, data: &MsgVpnSequencedTopicsResponse) -> Result<(), &'static str> where MsgVpnSequencedTopicsResponse: Serialize {
        match data.data() {
            Some(items) => {
                for item in items {
                    match MsgVpnSequencedTopic::save(dir, item) {
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


// update sequenced topic

impl Update<MsgVpnSequencedTopicResponse> for MsgVpnSequencedTopicResponse {

    fn delete(msg_vpn: &str, item_name: &str, sub_identifier: &str, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<SempMetaOnlyResponse, &'static str> {
        info!("deleting: {}", sub_identifier);
        let request = apiclient.default_api().delete_msg_vpn_sequenced_topic(msg_vpn, item_name);
        core_run_meta!(request, core)

    }
}