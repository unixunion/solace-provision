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
use solace_semp_client::models::{MsgVpnQueueSubscriptionsResponse, MsgVpnQueueSubscriptionResponse, MsgVpnQueueSubscription, SempMetaOnlyResponse};

// fetch queue subscription
impl Fetch<MsgVpnQueueSubscriptionsResponse> for MsgVpnQueueSubscriptionsResponse {
    fn fetch(in_vpn: &str, queue_name: &str, select_key: &str, select_value: &str, count: i32, cursor: &str, selector: &str, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<MsgVpnQueueSubscriptionsResponse, &'static str> {
        let (wherev, mut selectv) = helpers::getwhere(select_key, select_value, selector);

        let request = apiclient
            .default_api()
            .get_msg_vpn_queue_subscriptions(in_vpn, queue_name, count, cursor, wherev, selectv)
            .and_then(|item| {
                futures::future::ok(item)
            });

        core_run!(request, core)

    }
}

// provision queue subscription
impl Provision<MsgVpnQueueSubscriptionResponse> for MsgVpnQueueSubscriptionResponse {

    fn provision_with_file(in_vpn: &str, mut unused_1: &str, file_name: &str, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<MsgVpnQueueSubscriptionResponse, &'static str> {
        let file = std::fs::File::open(file_name).unwrap();
        let deserialized: Option<MsgVpnQueueSubscription> = serde_yaml::from_reader(file).unwrap();

        match deserialized {
            Some(mut item) => {
                let queue_name = &*item.queue_name().cloned().unwrap();
                item.set_msg_vpn_name(in_vpn.to_owned());
                let request = apiclient
                    .default_api()
                    .create_msg_vpn_queue_subscription(in_vpn, queue_name, item, helpers::getselect("*"));
                core_run!(request, core)

            }
            _ => unimplemented!()
        }
    }
}

// save queue Subscription

impl Save<MsgVpnQueueSubscription> for MsgVpnQueueSubscription {
    fn save(dir: &str, data: &MsgVpnQueueSubscription) -> Result<(), &'static str> where MsgVpnQueueSubscription: Serialize {
        let vpn_name = data.msg_vpn_name();
        let mut hasher = sha1::Sha1::new();
        match data.subscription_topic() {
            Some(i) => {
                hasher.update(i.as_bytes());
                let s = hasher.digest().to_string();
                let item_name = Option::from(&s);
                debug!("save queue-subscription: {:?}, {:?}", vpn_name, item_name);
                data.save_in_dir(dir, "queue-subscription", &vpn_name, &item_name);
                Ok(())
            },
            _ => {
                panic!("unable to get queue subscription from item")
            }
        }

    }
}

// save queue subscriptions responses

impl Save<MsgVpnQueueSubscriptionsResponse> for MsgVpnQueueSubscriptionsResponse {
    fn save(dir: &str, data: &MsgVpnQueueSubscriptionsResponse) -> Result<(), &'static str> where MsgVpnQueueSubscriptionsResponse: Serialize {
        match data.data() {
            Some(items) => {
                for item in items {
                    match MsgVpnQueueSubscription::save(dir, item) {
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

// update queue subscription

impl Update<MsgVpnQueueSubscriptionResponse> for MsgVpnQueueSubscriptionResponse {

    fn delete(msg_vpn: &str, queue_name: &str, subscription_topic: &str, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<SempMetaOnlyResponse, &'static str> {
        info!("deleting: {}", subscription_topic);
        let request = apiclient
            .default_api()
            .delete_msg_vpn_queue_subscription(msg_vpn, queue_name, subscription_topic);
        core_run_meta!(request, core)

    }
}

