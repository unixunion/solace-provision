#[macro_use]
extern crate log;
extern crate env_logger;
extern crate tokio_request;
//extern crate rand;

#[macro_use]
mod macros;

use solace_semp_client::apis::client::APIClient;
use solace_semp_client::apis::configuration::Configuration;
use hyper::{Client, Body};
use tokio_core::reactor::Core;
use std::prelude::v1::Vec;
use colored::*;
use futures::{Future};
use clap::{Arg, App, load_yaml};
use serde_yaml;
use std::borrow::Cow;
use solace_semp_client::models::{MsgVpn, MsgVpnQueueSubscription, MsgVpnQueueSubscriptionResponse, MsgVpnQueueSubscriptionsResponse, MsgVpnSequencedTopicsResponse, MsgVpnSequencedTopic, MsgVpnSequencedTopicResponse, MsgVpnTopicEndpointResponse, MsgVpnTopicEndpointsResponse, MsgVpnAuthorizationGroupResponse, MsgVpnAuthorizationGroupsResponse, MsgVpnAuthorizationGroup, MsgVpnBridgesResponse, MsgVpnBridgeResponse, MsgVpnBridgeRemoteMsgVpnResponse, MsgVpnBridgeRemoteMsgVpnsResponse, AboutApiResponse, MsgVpnReplayLogResponse, MsgVpnReplayLog, MsgVpnDmrBridgesResponse, MsgVpnDmrBridgeResponse, MsgVpnDmrBridge, DmrClusterResponse, DmrClustersResponse, DmrClusterLinksResponse, DmrClusterLinkRemoteAddressesResponse, MsgVpnAclProfilePublishExceptionsResponse, MsgVpnAclProfileSubscribeExceptionsResponse, MsgVpnAclProfilePublishExceptionResponse, MsgVpnAclProfilePublishException, MsgVpnAclProfileSubscribeException, MsgVpnAclProfileSubscribeExceptionResponse, DmrClusterLinkResponse, DmrClusterLinkRemoteAddressResponse, DmrCluster, MsgVpnTopicEndpoint, MsgVpnBridgeRemoteMsgVpn, DmrClusterLink, DmrClusterLinkRemoteAddress};
use solace_semp_client::models::MsgVpnQueue;
use solace_semp_client::models::MsgVpnResponse;
use solace_semp_client::models::MsgVpnAclProfile;
use solace_semp_client::models::MsgVpnClientProfile;
use solace_semp_client::models::MsgVpnClientUsername;
use std::net::Shutdown::Read;
use clap::SubCommand;
use std::error::Error;
use solace_semp_client::models::MsgVpnBridge;
use std::mem;
use serde::{Serialize, Deserialize};
use crate::clientconfig::SolaceApiConfig;
use solace_semp_client::models::MsgVpnsResponse;
use crate::fetch::Fetch;
use crate::save::Save;
use crate::provision::Provision;
use crate::update::Update;
use solace_semp_client::models::MsgVpnQueuesResponse;
use solace_semp_client::models::MsgVpnAclProfilesResponse;
use solace_semp_client::models::MsgVpnClientProfilesResponse;
use solace_semp_client::models::MsgVpnClientUsernameResponse;
use solace_semp_client::models::MsgVpnClientUsernamesResponse;
use solace_semp_client::models::MsgVpnQueueResponse;
use solace_semp_client::models::MsgVpnAclProfileResponse;
use solace_semp_client::models::MsgVpnClientProfileResponse;
use std::path::Path;
use std::fs;
use std::fs::File;
use std::io::Write;
use native_tls::{TlsConnector, Certificate};
use hyper::client::HttpConnector;
use hyper_tls::HttpsConnector;
use core::borrow::Borrow;
use core::fmt;
use std::process::exit;
use tokio_request::str::get;
use crate::commandlineparser::CommandLineParser;

mod commandlineparser;
mod provision;
mod clientconfig;
mod helpers;
mod update;
mod fetch;
mod save;
//mod clientconnection;
mod objects;
mod argparsers;

mod test {
    use solace_semp_client::models::MsgVpn;

    #[test]
    fn new_event_threshold_empty() {
        // create a new vpn, then test if our new traits and functions are bound
        let mvpn = MsgVpn::new();
        let x = serde_yaml::to_string(&mvpn.event_connection_count_threshold());
        match x {
            Ok(svpn) => {
                info!("{}", svpn);
                assert_eq!("---\n~", svpn);
            },
            Err(e) => {}
        }

    }
}



