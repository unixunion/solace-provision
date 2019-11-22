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
use solace_semp_client::models::{MsgVpnAclProfileSubscribeExceptionsResponse, MsgVpnAclProfileSubscribeExceptionResponse, MsgVpnAclProfileSubscribeException, SempMetaOnlyResponse};

// fetch ACL subscribe exceptions

impl Fetch<MsgVpnAclProfileSubscribeExceptionsResponse> for MsgVpnAclProfileSubscribeExceptionsResponse {
    fn fetch(in_vpn: &str, acl_profile_name: &str, select_key: &str, select_value: &str, count: i32, cursor: &str, selector: &str, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<MsgVpnAclProfileSubscribeExceptionsResponse, &'static str> {
        let (wherev, selectv) = helpers::getwhere(select_key, select_value, selector);
        let request = apiclient
            .default_api()
            .get_msg_vpn_acl_profile_subscribe_exceptions(in_vpn, acl_profile_name, count, cursor, wherev, selectv)
            .and_then(|acl| {
                futures::future::ok(acl)
            });

        core_run!(request, core)
    }
}

// provision ACL subscribe exception

impl Provision<MsgVpnAclProfileSubscribeExceptionResponse> for MsgVpnAclProfileSubscribeExceptionResponse {
    fn provision_with_file(msg_vpn_name: &str, item_name: &str, file_name: &str, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<MsgVpnAclProfileSubscribeExceptionResponse, &'static str> {
        let file = std::fs::File::open(file_name).unwrap();
        let deserialized: Option<MsgVpnAclProfileSubscribeException> = serde_yaml::from_reader(file).unwrap();
        match deserialized {
            Some(mut item) => {
                item.set_msg_vpn_name(msg_vpn_name.to_owned());
                let acl_profile_name = &*item.acl_profile_name().cloned().unwrap();
                let request = apiclient
                    .default_api()
                    .create_msg_vpn_acl_profile_subscribe_exception(msg_vpn_name, acl_profile_name, item, helpers::getselect("*"));
                core_run!(request, core)
            }
            _ => unimplemented!()
        }
    }

}

// save  ACL subscribe exception

impl Save<MsgVpnAclProfileSubscribeException> for MsgVpnAclProfileSubscribeException {
    fn save(dir: &str, data: &MsgVpnAclProfileSubscribeException) -> Result<(), &'static str> where MsgVpnAclProfileSubscribeException: Serialize {
        let mut hasher = sha1::Sha1::new();
        hasher.update(data.subscribe_exception_topic().unwrap().as_bytes());
        let s = hasher.digest().to_string();
        let topic_hash = Option::from(&s);
        let vpn_name = data.msg_vpn_name();
        let acl_profile_name = data.acl_profile_name();
        debug!("save acl-publish exception: {:?}, {:?}", vpn_name, acl_profile_name);
        data.save_in_dir(dir, &format!("acl/{}/subscribe-exceptions", &acl_profile_name.unwrap()), &vpn_name, &topic_hash);
        Ok(())
    }
}

// save ACL publish exceptions response

impl Save<MsgVpnAclProfileSubscribeExceptionsResponse> for MsgVpnAclProfileSubscribeExceptionsResponse {
    fn save(dir: &str, data: &MsgVpnAclProfileSubscribeExceptionsResponse) -> Result<(), &'static str> where MsgVpnAclProfileSubscribeExceptionsResponse: Serialize {
        match data.data() {
            Some(acls) => {
                for acl in acls {
                    match MsgVpnAclProfileSubscribeException::save(dir, acl) {
                        Ok(t) => debug!("success saving"),
                        Err(e) => error!("error writing: {:?}", e)
                    }

                }
                Ok(())
            },
            _ => {
                error!("no acls");
                Err("no acls")
            }
        }
    }
}

// update ACL subscribe exception

impl Update<MsgVpnAclProfileSubscribeExceptionResponse> for MsgVpnAclProfileSubscribeExceptionResponse {

    fn delete_by_sub_item(msg_vpn: &str, acl_profile_name: &str, topic_syntax: &str, subscribe_exception_topic: &str, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<SempMetaOnlyResponse, &'static str> {
        let request = apiclient.default_api().delete_msg_vpn_acl_profile_subscribe_exception(msg_vpn, acl_profile_name, topic_syntax, subscribe_exception_topic);
        core_run_meta!(request, core)
    }
}