
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
    info!("{}", "generating credentials".green());
    let password: Option<String> = Some(password);
    BasicAuth::from((username, password ))
}
