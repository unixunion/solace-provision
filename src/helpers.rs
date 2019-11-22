use solace_semp_client::apis::configuration::BasicAuth;
use solace_semp_client::models::MsgVpn;
use solace_semp_client::models::MsgVpnQueue;
use colored::*;
use log::{info};
use std::fs::File;
use std::io::prelude::*;
use futures::future::Ok;
use std::path::Path;
use std::error::Error;
use serde::{Serialize, Deserialize};
use std::any::Any;


// generate a credential for basicauth
pub fn gencred(username: String, password: String) -> BasicAuth {
    let password: Option<String> = Some(password);
    BasicAuth::from((username, password ))
}

// build a where selector
pub fn getwhere(key: &str, name: &str, select: &str) -> (Vec<String>,Vec<String>) {
    let mut wherevec: Vec<String> = Vec::new();
    let whereitem = format!("{}=={}", key, name);
    wherevec.push(String::from(whereitem));

    let selectvec = getselect(select);

    debug!("generated wherevec: {:?} and selectvec: {:?}", &wherevec, &selectvec);

    (wherevec, selectvec)
}

pub fn getselect(select: &str) -> Vec<String> {
    // SEMP selector
    let mut selectvec: Vec<String> = Vec::new();
    selectvec.push(String::from(select));
    selectvec
}




