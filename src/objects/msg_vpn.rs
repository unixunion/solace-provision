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

// fetch multple msgvpnsresponse
impl Fetch<MsgVpnsResponse> for MsgVpnsResponse {

    fn fetch(in_vpn: &str, name: &str, select_key: &str, select_value: &str, count: i32, cursor: &str, selector: &str, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<MsgVpnsResponse, &'static str> {
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

    fn provision_with_file(in_vpn: &str, item_name: &str, file_name: &str, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<MsgVpnResponse, &'static str> {
        let file = std::fs::File::open(file_name).unwrap();
        let deserialized: Option<MsgVpn> = serde_yaml::from_reader(file).unwrap();
        match deserialized {
            Some(mut item) => {
                item.set_msg_vpn_name(in_vpn.to_owned());
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
        data.save_in_dir(dir, "vpn", &vpn_name, &item_name);
        Ok(())
    }
}

// update vpn
impl Update<MsgVpnResponse> for MsgVpnResponse {

    fn update(msg_vpn: &str, file_name: &str, sub_item: &str, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<MsgVpnResponse, &'static str> {
        info!("updating message-vpn: {} from file", msg_vpn);
        let file = std::fs::File::open(file_name).unwrap();
        let deserialized: Option<MsgVpn> = serde_yaml::from_reader(file).unwrap();

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

    fn enabled(msg_vpn: &str, item_name: &str, selector: Vec<&str>, state: bool, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<MsgVpnResponse, &'static str> {
        info!("changing enabled state to: {:?} for message-vpn: {}", state, msg_vpn);
        let mut vpn = MsgVpnsResponse::fetch(item_name, item_name, "msgVpnName",item_name, 10, "", "", core, apiclient)?;

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


//        Ok(())

    }

    fn delete(msg_vpn: &str, item_name: &str, sub_identifier: &str,  core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<SempMetaOnlyResponse, &'static str> {
        let request = apiclient.default_api().delete_msg_vpn(item_name);
        core_run_meta!(request, core)

    }

}