fn main() -> Result<(), Box<Error>> {

    // initialize the logger
    env_logger::init();

    // load args.yaml
    let yaml = load_yaml!("args.yaml");
    let matches = App::from_yaml(yaml).get_matches();

    // get the config file name
    let config_file_name = matches.value_of("config").unwrap_or("default.yaml");
    info!("config_file: {:?}", config_file_name);

//    let count = matches.value_of("count").unwrap();
    let count = matches.value_of("count").unwrap().parse::<i32>().unwrap();
    debug!("count: {:?}", count);

    let mut output_dir  = matches.value_of("output").unwrap();
    let mut write_fetch_files = matches.is_present("save");
//    let mut write_fetch_files = matches.value_of("save").unwrap().parse::<bool>().unwrap();

    // future impl might use this.
    let mut cursor = Cow::Borrowed("");
    let mut select = matches.value_of("select").unwrap();

    // default emoji for OK / Error logging
    let mut ok_emoji = Cow::Borrowed("👍");
    let mut err_emoji = Cow::Borrowed("❌");

    // configure the http client
    let mut core = Core::new().unwrap();
    let handle = core.handle();

    let mut http = HttpConnector::new(4, &handle);
    http.enforce_http(false);

    let mut version_check_url = "";

    let mut tls = TlsConnector::builder()?;
    let mut sac = clientconfig::readconfig(config_file_name.to_owned());

    match sac {
        Ok(c) => {
            match c.certs {
                Some(certs) => {
                    for cert in certs.iter() {
                        info!("Adding certificate to chain");
                        let t: Certificate = Certificate::from_pem(cert.as_bytes())?;
                        tls.add_root_certificate(t);
                    }
                },
                None => info!("No certs")
            }
        },
        Err(e) => panic!("Error parsing config file")
    }

    let hyperclient = Client::configure()
        .connector(hyper_tls::HttpsConnector::from((http, tls.build()?))).build(&handle);

    // auth
    let auth = helpers::gencred("admin".to_owned(), "admin".to_owned());

    // the configuration for the APIClient
    let mut configuration = Configuration {
        base_path: "http://localhost:8080/SEMP/v2/config".to_owned(),
        user_agent: Some("Swagger-Codegen/2.10/rust".to_owned()),
        client: hyperclient,
        basic_auth: Some(auth),
        oauth_access_token: None,
        api_key: None,
    };



    let mut sac = clientconfig::readconfig(config_file_name.to_owned());
    match sac {
        Ok(sc) => {
            configuration.base_path = sc.host;
            let auth = helpers::gencred(sc.username, sc.password);
            configuration.basic_auth = Some(auth);
            ok_emoji = Cow::Owned(sc.ok_emoji);
            err_emoji = Cow::Owned(sc.err_emoji);
        },
        Err(e) => error!("error reading config: {}", e)
    }

    // the API Client from swagger spec
    let client = APIClient::new(configuration);

    //
    // VPN
    //
    if matches.is_present("vpn") {
        MsgVpn::parse(&matches, &mut core, &client);
    }

    //
    // QUEUE
    //
    if matches.is_present("queue") {
        MsgVpnQueue::parse(&matches, &mut core, &client);
    }

    //
    // ACL
    //
    if matches.is_present("acl-profile") {
        MsgVpnAclProfile::parse(&matches, &mut core, &client);
    }

    //
    // ACL profile publish exceptions
    //
    if matches.is_present("acl-profile-publish-exception") {
        MsgVpnAclProfilePublishException::parse(&matches, &mut core, &client);
    }

    //
    // ACL profile subscribe exceptions
    //
    if matches.is_present("acl-profile-subscribe-exception") {
        MsgVpnAclProfileSubscribeException::parse(&matches, &mut core, &client);
    }

    //
    // CLIENT-PROFILE
    //
    if matches.is_present("client-profile") {
        MsgVpnClientProfile::parse(&matches, &mut core, &client);
    }

    //
    // CLIENT-USERNAME
    //
    if matches.is_present("client-username") {
        MsgVpnClientUsername::parse(&matches, &mut core, &client);
    }

    //
    // QUEUE-SUBSCRIPTION
    //
    if matches.is_present("queue-subscription") {
        MsgVpnQueueSubscription::parse(&matches, &mut core, &client);
    }

    //
    // SEQUENCED-TOPICS
    //
    if matches.is_present("sequenced-topic") {
        MsgVpnSequencedTopic::parse(&matches, &mut core, &client);
    }

    //
    // TOPIC-ENDPOINTS
    //
    if matches.is_present("topic-endpoint") {
        MsgVpnTopicEndpoint::parse(&matches, &mut core, &client);
    }

    //
    // AUTHORIZATION-GROUPS
    //
    if matches.is_present("auth-group") {
        MsgVpnAuthorizationGroup::parse(&matches, &mut core, &client);
    }

    //
    // BRIDGE
    //
    if matches.is_present("bridge") {
        MsgVpnBridge::parse(&matches, &mut core, &client);
    }

    //
    // REMOTE BRIDGE
    //
    if matches.is_present("remote-bridge") {
        MsgVpnBridgeRemoteMsgVpn::parse(&matches, &mut core, &client);
    }

    //
    // REPLAY LOG
    //
    if matches.is_present("replay-log") {
        MsgVpnReplayLog::parse(&matches, &mut core, &client);
    }

    //
    // DMR BRIDGE
    //
    if matches.is_present("dmr-bridge") {
        MsgVpnDmrBridge::parse(&matches, &mut core, &client);
    }

    //
    // DMR CLUSTER
    //
    if matches.is_present("dmr-cluster") {
        DmrCluster::parse(&matches, &mut core, &client);
    }

    //
    // DMR CLUSTER LINK
    //
    if matches.is_present("dmr-cluster-link") {
        DmrClusterLink::parse(&matches, &mut core, &client);
    }

    //
    // DMR CLUSTER LINK REMOTE
    //
    if matches.is_present("dmr-cluster-link-remote") {
        DmrClusterLinkRemoteAddress::parse(&matches, &mut core, &client);
    }

    // other
    match matches.subcommand_name() {
        None => {
            println!("Please specify a subcommand, --help for more info");
            exit(1)
        },
        _  => {}
    }

    info!("{}", ok_emoji);

    Ok(())

}
