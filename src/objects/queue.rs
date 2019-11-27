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
use crate::commandlineparser::CommandLineParser;
use clap::ArgMatches;
use std::borrow::Cow;


// fetch queues
impl Fetch<MsgVpnQueuesResponse> for MsgVpnQueuesResponse {

    fn fetch(in_vpn: &str, unused_1: &str, select_key: &str, select_value: &str, count: i32, cursor: &str, selector: &str, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<MsgVpnQueuesResponse, &'static str> {
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

    fn provision_with_file(override_vpn_name: &str, unused_1: &str, file_name: &str, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<MsgVpnQueueResponse, &'static str> {

        let deserialized = deserialize_file_into_type!(file_name, MsgVpnQueue);
        match deserialized {
            Some(mut item) => {
                let mut vpn_name = maybe_set_vpn_name!(item, override_vpn_name.to_owned());
                let request = apiclient
                    .default_api()
                    .create_msg_vpn_queue(&vpn_name, item, helpers::getselect("*"));
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

    fn update(override_vpn_name: &str, file_name: &str, unused_1: &str, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<MsgVpnQueueResponse, &'static str> {

        let deserialized = deserialize_file_into_type!(file_name, MsgVpnQueue);

        match deserialized {
            Some(mut item) => {
                let mut vpn_name = maybe_set_vpn_name!(item, override_vpn_name.to_owned());
                let item_name = item.queue_name().cloned();
                let request = apiclient
                    .default_api()
                    .update_msg_vpn_queue(&vpn_name, &*item_name.unwrap(), item, helpers::getselect("*"));
                core_run!(request, core)

            }
            _ => unimplemented!()
        }
    }

    fn ingress(msg_vpn: &str, queue_name: &str, state: bool, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<MsgVpnQueueResponse, &'static str> {
        info!("retrieving current queue from appliance");
        let mut item = MsgVpnQueuesResponse::fetch(msg_vpn, "", "queueName", queue_name, 10, "", "", core, apiclient)?;
        let mut titem = item.data().unwrap().clone();

        if titem.len() == 1 {
            info!("changing ingress state to: {}", state.to_string());
            let mut x = titem.pop().unwrap();
            x.set_ingress_enabled(state);
            let request = apiclient.default_api().update_msg_vpn_queue(msg_vpn, queue_name, x, helpers::getselect("*"));
            core_run!(request, core)

        } else {
            error!("error, did not find exactly one item matching query");
            exit(126);
        }

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
    }

    fn delete(msg_vpn: &str, queue_name: &str, unused_1: &str, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<SempMetaOnlyResponse, &'static str> {
        let request = apiclient
            .default_api()
            .delete_msg_vpn_queue(msg_vpn, queue_name);
        core_run_meta!(request, core)

    }
}

mod tests {
    use solace_semp_client::models::{MsgVpn, MsgVpnResponse, MsgVpnsResponse, MsgVpnQueueResponse};
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
    use futures::future::err;

    #[test]
    fn provision() {
        println!("queue tests");

        // solace connection
        let (mut core, mut client) = solace_connect!();

        println!("delete testvpn");
        let d = MsgVpnResponse::delete("testvpn", "", "", &mut core, &client);

        println!("create vpn");
        let v = MsgVpnResponse::provision_with_file(
            "",
            "",
            "test_yaml/queue/vpn.yaml",
            &mut core,
            &client);
        match v {
            Ok(vpn) => {
                assert_eq!(vpn.data().unwrap().msg_vpn_name().unwrap(), "testvpn");
            },
            Err(e) => {
                error!("cannot test");
            }
        }


        // queue tests
        println!("create queue overriding vpn name");
        let q  = MsgVpnQueueResponse::provision_with_file(
            "testvpn",
            "",
            "test_yaml/queue/queue1.yaml",
            &mut core,
            &client);
        match q {
            Ok(queue) => {
                assert_eq!(queue.data().unwrap().queue_name().unwrap(), "queue1");
                // delete queue1 in testvpn
            },
            Err(e) => {
                error!("cannot test");
            }
        }

        // test updating queue
        let uq = MsgVpnQueueResponse::update(
            "testvpn",
            "test_yaml/queue/queue1_update.yaml",
            "",
            &mut core,
            &client);
        match uq {
            Ok(queue) => {
                assert_eq!(queue.data().unwrap().max_msg_size().unwrap(), &100);
            },
            Err(e) => {
                error!("cannot test");
            }
        }

        // change egress
        let uq = MsgVpnQueueResponse::egress(
            "testvpn",
            "queue1",
            false,
            &mut core,
            &client
        );
        match uq {
            Ok(queue) => {
                assert_eq!(queue.data().unwrap().egress_enabled().unwrap(), &false);
            },
            Err(e) => {
                error!("cannot test");
            }
        }

        // change ingress
        let uq = MsgVpnQueueResponse::ingress(
            "testvpn",
            "queue1",
            false,
            &mut core,
            &client
        );
        match uq {
            Ok(queue) => {
                assert_eq!(queue.data().unwrap().ingress_enabled().unwrap(), &false);
            },
            Err(e) => {
                error!("cannot test");
            }
        }


        let qd = MsgVpnQueueResponse::delete(
            "testvpn",
            "queue1",
            "",
            &mut core,
            &client
        );

        // test for not overriding name of vpn
        println!("create queue not overriding vpn name");
        let q  = MsgVpnQueueResponse::provision_with_file(
            "",
            "",
            "test_yaml/queue/queue2.yaml",
            &mut core,
            &client);
        match q {
            Ok(queue) => {
                assert_eq!(queue.data().unwrap().msg_vpn_name().unwrap(), "default");
                // delete queue2
                let qd = MsgVpnQueueResponse::delete(
                    "default",
                    "queue2",
                    "",
                    &mut core,
                    &client
                );
            },
            Err(e) => {
                error!("cannot test");
            }
        }


        // cleanup
        println!("delete vpn");
        let d = MsgVpnResponse::delete("testvpn", "", "", &mut core, &client);


    }
}
