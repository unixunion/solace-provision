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
use solace_semp_client::models::{MsgVpnReplayLogResponse, MsgVpnReplayLog, SempMetaOnlyResponse};

impl Fetch<MsgVpnReplayLogResponse> for MsgVpnReplayLogResponse {
    fn fetch(in_vpn: &str, replay_log_name: &str, unused_1: &str, unused_2: &str, count: i32, cursor: &str, selector: &str, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<MsgVpnReplayLogResponse, &'static str> {
        let (wherev, mut selectv) = helpers::getwhere("replayLogName", replay_log_name, selector);
        let request = apiclient
            .default_api()
            .get_msg_vpn_replay_log(in_vpn, replay_log_name,  selectv)
            .and_then(|item| {
                futures::future::ok(item)
            });
        core_run!(request, core)

    }
}


impl Provision<MsgVpnReplayLogResponse> for MsgVpnReplayLogResponse {

    fn provision_with_file(in_vpn: &str, unused_name: &str, file_name: &str, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<MsgVpnReplayLogResponse, &'static str> {
        let file = std::fs::File::open(file_name).unwrap();
        let deserialized: Option<MsgVpnReplayLog> = serde_yaml::from_reader(file).unwrap();
        match deserialized {
            Some(mut item) => {
                item.set_msg_vpn_name(in_vpn.to_owned());
                let request = apiclient
                    .default_api()
                    .create_msg_vpn_replay_log(in_vpn, item,  helpers::getselect("*"));
                core_run!(request, core)

            }
            _ => unimplemented!()
        }
    }
}


impl Save<MsgVpnReplayLog> for MsgVpnReplayLog {
    fn save(dir: &str, data: &MsgVpnReplayLog) -> Result<(), &'static str> where MsgVpnReplayLog: Serialize {
        let vpn_name = data.msg_vpn_name();
        let item_name = data.replay_log_name();
        debug!("save replay-log: {:?}, {:?}", vpn_name, item_name);
        data.save_in_dir(dir, "replay-log", &vpn_name, &item_name);
        Ok(())
    }
}



impl Update<MsgVpnReplayLogResponse> for MsgVpnReplayLogResponse {

    fn ingress(msg_vpn: &str, replay_log_name: &str, state: bool, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<MsgVpnReplayLogResponse, &'static str> {
        info!("retrieving current replay-log from appliance");
        let mut item = MsgVpnReplayLogResponse::fetch(msg_vpn, replay_log_name, "x","x", 10, "", "", core, apiclient)?;
        let mut titem = item.data().unwrap().clone();

        info!("changing ingress state to: {}", state.to_string());
        titem.set_ingress_enabled(state);
        let request = apiclient.default_api().update_msg_vpn_replay_log(msg_vpn, replay_log_name, titem, helpers::getselect("*"));
        core_run!(request, core)

    }

    fn egress(msg_vpn: &str, replay_log_name: &str, state: bool, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<MsgVpnReplayLogResponse, &'static str> {
        info!("retrieving current queue from appliance");
        let mut item = MsgVpnReplayLogResponse::fetch(msg_vpn, replay_log_name, "x","x", 10, "", "", core, apiclient)?;
        let mut titem = item.data().unwrap().clone();

        info!("changing egress state to: {}", state.to_string());
        titem.set_egress_enabled(state);

        let request = apiclient.default_api().update_msg_vpn_replay_log(msg_vpn, replay_log_name, titem, helpers::getselect("*"));
        core_run!(request, core)

    }

    fn delete(msg_vpn: &str, replay_log_name: &str, sub_identifier: &str, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<SempMetaOnlyResponse, &'static str> {
        info!("deleting replay-log: {}", replay_log_name);
        let request = apiclient.default_api().delete_msg_vpn_replay_log(msg_vpn, replay_log_name);
        core_run_meta!(request, core)

    }
}

