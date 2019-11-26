use crate::fetch::Fetch;
use tokio_core::reactor::Core;
use solace_semp_client::apis::client::APIClient;
use hyper_tls::HttpsConnector;
use hyper::client::HttpConnector;
use futures::future::Future;
use solace_semp_client::models::{MsgVpnsResponse, MsgVpn, MsgVpnResponse, SempMetaOnlyResponse};
use crate::helpers;
use crate::provision::Provision;
use std::process::exit;
use serde::Serialize;
use crate::save::Save;
use crate::update::Update;
use crate::commandlineparser::CommandLineParser;
use clap::ArgMatches;
use std::borrow::Cow;

// fetch multple msgvpnsresponse
impl Fetch<MsgVpnsResponse> for MsgVpnsResponse {

    /// Returns a message vpn
    ///
    /// # Arguments
    ///
    /// * `select_key` - the key to fetch with, e.g: msgVpnName
    /// * `select_value` - the value to filter on e.g: myvpn
    ///
    /// # Example
    ///
    /// ```
    /// // Test for vpn fetch
    /// use fetch::Fetch;
    /// use solace_semp_client::models::MsgVpnsResponse;
    /// let (mut core, mut client) = solace_connect!();
    /// let result = MsgVpnsResponse::fetch("", "", "msgVpnName", "default", 10, "", "*", &mut core, &client);
    /// ```
    fn fetch(unused_1: &str, unused_2: &str, select_key: &str, select_value: &str, count: i32, cursor: &str, selector: &str, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<MsgVpnsResponse, &'static str> {
        let (wherev, selectv) = helpers::getwhere(select_key, select_value, selector);
        let request = apiclient
            .msg_vpn_api()
            .get_msg_vpns(count, cursor, wherev, selectv)
            .and_then(|vpn| {
                debug!("{:?}", vpn);
                futures::future::ok(vpn)
            });

        core_run!(request, core)

    }

}


// provision a vpn
impl Provision<MsgVpnResponse> for MsgVpnResponse {

    fn provision_with_file(unused_0: &str, unused_1: &str, file_name: &str, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<MsgVpnResponse, &'static str> {
        let deserialized = deserialize_file_into_type!(file_name, MsgVpn);
        match deserialized {
            Some(mut item) => {
//                if (&unused_0 != &"") {
//                    &item.set_msg_vpn_name(unused_0.to_owned());
//                }
                let request = apiclient
                    .default_api()
                    .create_msg_vpn(item, helpers::getselect("*"));
                core_run!(request, core)

            }
            _ => unimplemented!()
        }
    }

}


// save for vpns
impl Save<MsgVpnsResponse> for MsgVpnsResponse {
    fn save(dir: &str, data: &MsgVpnsResponse) -> Result<(), &'static str> where MsgVpnsResponse: Serialize {
        match data.data() {
            Some(vpns) => {
                for vpn in vpns {
                    match MsgVpn::save(dir, vpn) {
                        Ok(t) => debug!("success saving"),
                        Err(e) => error!("error writing: {:?}", e)
                    }

                }
                Ok(())
            },
            _ => {
                error!("no vpns");
                Err("no vpns")
            }
        }
    }
}

// save for single vpn
impl Save<MsgVpn> for MsgVpn {
    fn save(dir: &str, data: &MsgVpn) -> Result<(), &'static str> where MsgVpn: Serialize {
        let vpn_name = data.msg_vpn_name();
        let item_name = data.msg_vpn_name();
        debug!("save vpn: {:?}, {:?}", vpn_name, item_name);
        data.save_in_dir(dir, "vpn", &vpn_name, &Some(&"vpn".to_owned()));
        Ok(())
    }
}

// update vpn
impl Update<MsgVpnResponse> for MsgVpnResponse {

    fn update(msg_vpn: &str, file_name: &str, sub_item: &str, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<MsgVpnResponse, &'static str> {
        info!("updating message-vpn: {} from file", msg_vpn);
        let deserialized = deserialize_file_into_type!(file_name, MsgVpn);

        match deserialized {
            Some(mut item) => {
                item.set_msg_vpn_name(msg_vpn.to_owned());
                let request = apiclient
                    .default_api()
                    .update_msg_vpn(msg_vpn, item, helpers::getselect("*"));
                core_run!(request, core)

            }
            _ => unimplemented!()
        }
    }

    fn enabled(msg_vpn: &str, unused_1: &str, selector: Vec<&str>, state: bool, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<MsgVpnResponse, &'static str> {
        info!("changing enabled state to: {:?} for message-vpn: {}", state, msg_vpn);
        let mut vpn = MsgVpnsResponse::fetch("", "", "msgVpnName",msg_vpn, 10, "", "", core, apiclient)?;

        let mut tvpn = vpn.data().unwrap().clone();
        if tvpn.len() == 1 {
            info!("changing enabled state to: {}", state.to_string());
            let mut x = tvpn.pop().unwrap();
            x.set_enabled(state);
            let request = apiclient.default_api().update_msg_vpn(msg_vpn, x, helpers::getselect("*"));
            core_run!(request, core)

        } else {
            error!("error, did not find exactly one item matching query");
            exit(126);
        }

    }

    fn delete(msg_vpn: &str, unused_1: &str, sub_identifier: &str, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<SempMetaOnlyResponse, &'static str> {
        let request = apiclient.default_api().delete_msg_vpn(msg_vpn);
        core_run_meta!(request, core)
    }

}


mod tests {
    use solace_semp_client::models::{MsgVpn, MsgVpnResponse, MsgVpnsResponse};
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
        println!("vpn tests");

        let (mut core, mut client) = solace_connect!();

        println!("delete testvpn");
        let d = MsgVpnResponse::delete("testvpn", "", "", &mut core, &client);

        println!("create vpn");
        let v = MsgVpnResponse::provision_with_file("",
                                                    "",
                                                    "test_yaml/msg_vpn/vpn.yaml", &mut core,
                                                    &client);
        match v {
            Ok(vpn) => {
                assert_eq!(vpn.data().unwrap().msg_vpn_name().unwrap(), "testvpn");
            },
            Err(e) => {
                error!("cannot test");
            }
        }

        println!("fetch vpn");
        let f = MsgVpnsResponse::fetch("", "", "msgVpnName", "testvpn", 10, "", "*", &mut core, &client);
        match f {
            Ok(vpn) => {
                assert_eq!(vpn.data().unwrap().len(), 1);
            },
            Err(e) => {
                error!("cannot test")
            }
        }

        println!("update vpn");
        let u = MsgVpnResponse::update("testvpn", "test_yaml/msg_vpn/update.yaml", "", &mut core, &client);
        match u {
            Ok(vpn) => {
                assert_eq!(vpn.data().unwrap().max_connection_count().unwrap(), &1000);
            },
            Err(e) => {
                error!("cannot test");
            }
        }

        println!("save vpn");
        let mut vpn = MsgVpn::new();
        vpn.set_msg_vpn_name("tmpvpn".to_owned());
        MsgVpn::save("tmp", &vpn);
        let deserialized = deserialize_file_into_type!("tmp/tmpvpn/vpn/vpn.yaml", MsgVpn);
        match deserialized {
            Some(vpn) => {
                assert_eq!(vpn.msg_vpn_name().unwrap(), "tmpvpn");
            },
            _ => {
                error!("cannot save vpn");
            }
        }

        println!("save vpns response");
        let f = MsgVpnsResponse::fetch("testvpn", "testvpn", "msgVpnName", "*", 10, "", "*", &mut core, &client);
        match f {
            Ok(vpns) => {
                MsgVpnsResponse::save("tmp", &vpns);
                let default_vpn = deserialize_file_into_type!("tmp/default/vpn/vpn.yaml", MsgVpn);
                let testvpn_vpn = deserialize_file_into_type!("tmp/testvpn/vpn/vpn.yaml", MsgVpn);
                assert_eq!(default_vpn.unwrap().msg_vpn_name().unwrap(), "default");
                assert_eq!(testvpn_vpn.unwrap().msg_vpn_name().unwrap(), "testvpn");
            },
            Err(e) => {
                error!("unable to save vpns response");
            }
        }

        println!("disable vpn");
        let d = MsgVpnResponse::enabled("testvpn", "", vec![], false, &mut core, &client);
        match d {
            Ok(vpn) => {
                assert_eq!(vpn.data().unwrap().enabled().unwrap(), &false);
            },
            Err(e) => {
                error!("error in disable test");
            }
        }

        println!("delete vpn");
        let d = MsgVpnResponse::delete("testvpn", "", "", &mut core, &client);

    }
}