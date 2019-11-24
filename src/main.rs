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
use solace_semp_client::models::{MsgVpn, MsgVpnQueueSubscription, MsgVpnQueueSubscriptionResponse, MsgVpnQueueSubscriptionsResponse, MsgVpnSequencedTopicsResponse, MsgVpnSequencedTopic, MsgVpnSequencedTopicResponse, MsgVpnTopicEndpointResponse, MsgVpnTopicEndpointsResponse, MsgVpnAuthorizationGroupResponse, MsgVpnAuthorizationGroupsResponse, MsgVpnAuthorizationGroup, MsgVpnBridgesResponse, MsgVpnBridgeResponse, MsgVpnBridgeRemoteMsgVpnResponse, MsgVpnBridgeRemoteMsgVpnsResponse, AboutApiResponse, MsgVpnReplayLogResponse, MsgVpnReplayLog, MsgVpnDmrBridgesResponse, MsgVpnDmrBridgeResponse, MsgVpnDmrBridge, DmrClusterResponse, DmrClustersResponse, DmrClusterLinksResponse, DmrClusterLinkRemoteAddressesResponse, MsgVpnAclProfilePublishExceptionsResponse, MsgVpnAclProfileSubscribeExceptionsResponse, MsgVpnAclProfilePublishExceptionResponse, MsgVpnAclProfilePublishException, MsgVpnAclProfileSubscribeException, MsgVpnAclProfileSubscribeExceptionResponse, DmrClusterLinkResponse, DmrClusterLinkRemoteAddressResponse};
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

mod provision;
mod clientconfig;
mod helpers;
mod update;
mod fetch;
mod save;
//mod clientconnection;
mod objects;

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

    let count = matches.value_of("count").unwrap_or("10");
    let count = count.parse::<i32>().unwrap();
    debug!("count: {:?}", count);

    let mut output_dir = "output";
    let mut write_fetch_files = false;
    if matches.is_present("output") {
        output_dir = matches.value_of("output").unwrap();
        write_fetch_files = true;
        debug!("output_dir: {}", output_dir);
    }

    // future impl might use this.
    let mut cursor = Cow::Borrowed("");
    let mut select = "*";

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


//    let about_url = &configuration.base_path.clone();

    // the API Client from swagger spec
    let client = APIClient::new(configuration);


//    // version check
//    let about_url = format!("{}/about/api", &about_url);
//    let future = get(&about_url)
//        .header("User-Agent", "solace-provision")
//        .header("Authorization", &about_url)
//        .send(core.handle());
//    let result = core.run(future).expect("HTTP Request failed!");
//    println!(
//        "Site answered with status code {} and body\n{}",
//        result.status_code(),
//        result.body_str().unwrap_or("<No response body>")
//    );


//    let v = AboutApiResponse::fetch("", "", "", "", 0, "", "", &mut core, &client);
//    match v {
//        Ok(t) => {
//            info!("Appliance Version: {}", t.data().unwrap().semp_version());
//        },
//        Err(e) => {
//            panic!("unable to determine version")
//        }
//    }

    //
    // VPN
    //

    // check for the vpn subcommand
    if matches.is_present("vpn") {

        // source subcommand args into matches
        if let Some(matches) = matches.subcommand_matches("vpn") {

            let message_vpn = matches.value_of("message-vpn").unwrap();
            let update_item = matches.is_present("update");
            let shutdown_item = matches.is_present("shutdown");
            let no_shutdown_item = matches.is_present("no-shutdown");
            let fetch = matches.is_present("fetch");
            let delete = matches.is_present("delete");


            if update_item || shutdown_item || no_shutdown_item || fetch || delete || matches.is_present("file") {

                // early shutdown if not provisioning new
                if shutdown_item && update_item && matches.is_present("message-vpn") {
                    MsgVpnResponse::enabled(message_vpn, message_vpn, vec![],
                                            false, &mut core, &client)?;
                }

                // if file is passed, it means either provision or update.
                let file_name = matches.value_of("file");
                match file_name {
                    Some(file_name) => {
                        info!("using file: {:?}", file_name);

                        // provision / update from file
                        let file = std::fs::File::open(file_name).unwrap();
                        let deserialized: Option<MsgVpn> = serde_yaml::from_reader(file).unwrap();

                        match deserialized {
                            Some(mut item) => {
                                if update_item {
                                    MsgVpnResponse::update(message_vpn, file_name, "",
                                                           &mut core, &client);
                                } else {
                                    MsgVpnResponse::provision_with_file(message_vpn, "",
                                                                        file_name, &mut core,
                                                                        &client);
                                }
                            },
                            _ => unimplemented!()
                        }
                    },
                    None => {}
                }

                // late un-shutdown anything
                if no_shutdown_item {
                    MsgVpnResponse::enabled(message_vpn, message_vpn, vec![],
                                            true, &mut core, &client);
                }

                // finally if fetch is specified, we do this last.
                while fetch {
                    let data = MsgVpnsResponse::fetch(message_vpn, message_vpn, "msgVpnName", message_vpn, count, &*cursor.to_string(), select, &mut core, &client);

                    match data {
                        Ok(item) => {
                            if write_fetch_files {
                                MsgVpnsResponse::save(output_dir, &item);
                            }

                            let cq = item.meta().paging();
                            match cq {
                                Some(paging) => {
                                    info!("cq: {:?}", paging.cursor_query());
                                    cursor = Cow::Owned(paging.cursor_query().clone());
                                },
                                _ => {
                                    break
                                }
                            }
                        },
                        Err(e) => {
                            error!("error: {}", e)
                        }
                    }

                }


                if delete {
                    info!("deleting message vpn");
                    MsgVpnResponse::delete(message_vpn, message_vpn, "", &mut core, &client);
                }
            } else {
                error!("No operation was specified, see --help")
            }

        }

    }


    //
    // QUEUE
    //


    if matches.is_present("queue") {

        // source subcommand args into matches
        if let Some(matches) = matches.subcommand_matches("queue") {

            // get all args within the subcommand
            let message_vpn = matches.value_of("message-vpn").unwrap_or("undefined");
            let queue = matches.value_of("queue").unwrap_or("undefined");
            let update_item = matches.is_present("update");
            let shutdown_item = matches.is_present("shutdown");
            let no_shutdown_item = matches.is_present("no-shutdown");
            let mut shutdown_ingress = matches.is_present("shutdown-ingress");
            let mut no_shutdown_ingress = matches.is_present("no-shutdown-ingress");
            let mut shutdown_egress = matches.is_present("shutdown-egress");
            let mut no_shutdown_egress = matches.is_present("no-shutdown-egress");
            let fetch = matches.is_present("fetch");
            let delete = matches.is_present("delete");

            if update_item || shutdown_item || no_shutdown_item || shutdown_egress || no_shutdown_egress || shutdown_ingress || no_shutdown_ingress || fetch || delete || matches.is_present("file") {

                // early shutdown if not provisioning new
                if shutdown_item {
                    shutdown_ingress = true;
                    shutdown_egress = true;
//                    MsgVpnQueueResponse::ingress(message_vpn, queue,
//                                                 false, &mut core, &client);
//                    MsgVpnQueueResponse::egress(message_vpn, queue,
//                                                 false, &mut core, &client);
                }

                if shutdown_ingress {
                    MsgVpnQueueResponse::ingress(message_vpn, queue,
                                                 false, &mut core, &client);
                }

                if shutdown_egress {
                    MsgVpnQueueResponse::egress(message_vpn, queue,
                                                 false, &mut core, &client);
                }

                // if file is passed, it means either provision or update.
                let file_name = matches.value_of("file");
                match file_name {
                    Some(file_name) => {
                        info!("using file: {:?}", file_name);

                        // provision / update from file
                        let file = std::fs::File::open(file_name).unwrap();
                        let deserialized: Option<MsgVpnQueue> = serde_yaml::from_reader(file).unwrap();
                        match deserialized {
                            Some(mut item) => {
                                if update_item {
                                    MsgVpnQueueResponse::update(message_vpn, file_name, "",
                                                                &mut core, &client);
                                } else {
                                    MsgVpnQueueResponse::provision_with_file(message_vpn, "", file_name,
                                                                             &mut core, &client);
                                }
                            },
                            _ => unimplemented!()
                        }
                    },
                    None => {}
                }

                // late un-shutdown anything
                if no_shutdown_item {
                    no_shutdown_egress = true;
                    no_shutdown_ingress = true;
//                    MsgVpnQueueResponse::ingress(message_vpn, queue,
//                                                 true, &mut core, &client);
//                    MsgVpnQueueResponse::egress(message_vpn, queue,
//                                                 true, &mut core, &client);
                }

                if no_shutdown_ingress {
                    MsgVpnQueueResponse::ingress(message_vpn, queue,
                                                 true, &mut core, &client);
                }

                if no_shutdown_egress {
                    MsgVpnQueueResponse::egress(message_vpn, queue,
                                                true, &mut core, &client);
                }


                // finally if fetch is specified, we do this last.
                while fetch {
                    let data = MsgVpnQueuesResponse::fetch(message_vpn,
                                                           queue, "queueName", queue, count, &*cursor.to_string(), select,
                                                           &mut core, &client);


                    match data {
                        Ok(item) => {
                            if write_fetch_files {
                                MsgVpnQueuesResponse::save(output_dir, &item);
                            }

                            let cq = item.meta().paging();
                            match cq {
                                Some(paging) => {
                                    info!("cq: {:?}", paging.cursor_query());
                                    cursor = Cow::Owned(paging.cursor_query().clone());
                                },
                                _ => {
                                    break
                                }
                            }
                        },
                        Err(e) => {
                            error!("error: {}", e)
                        }
                    }


                }

                if delete {
                    info!("deleting queue");
                    MsgVpnQueueResponse::delete(message_vpn, queue, "", &mut core, &client);
                }
            } else {
                error!("No operation was specified, see --help")
            }

        }

    }




    //
    // ACL
    //

    if matches.is_present("acl-profile") {

        // source subcommand args into matches
        if let Some(matches) = matches.subcommand_matches("acl-profile") {

            // get all args within the subcommand
            let message_vpn = matches.value_of("message-vpn").unwrap_or("undefined");
            let acl = matches.value_of("acl-profile").unwrap_or("undefined");
            let update_item = matches.is_present("update");
            let fetch = matches.is_present("fetch");
            let delete = matches.is_present("delete");

            if update_item || fetch || delete || matches.is_present("file") {

                // if file is passed, it means either provision or update.
                let file_name = matches.value_of("file");
                match file_name {
                    Some(file_name) => {
                        info!("using file: {:?}", file_name);

                        // provision / update from file
                        let file = std::fs::File::open(file_name).unwrap();
                        let deserialized: Option<MsgVpnAclProfile> = serde_yaml::from_reader(file).unwrap();

                        match deserialized {
                            Some(mut item) => {
                                if update_item {
                                    MsgVpnAclProfileResponse::update(message_vpn, file_name, "",
                                                                     &mut core, &client);
                                } else {
                                    MsgVpnAclProfileResponse::provision_with_file(message_vpn, "", file_name,
                                                                                  &mut core, &client);
                                }
                            },
                            _ => unimplemented!()
                        }
                    },
                    None => {}
                }

                // finally if fetch is specified
                while fetch {
                    info!("fetching acl");
                    let data = MsgVpnAclProfilesResponse::fetch(message_vpn,
                                                                acl, "aclProfileName",acl, count,
                                                                &*cursor.to_string(), select, &mut core, &client);
                    match data {
                        Ok(item) => {
                            if write_fetch_files {
                                MsgVpnAclProfilesResponse::save(output_dir, &item);
                            }

                            let cq = item.meta().paging();
                            match cq {
                                Some(paging) => {
                                    info!("cq: {:?}", paging.cursor_query());
                                    cursor = Cow::Owned(paging.cursor_query().clone());
                                },
                                _ => {
                                    break
                                }
                            }
                        },
                        Err(e) => {
                            error!("error: {}", e)
                        }
                    }
                }

                if delete {
                    info!("deleting acl");
                    MsgVpnAclProfileResponse::delete(message_vpn, acl, "", &mut core, &client);
                }
            } else {
                error!("No operation was specified, see --help")
            }

        }

    }


    // ACL profile publish exceptions

    if matches.is_present("acl-profile-publish-exception") {

        // source subcommand args into matches
        if let Some(matches) = matches.subcommand_matches("acl-profile-publish-exception") {

            // get all args within the subcommand
            let message_vpn = matches.value_of("message-vpn").unwrap_or("undefined");
            let acl = matches.value_of("acl-profile").unwrap_or("undefined");
            let topic_syntax = matches.value_of("topic-syntax").unwrap_or("undefined");
            let topic = matches.value_of("topic").unwrap_or("undefined");
            let update_item = matches.is_present("update");
            let fetch = matches.is_present("fetch");
            let delete = matches.is_present("delete");

            if update_item || fetch || delete || matches.is_present("file") {

                // if file is passed, it means either provision or update.
                let file_name = matches.value_of("file");
                match file_name {
                    Some(file_name) => {
                        info!("using file: {:?}", file_name);

                        // provision / update from file
                        let file = std::fs::File::open(file_name).unwrap();
                        let deserialized: Option<MsgVpnAclProfilePublishException> = serde_yaml::from_reader(file).unwrap();

                        match deserialized {
                            Some(mut item) => {
//                                if update_item {
//                                    MsgVpnAclProfileResponse::update(message_vpn, file_name, "",
//                                                                     &mut core, &client);
//                                } else {
                                    MsgVpnAclProfilePublishExceptionResponse::provision_with_file(message_vpn, "", file_name,
                                                                                  &mut core, &client);
//                                }
                            },
                            _ => unimplemented!()
                        }
                    },
                    None => {}
                }

                // finally if fetch is specified
                while fetch {
                    info!("fetching acl-profile-publish-exception");
                    let data = MsgVpnAclProfilePublishExceptionsResponse::fetch(message_vpn,
                                                                acl, "aclProfileName",acl, count,
                                                                &*cursor.to_string(), select, &mut core, &client);
                    match data {
                        Ok(item) => {
                            if write_fetch_files {
                                MsgVpnAclProfilePublishExceptionsResponse::save(output_dir, &item);
                            }

                            let cq = item.meta().paging();
                            match cq {
                                Some(paging) => {
                                    info!("cq: {:?}", paging.cursor_query());
                                    cursor = Cow::Owned(paging.cursor_query().clone());
                                },
                                _ => {
                                    break
                                }
                            }
                        },
                        Err(e) => {
                            error!("error: {}", e)
                        }
                    }
                }

                if delete {
                    info!("deleting acl publish exception");
                    MsgVpnAclProfilePublishExceptionResponse::delete_by_sub_item(message_vpn, acl, topic_syntax, topic , &mut core, &client);
                }
            } else {
                error!("No operation was specified, see --help")
            }

        }

    }


    // ACL profile subscribe exceptions

    if matches.is_present("acl-profile-subscribe-exception") {

        // source subcommand args into matches
        if let Some(matches) = matches.subcommand_matches("acl-profile-subscribe-exception") {

            // get all args within the subcommand
            let message_vpn = matches.value_of("message-vpn").unwrap_or("undefined");
            let acl = matches.value_of("acl-profile").unwrap_or("undefined");
            let topic_syntax = matches.value_of("topic-syntax").unwrap_or("undefined");
            let topic = matches.value_of("topic").unwrap_or("undefined");
            let update_item = matches.is_present("update");
            let fetch = matches.is_present("fetch");
            let delete = matches.is_present("delete");

            if update_item || fetch || delete || matches.is_present("file") {

                // if file is passed, it means either provision or update.
                let file_name = matches.value_of("file");
                match file_name {
                    Some(file_name) => {
                        info!("using file: {:?}", file_name);

                        // provision / update from file
                        let file = std::fs::File::open(file_name).unwrap();
                        let deserialized: Option<MsgVpnAclProfileSubscribeException> = serde_yaml::from_reader(file).unwrap();
//
                        match deserialized {
                            Some(mut item) => {
//                                if update_item {
//                                    MsgVpnAclProfileResponse::update(message_vpn, file_name, "",
//                                                                     &mut core, &client);
//                                } else {
                                MsgVpnAclProfileSubscribeExceptionResponse::provision_with_file(message_vpn, "", file_name,
                                                                                  &mut core, &client);
//                                }
                            },
                            _ => unimplemented!()
                        }
                    },
                    None => {}
                }

                // finally if fetch is specified
                while fetch {
                    info!("fetching acl-profile-subscribe-exception");
                    let data = MsgVpnAclProfileSubscribeExceptionsResponse::fetch(message_vpn,
                                                                                acl, "aclProfileName",acl, count,
                                                                                &*cursor.to_string(), select, &mut core, &client);
                    match data {
                        Ok(item) => {
                            if write_fetch_files {
                                MsgVpnAclProfileSubscribeExceptionsResponse::save(output_dir, &item);
                            }

                            let cq = item.meta().paging();
                            match cq {
                                Some(paging) => {
                                    info!("cq: {:?}", paging.cursor_query());
                                    cursor = Cow::Owned(paging.cursor_query().clone());
                                },
                                _ => {
                                    break
                                }
                            }
                        },
                        Err(e) => {
                            error!("error: {}", e)
                        }
                    }
                }

                if delete {
                    info!("deleting acl subscribe exception");
                    MsgVpnAclProfileSubscribeExceptionResponse::delete_by_sub_item(message_vpn, acl, topic_syntax, topic, &mut core, &client);
                }
            } else {
                error!("No operation was specified, see --help")
            }

        }

    }


    //
    // CLIENT-PROFILE
    //

    if matches.is_present("client-profile") {

        // source subcommand args into matches
        if let Some(matches) = matches.subcommand_matches("client-profile") {

            // get all args within the subcommand
            let message_vpn = matches.value_of("message-vpn").unwrap_or("undefined");
            let client_profile = matches.value_of("client-profile").unwrap_or("undefined");
            let update_item = matches.is_present("update");
            let fetch = matches.is_present("fetch");
            let delete = matches.is_present("delete");

            if update_item || fetch || delete || matches.is_present("file") {

                // if file is passed, it means either provision or update.
                let file_name = matches.value_of("file");
                match file_name {
                    Some(file_name) => {
                        info!("using file: {:?}", file_name);

                        // provision / update from file
                        let file = std::fs::File::open(file_name).unwrap();
                        let deserialized: Option<MsgVpnClientProfile> = serde_yaml::from_reader(file).unwrap();

                        match deserialized {
                            Some(mut item) => {
                                if update_item {
                                    MsgVpnClientProfileResponse::update(message_vpn, file_name, "",
                                                                        &mut core, &client);
                                } else {
                                    MsgVpnClientProfileResponse::provision_with_file(message_vpn, "", file_name,
                                                                                     &mut core, &client);
                                }
                            },
                            _ => unimplemented!()
                        }
                    },
                    None => {}
                }

                // finally if fetch is specified
                while fetch {
                    info!("fetching client-profile");
                    let data = MsgVpnClientProfilesResponse::fetch(message_vpn, client_profile,
                                                                   "clientProfileName", client_profile, count,
                                                                   &*cursor.to_string(), select, &mut core, &client);
                    match data {
                        Ok(item) => {
                            if write_fetch_files {
                                MsgVpnClientProfilesResponse::save(output_dir, &item);
                            }

                            let cq = item.meta().paging();
                            match cq {
                                Some(paging) => {
                                    info!("cq: {:?}", paging.cursor_query());
                                    cursor = Cow::Owned(paging.cursor_query().clone());
                                },
                                _ => {
                                    break
                                }
                            }
                        },
                        Err(e) => {
                            error!("error: {}", e)
                        }
                    }
                }

                if delete {
                    info!("deleting client-profile");
                    MsgVpnClientProfileResponse::delete(message_vpn, client_profile, "", &mut core, &client);
                }
            } else {
                error!("No operation was specified, see --help")
            }

        }

    }






    //
    // CLIENT-USERNAME
    //


    if matches.is_present("client-username") {

        // source subcommand args into matches
        if let Some(matches) = matches.subcommand_matches("client-username") {

            // get all args within the subcommand
            let message_vpn = matches.value_of("message-vpn").unwrap_or("undefined");
            let client_username = matches.value_of("client-username").unwrap_or("undefined");
            let update_item = matches.is_present("update");
            let shutdown_item = matches.is_present("shutdown");
            let no_shutdown_item = matches.is_present("no-shutdown");
            let fetch = matches.is_present("fetch");
            let delete = matches.is_present("delete");

            if update_item || shutdown_item || no_shutdown_item || fetch || delete || matches.is_present("file") {

                // early shutdown if not provisioning new
                if shutdown_item && update_item && matches.is_present("client-username") && matches.is_present("message-vpn") {
                    MsgVpnClientUsernameResponse::enabled(message_vpn, client_username, vec![],
                                                          false, &mut core, &client);
                }


                // if file is passed, it means either provision or update.
                if matches.is_present("file") {
                    let file_name = matches.value_of("file");
                    if update_item {
                        MsgVpnClientUsernameResponse::update(message_vpn, file_name.unwrap(), "",
                                                             &mut core, &client);
                    } else {
                        MsgVpnClientUsernameResponse::provision_with_file(message_vpn, "", file_name.unwrap(),
                                                                          &mut core, &client);
                    }
                }

                // late un-shutdown anything
                if no_shutdown_item {
                    MsgVpnClientUsernameResponse::enabled(message_vpn, client_username, vec![],
                                                          true, &mut core, &client);
                }

                // finally if fetch is specified, we do this last.
                while fetch {
                    let data = MsgVpnClientUsernamesResponse::fetch(message_vpn, client_username,
                                                                    "clientUsername", client_username,count,
                                                                    &*cursor.to_string(), select, &mut core, &client);

                    match data {
                        Ok(item) => {
                            if write_fetch_files {
                                MsgVpnClientUsernamesResponse::save(output_dir, &item);
                            }

                            let cq = item.meta().paging();
                            match cq {
                                Some(paging) => {
                                    info!("cq: {:?}", paging.cursor_query());
                                    cursor = Cow::Owned(paging.cursor_query().clone());
                                },
                                _ => {
                                    break
                                }
                            }
                        },
                        Err(e) => {
                            error!("error: {}", e)
                        }
                    }

                }

                if delete {
                    info!("deleting client-username");
                    MsgVpnClientUsernameResponse::delete(message_vpn, client_username, "", &mut core, &client);
                }
            } else {
                error!("No operation was specified, see --help")
            }

        }

    }



    //
    // QUEUE-SUBSCRIPTION
    //


    if matches.is_present("queue-subscription") {

        // source subcommand args into matches
        if let Some(matches) = matches.subcommand_matches("queue-subscription") {

            // get all args within the subcommand
            let message_vpn = matches.value_of("message-vpn").unwrap_or("undefined");
            let queue = matches.value_of("queue").unwrap_or("undefined");
//            let queue = "undefined";
            let delete = matches.is_present("delete");
            let fetch = matches.is_present("fetch");
            let mut subscription = "";

            if matches.is_present("subscription") {
                subscription = matches.value_of("subscription").expect("error interpreting subscription");
                info!("subscription is: {}", subscription);
            }

            if fetch || delete || matches.is_present("file") {

                // if file is passed, it means either provision or update.
                if matches.is_present("file") {
                    let file_name = matches.value_of("file");
                    MsgVpnQueueSubscriptionResponse::provision_with_file(message_vpn, queue, file_name.unwrap(),
                                                                         &mut core, &client);
                }

                // finally if fetch is specified, we do this last.
                while fetch {
                    let data = MsgVpnQueueSubscriptionsResponse::fetch(message_vpn, queue, "queueName", queue, count, &*cursor.to_string(),
                                                                    select, &mut core, &client);

                    match data {
                        Ok(item) => {
                            if write_fetch_files {
                                MsgVpnQueueSubscriptionsResponse::save(output_dir, &item);
                            }

                            let cq = item.meta().paging();
                            match cq {
                                Some(paging) => {
                                    info!("cq: {:?}", paging.cursor_query());
                                    cursor = Cow::Owned(paging.cursor_query().clone());
                                },
                                _ => {
                                    break
                                }
                            }
                        },
                        Err(e) => {
                            error!("error: {}", e)
                        }
                    }

                }

                if delete {
                    info!("deleting queue-subscription");
                    MsgVpnQueueSubscriptionResponse::delete(message_vpn, queue, subscription, &mut core, &client);
                }
            } else {
                error!("No operation was specified, see --help")
            }

        }

    }




    //
    // SEQUENCED-TOPICS
    //


    if matches.is_present("sequenced-topic") {

        // source subcommand args into matches
        if let Some(matches) = matches.subcommand_matches("sequenced-topic") {

            // get all args within the subcommand
            let message_vpn = matches.value_of("message-vpn").unwrap_or("undefined");
            let delete = matches.is_present("delete");
            let fetch = matches.is_present("fetch");
            let mut sequenced_topic = "";

            if matches.is_present("sequenced-topic") {
                sequenced_topic = matches.value_of("sequenced-topic").unwrap_or("*");
                info!("sequenced-topic is: {}", sequenced_topic);
            }

            if fetch || delete || matches.is_present("file") {

                // if file is passed, it means either provision or update.
                if matches.is_present("file") {
                    let file_name = matches.value_of("file");
                    MsgVpnSequencedTopicResponse::provision_with_file(message_vpn, "", file_name.unwrap(),
                                                                      &mut core, &client);
                }

                // finally if fetch is specified, we do this last.
                while fetch {
                    let data = MsgVpnSequencedTopicsResponse::fetch(message_vpn, sequenced_topic, "sequencedTopic", sequenced_topic, count, &*cursor.to_string(),
                                                                       select, &mut core, &client);

                    match data {
                        Ok(item) => {
                            if write_fetch_files {
                                MsgVpnSequencedTopicsResponse::save(output_dir, &item);
                            }

                            let cq = item.meta().paging();
                            match cq {
                                Some(paging) => {
                                    info!("cq: {:?}", paging.cursor_query());
                                    cursor = Cow::Owned(paging.cursor_query().clone());
                                },
                                _ => {
                                    break
                                }
                            }
                        },
                        Err(e) => {
                            error!("error: {}", e)
                        }
                    }

                }

                if delete {
                    info!("deleting sequence-topic");
                    MsgVpnSequencedTopicResponse::delete(message_vpn, sequenced_topic, "", &mut core, &client);
                }
            } else {
                error!("No operation was specified, see --help")
            }

        }

    }



    // topic endpoint

    if matches.is_present("topic-endpoint") {

        // source subcommand args into matches
        if let Some(matches) = matches.subcommand_matches("topic-endpoint") {

            // get all args within the subcommand
            let message_vpn = matches.value_of("message-vpn").unwrap_or("undefined");
            let topic_endpoint = matches.value_of("topic-endpoint").unwrap_or("undefined");
            let update_item = matches.is_present("update");
            let shutdown_item = matches.is_present("shutdown");
            let no_shutdown_item = matches.is_present("no-shutdown");
            let mut shutdown_ingress = matches.is_present("shutdown-ingress");
            let mut no_shutdown_ingress = matches.is_present("no-shutdown-ingress");
            let mut shutdown_egress = matches.is_present("shutdown-egress");
            let mut no_shutdown_egress = matches.is_present("no-shutdown-egress");
            let fetch = matches.is_present("fetch");
            let delete = matches.is_present("delete");

            if update_item || shutdown_item || no_shutdown_item || shutdown_egress || no_shutdown_egress || shutdown_ingress || no_shutdown_ingress || fetch || delete || matches.is_present("file") {

                // early shutdown if not provisioning new
                if shutdown_item {
                    shutdown_ingress = true;
                    shutdown_egress = true;
                }

                if shutdown_ingress {
                    MsgVpnTopicEndpointResponse::ingress(message_vpn, topic_endpoint,
                                                 false, &mut core, &client);
                }

                if shutdown_egress {
                    MsgVpnTopicEndpointResponse::egress(message_vpn, topic_endpoint,
                                                false, &mut core, &client);
                }



                // if file is passed, it means either provision or update.
                if matches.is_present("file") {
                    let file_name = matches.value_of("file").unwrap();
                    if update_item {
                        MsgVpnTopicEndpointResponse::update(message_vpn, file_name, "",
                                                            &mut core, &client);
                    } else {
                        MsgVpnTopicEndpointResponse::provision_with_file(message_vpn, "", file_name,
                                                                         &mut core, &client);
                    }
                }


                // late un-shutdown anything
                if no_shutdown_item {
                    no_shutdown_egress = true;
                    no_shutdown_ingress = true;
                }

                if no_shutdown_ingress {
                    MsgVpnTopicEndpointResponse::ingress(message_vpn, topic_endpoint,
                                                 true, &mut core, &client);
                }

                if no_shutdown_egress {
                    MsgVpnTopicEndpointResponse::egress(message_vpn, topic_endpoint,
                                                true, &mut core, &client);
                }


                // finally if fetch is specified, we do this last.
                while fetch {
                    let data = MsgVpnTopicEndpointsResponse::fetch(message_vpn,
                                                           topic_endpoint, "topicEndpointName",topic_endpoint, count, &*cursor.to_string(), select,
                                                           &mut core, &client);


                    match data {
                        Ok(item) => {
                            if write_fetch_files {
                                MsgVpnTopicEndpointsResponse::save(output_dir, &item);
                            }

                            let cq = item.meta().paging();
                            match cq {
                                Some(paging) => {
                                    info!("cq: {:?}", paging.cursor_query());
                                    cursor = Cow::Owned(paging.cursor_query().clone());
                                },
                                _ => {
                                    break
                                }
                            }
                        },
                        Err(e) => {
                            error!("error: {}", e)
                        }
                    }


                }

                if delete {
                    info!("deleting topic endpoint");
                    MsgVpnTopicEndpointResponse::delete(message_vpn, topic_endpoint, "", &mut core, &client);
                }
            } else {
                error!("No operation was specified, see --help")
            }

        }

    }




    // authorization groups

    if matches.is_present("auth-group") {

        // source subcommand args into matches
        if let Some(matches) = matches.subcommand_matches("auth-group") {

            // get all args within the subcommand
            let message_vpn = matches.value_of("message-vpn").unwrap_or("undefined");
            let authorization_group = matches.value_of("auth-group").unwrap_or("undefined");
            let update_item = matches.is_present("update");
            let shutdown_item = matches.is_present("shutdown");
            let no_shutdown_item = matches.is_present("no-shutdown");
            let fetch = matches.is_present("fetch");
            let delete = matches.is_present("delete");

            if update_item || shutdown_item || fetch || delete || matches.is_present("file") {

                if shutdown_item {
                    MsgVpnAuthorizationGroupResponse::enabled(message_vpn, authorization_group, vec![],
                                            false, &mut core, &client);
                }

                // if file is passed, it means either provision or update.
                if matches.is_present("file") {
                    let file_name = matches.value_of("file").unwrap();
                    if update_item {
                        MsgVpnAuthorizationGroupResponse::update(message_vpn, file_name, "",
                                                                 &mut core, &client);
                    } else {
                        MsgVpnAuthorizationGroupResponse::provision_with_file(message_vpn, "", file_name,
                                                                              &mut core, &client);
                    }
                }


                if no_shutdown_item {
                    MsgVpnAuthorizationGroupResponse::enabled(message_vpn, authorization_group, vec![],
                                            true, &mut core, &client);
                }

                // finally if fetch is specified, we do this last.
                while fetch {
                    let data = MsgVpnAuthorizationGroupsResponse::fetch(message_vpn,
                                                                   authorization_group, "authorizationGroupName",authorization_group, count, &*cursor.to_string(), select,
                                                                   &mut core, &client);

                    match data {
                        Ok(item) => {
                            if write_fetch_files {
                                MsgVpnAuthorizationGroupsResponse::save(output_dir, &item);
                            }

                            let cq = item.meta().paging();
                            match cq {
                                Some(paging) => {
                                    info!("cq: {:?}", paging.cursor_query());
                                    cursor = Cow::Owned(paging.cursor_query().clone());
                                },
                                _ => {
                                    break
                                }
                            }
                        },
                        Err(e) => {
                            error!("error: {}", e)
                        }
                    }


                }

                if delete {
                    info!("deleting authorization group");
                    MsgVpnAuthorizationGroupResponse::delete(message_vpn, authorization_group, "", &mut core, &client);
                }
            } else {
                error!("No operation was specified, see --help")
            }

        }

    }


    // bridge

    if matches.is_present("bridge") {

        // source subcommand args into matches
        if let Some(matches) = matches.subcommand_matches("bridge") {

            // get all args within the subcommand
            let message_vpn = matches.value_of("message-vpn").unwrap_or("undefined");
            let bridge = matches.value_of("bridge").unwrap_or("undefined");
            let virtual_router = matches.value_of("virtual-router").unwrap_or("undefined");
            let update_item = matches.is_present("update");
            let shutdown_item = matches.is_present("shutdown");
            let no_shutdown_item = matches.is_present("no-shutdown");
            let fetch = matches.is_present("fetch");
            let delete = matches.is_present("delete");

            if shutdown_item || no_shutdown_item || fetch || delete || matches.is_present("file") {

                if shutdown_item {
                    MsgVpnBridgeResponse::enabled(message_vpn, bridge, vec![],
                                                              false, &mut core, &client);
                }

                // if file is passed, it means either provision or update.
                if matches.is_present("file") {
                    let file_name = matches.value_of("file").unwrap();
                    if update_item {
                        MsgVpnBridgeResponse::update(message_vpn, file_name, "", &mut core,
                                                                 &client);
                    } else {
                        MsgVpnBridgeResponse::provision_with_file(message_vpn, "", file_name,
                                                                  &mut core, &client);
                    }
                }


                if no_shutdown_item {
                    MsgVpnBridgeResponse::enabled(message_vpn, bridge, vec![],
                                                              true, &mut core, &client);
                }

                // finally if fetch is specified, we do this last.
                while fetch {
                    let data = MsgVpnBridgesResponse::fetch(message_vpn,
                                                                        bridge, "bridgeName",bridge, count, &*cursor.to_string(), select,
                                                                        &mut core, &client);

                    match data {
                        Ok(item) => {
                            if write_fetch_files {
                                MsgVpnBridgesResponse::save(output_dir, &item);
                            }

                            let cq = item.meta().paging();
                            match cq {
                                Some(paging) => {
                                    info!("cq: {:?}", paging.cursor_query());
                                    cursor = Cow::Owned(paging.cursor_query().clone());
                                },
                                _ => {
                                    break
                                }
                            }
                        },
                        Err(e) => {
                            error!("error: {}", e)
                        }
                    }


                }

                if delete {
                    info!("deleting authorization group");
                    MsgVpnBridgeResponse::delete(message_vpn, bridge, virtual_router, &mut core, &client);
                }
            } else {
                error!("No operation was specified, see --help")
            }

        }

    }


    // remote bridge

    if matches.is_present("remote-bridge") {

        // source subcommand args into matches
        if let Some(matches) = matches.subcommand_matches("remote-bridge") {

            // get all args within the subcommand
            let message_vpn = matches.value_of("message-vpn").unwrap_or("undefined");
            let bridge = matches.value_of("bridge").unwrap_or("undefined");
            let virtual_router = matches.value_of("virtual-router").unwrap_or("undefined");
            let update_item = matches.is_present("update");
            let shutdown_item = matches.is_present("shutdown");
            let no_shutdown_item = matches.is_present("no-shutdown");
            let fetch = matches.is_present("fetch");
            let delete = matches.is_present("delete");

            if shutdown_item || no_shutdown_item || fetch || delete || matches.is_present("file") {

                if shutdown_item {
                    MsgVpnBridgeRemoteMsgVpnResponse::enabled(message_vpn,
                                                              bridge,
                                                              vec![virtual_router],
                                                              false,
                                                              &mut core,
                                                              &client);
                }

                // if file is passed, it means either provision or update.
                if matches.is_present("file") {
                    let file_name = matches.value_of("file").unwrap();
                    if update_item {
                        MsgVpnBridgeRemoteMsgVpnResponse::update(message_vpn, file_name, "", &mut core,
                                                     &client);
                    } else {
                        MsgVpnBridgeRemoteMsgVpnResponse::provision_with_file(message_vpn,
                                                                              bridge,
                                                                              file_name,
                                                                              &mut core,
                                                                              &client);
                    }
                }


                if no_shutdown_item {
                    MsgVpnBridgeRemoteMsgVpnResponse::enabled(message_vpn,
                                                              bridge,
                                                              vec![virtual_router],
                                                              true,
                                                              &mut core,
                                                              &client);
                }

                // finally if fetch is specified, we do this last.
                while fetch {
                    let data = MsgVpnBridgeRemoteMsgVpnsResponse::fetch(
                        message_vpn,
                        bridge,
                        virtual_router,
                        bridge,
                        count,
                        &*cursor.to_string(),
                        select,
                        &mut core,
                        &client);

                    match data {
                        Ok(item) => {
                            if write_fetch_files {
                                MsgVpnBridgeRemoteMsgVpnsResponse::save(output_dir, &item);
                            }

                            let cq = item.meta().paging();
                            match cq {
                                Some(paging) => {
                                    info!("cq: {:?}", paging.cursor_query());
                                    cursor = Cow::Owned(paging.cursor_query().clone());
                                },
                                _ => {
                                    break
                                }
                            }
                        },
                        Err(e) => {
                            error!("error: {}", e)
                        }
                    }


                }

                if delete {
                    info!("deleting authorization group");
                    MsgVpnBridgeRemoteMsgVpnResponse::delete(message_vpn,
                                                             bridge,
                                                             virtual_router,
                                                             &mut core,
                                                             &client);
                }
            } else {
                error!("No operation was specified, see --help")
            }

        }

    }


    // replay log
    if matches.is_present("replay-log") {

        // source subcommand args into matches
        if let Some(matches) = matches.subcommand_matches("replay-log") {

            // get all args within the subcommand
            let message_vpn = matches.value_of("message-vpn").unwrap();
            let replay_log = matches.value_of("replay-log").unwrap();
            let update_item = matches.is_present("update");
            let shutdown_item = matches.is_present("shutdown");
            let no_shutdown_item = matches.is_present("no-shutdown");
            let mut shutdown_ingress = matches.is_present("shutdown-ingress");
            let mut no_shutdown_ingress = matches.is_present("no-shutdown-ingress");
            let mut shutdown_egress = matches.is_present("shutdown-egress");
            let mut no_shutdown_egress = matches.is_present("no-shutdown-egress");
            let fetch = matches.is_present("fetch");
            let delete = matches.is_present("delete");

            if update_item || shutdown_item || no_shutdown_item || shutdown_egress || no_shutdown_egress || shutdown_ingress || no_shutdown_ingress || fetch || delete || matches.is_present("file") {

                // early shutdown if not provisioning new
                if shutdown_item {
                    shutdown_ingress = true;
                    shutdown_egress = true;
                }

                if shutdown_ingress {
                    MsgVpnReplayLogResponse::ingress(message_vpn, replay_log,
                                                         false, &mut core, &client);
                }

                if shutdown_egress {
                    MsgVpnReplayLogResponse::egress(message_vpn, replay_log,
                                                        false, &mut core, &client);
                }



                // if file is passed, it means either provision or update.
                if matches.is_present("file") {
                    let file_name = matches.value_of("file").unwrap();
                    if update_item {
                        MsgVpnReplayLogResponse::update(message_vpn, file_name, "",
                                                            &mut core, &client);
                    } else {
                        MsgVpnReplayLogResponse::provision_with_file(message_vpn, "", file_name,
                                                                     &mut core, &client);
                    }
                }


                // late un-shutdown anything
                if no_shutdown_item {
                    no_shutdown_egress = true;
                    no_shutdown_ingress = true;
                }

                if no_shutdown_ingress {
                    MsgVpnReplayLogResponse::ingress(message_vpn, replay_log,
                                                         true, &mut core, &client);
                }

                if no_shutdown_egress {
                    MsgVpnReplayLogResponse::egress(message_vpn, replay_log,
                                                        true, &mut core, &client);
                }


                // finally if fetch is specified, we do this last.
                while fetch {
                    let data = MsgVpnReplayLogResponse::fetch(message_vpn,
                                                                   replay_log, "x","x", count, &*cursor.to_string(), select,
                                                                   &mut core, &client);


                    match data {
                        Ok(item) => {
                            if write_fetch_files {
                                MsgVpnReplayLog::save(output_dir, &item.data().unwrap());
                            }

                            let cq = item.meta().paging();
                            match cq {
                                Some(paging) => {
                                    info!("cq: {:?}", paging.cursor_query());
                                    cursor = Cow::Owned(paging.cursor_query().clone());
                                },
                                _ => {
                                    break
                                }
                            }
                        },
                        Err(e) => {
                            error!("error: {}", e)
                        }
                    }


                }

                if delete {
                    info!("deleting topic endpoint");
                    MsgVpnReplayLogResponse::delete(message_vpn, replay_log, "", &mut core, &client);
                }
            } else {
                error!("No operation was specified, see --help")
            }

        }

    }


    // dmr
    if matches.is_present("dmr-bridge") {

        // source subcommand args into matches
        if let Some(matches) = matches.subcommand_matches("dmr-bridge") {

            // get all args within the subcommand
            let message_vpn = matches.value_of("message-vpn").unwrap();
            let remote_vpn_name = matches.value_of("remote-vpn").unwrap();
            let update_item = matches.is_present("update");
            let shutdown_item = matches.is_present("shutdown");
            let no_shutdown_item = matches.is_present("no-shutdown");
            let mut shutdown_ingress = matches.is_present("shutdown-ingress");
            let mut no_shutdown_ingress = matches.is_present("no-shutdown-ingress");
            let mut shutdown_egress = matches.is_present("shutdown-egress");
            let mut no_shutdown_egress = matches.is_present("no-shutdown-egress");
            let fetch = matches.is_present("fetch");
            let delete = matches.is_present("delete");

            if update_item || shutdown_item || no_shutdown_item || shutdown_egress || no_shutdown_egress || shutdown_ingress || no_shutdown_ingress || fetch || delete || matches.is_present("file") {

                // early shutdown if not provisioning new
//                if shutdown_item {
//                    shutdown_ingress = true;
//                    shutdown_egress = true;
//                }

//                if shutdown_ingress {
//                    MsgVpnReplayLogResponse::ingress(message_vpn, replay_log,
//                                                     false, &mut core, &client);
//                }
//
//                if shutdown_egress {
//                    MsgVpnReplayLogResponse::egress(message_vpn, replay_log,
//                                                    false, &mut core, &client);
//                }



                // if file is passed, it means either provision or update.
                if matches.is_present("file") {
                    let file_name = matches.value_of("file").unwrap();
//                    if update_item {
//                        MsgVpnDmrBridgesResponse::update(message_vpn, file_name, "",
//                                                        &mut core, &client);
//                    } else {
                    MsgVpnDmrBridgeResponse::provision_with_file(message_vpn, "", file_name,
                                                                     &mut core, &client);
//                    }
                }


                // late un-shutdown anything
//                if no_shutdown_item {
//                    no_shutdown_egress = true;
//                    no_shutdown_ingress = true;
//                }
//
//                if no_shutdown_ingress {
//                    MsgVpnReplayLogResponse::ingress(message_vpn, replay_log,
//                                                     true, &mut core, &client);
//                }
//
//                if no_shutdown_egress {
//                    MsgVpnReplayLogResponse::egress(message_vpn, replay_log,
//                                                    true, &mut core, &client);
//                }


                // finally if fetch is specified, we do this last.
                while fetch {
                    let data = MsgVpnDmrBridgesResponse::fetch(message_vpn,
                                                              "", "remoteMsgVpnName",remote_vpn_name, count, &*cursor.to_string(), select,
                                                              &mut core, &client);


                    match data {
                        Ok(item) => {

                            if write_fetch_files {
                                MsgVpnDmrBridgesResponse::save(output_dir, &item);
                            }

                            let cq = item.meta().paging();
                            match cq {
                                Some(paging) => {
                                    info!("cq: {:?}", paging.cursor_query());
                                    cursor = Cow::Owned(paging.cursor_query().clone());
                                },
                                _ => {
                                    break
                                }
                            }
                        },
                        Err(e) => {
                            error!("error: {}", e)
                        }
                    }


                }

//                if delete {
//                    info!("deleting dmr-bridge");
//                    MsgVpnReplayLogResponse::delete(message_vpn, replay_log, "", &mut core, &client);
//                }
            } else {
                error!("No operation was specified, see --help")
            }

        }

    }


    // DMR Cluster
    if matches.is_present("dmr-cluster") {

        // source subcommand args into matches
        if let Some(matches) = matches.subcommand_matches("dmr-cluster") {

            // get all args within the subcommand
            let cluster_name = matches.value_of("cluster-name").unwrap_or("*");
            let update_item = matches.is_present("update");
            let shutdown_item = matches.is_present("shutdown");
            let no_shutdown_item = matches.is_present("no-shutdown");
            let mut shutdown_ingress = matches.is_present("shutdown-ingress");
            let mut no_shutdown_ingress = matches.is_present("no-shutdown-ingress");
            let mut shutdown_egress = matches.is_present("shutdown-egress");
            let mut no_shutdown_egress = matches.is_present("no-shutdown-egress");
            let fetch = matches.is_present("fetch");
            let delete = matches.is_present("delete");

            if update_item || shutdown_item || no_shutdown_item || shutdown_egress || no_shutdown_egress || shutdown_ingress || no_shutdown_ingress || fetch || delete || matches.is_present("file") {

                // early shutdown if not provisioning new
//                if shutdown_item {
//                    shutdown_ingress = true;
//                    shutdown_egress = true;
//                }

//                if shutdown_ingress {
//                    MsgVpnReplayLogResponse::ingress(message_vpn, replay_log,
//                                                     false, &mut core, &client);
//                }
//
//                if shutdown_egress {
//                    MsgVpnReplayLogResponse::egress(message_vpn, replay_log,
//                                                    false, &mut core, &client);
//                }



                // if file is passed, it means either provision or update.
                if matches.is_present("file") && !delete {
                    let file_name = matches.value_of("file").unwrap();
//                    if update_item {
//                        DmrClusterResponse::update(message_vpn, file_name, "",
//                                                        &mut core, &client);
//                    } else {
                    DmrClusterResponse::provision_with_file("", "", file_name,
                                                                 &mut core, &client);
//                    }
                }


                // late un-shutdown anything
//                if no_shutdown_item {
//                    no_shutdown_egress = true;
//                    no_shutdown_ingress = true;
//                }
//
//                if no_shutdown_ingress {
//                    MsgVpnReplayLogResponse::ingress(message_vpn, replay_log,
//                                                     true, &mut core, &client);
//                }
//
//                if no_shutdown_egress {
//                    MsgVpnReplayLogResponse::egress(message_vpn, replay_log,
//                                                    true, &mut core, &client);
//                }


                // finally if fetch is specified, we do this last."
                while fetch {
                    let data = DmrClustersResponse::fetch("",
                                                         cluster_name, "dmrClusterName",cluster_name, count, &*cursor.to_string(), select,
                                                               &mut core, &client);


                    match data {
                        Ok(item) => {

                            if write_fetch_files {
                                DmrClustersResponse::save(output_dir, &item);
                            }

                            let cq = item.meta().paging();
                            match cq {
                                Some(paging) => {
                                    info!("cq: {:?}", paging.cursor_query());
                                    cursor = Cow::Owned(paging.cursor_query().clone());
                                },
                                _ => {
                                    break
                                }
                            }
                        },
                        Err(e) => {
                            error!("error: {}", e)
                        }
                    }


                }

                if delete {
                    info!("deleting dmr-bridge");
                    DmrClusterResponse::delete(cluster_name, "", "", &mut core, &client);
                }
            } else {
                error!("No operation was specified, see --help")
            }

        }

    }


    // DMR Cluster Link
    if matches.is_present("dmr-cluster-link") {

        // source subcommand args into matches
        if let Some(matches) = matches.subcommand_matches("dmr-cluster-link") {

            // get all args within the subcommand
            let cluster_name = matches.value_of("cluster-name").unwrap_or("undefined");
            let update_item = matches.is_present("update");
            let shutdown_item = matches.is_present("shutdown");
            let no_shutdown_item = matches.is_present("no-shutdown");
            let fetch = matches.is_present("fetch");
            let delete = matches.is_present("delete");

            if update_item || shutdown_item || no_shutdown_item || fetch || delete || matches.is_present("file") {

                // early shutdown if not provisioning new
//                if shutdown_item {
//                    shutdown_ingress = true;
//                    shutdown_egress = true;
//                }

//                if shutdown_ingress {
//                    MsgVpnReplayLogResponse::ingress(message_vpn, replay_log,
//                                                     false, &mut core, &client);
//                }
//
//                if shutdown_egress {
//                    MsgVpnReplayLogResponse::egress(message_vpn, replay_log,
//                                                    false, &mut core, &client);
//                }

                if shutdown_item && update_item && matches.is_present("cluster-name") {
                    DmrClusterLinkResponse::enabled(cluster_name, matches.value_of("remote-node-name").unwrap_or(""), vec![],
                                            false, &mut core, &client)?;
                }


                // if file is passed, it means either provision or update.
                if matches.is_present("file") {
                    let file_name = matches.value_of("file").unwrap();
////                    if update_item {
////                        DmrClusterResponse::update(message_vpn, file_name, "",
////                                                        &mut core, &client);
////                    } else {
                    DmrClusterLinkResponse::provision_with_file(cluster_name, "", file_name,
                                                            &mut core, &client);
//                    }
                }


                // late un-shutdown anything
//                if no_shutdown_item {
//                    no_shutdown_egress = true;
//                    no_shutdown_ingress = true;
//                }
//
//                if no_shutdown_ingress {
//                    MsgVpnReplayLogResponse::ingress(message_vpn, replay_log,
//                                                     true, &mut core, &client);
//                }
//
//                if no_shutdown_egress {
//                    MsgVpnReplayLogResponse::egress(message_vpn, replay_log,
//                                                    true, &mut core, &client);
//                }


                // finally if fetch is specified, we do this last."
                while fetch {
                    let data = DmrClusterLinksResponse::fetch(cluster_name,
                                                          cluster_name, "dmrClusterName",cluster_name, count, &*cursor.to_string(), select,
                                                          &mut core, &client);


                    match data {
                        Ok(item) => {

                            if write_fetch_files {
                                DmrClusterLinksResponse::save(output_dir, &item);
                            }

                            let cq = item.meta().paging();
                            match cq {
                                Some(paging) => {
                                    info!("cq: {:?}", paging.cursor_query());
                                    cursor = Cow::Owned(paging.cursor_query().clone());
                                },
                                _ => {
                                    break
                                }
                            }
                        },
                        Err(e) => {
                            error!("error: {}", e)
                        }
                    }


                }

                if no_shutdown_item && update_item && matches.is_present("cluster_name") {
                    DmrClusterLinkResponse::enabled(cluster_name, "", vec![],
                                                    true, &mut core, &client)?;
                }

//                if delete {
//                    info!("deleting dmr-bridge");
//                    MsgVpnReplayLogResponse::delete(message_vpn, replay_log, "", &mut core, &client);
//                }
            } else {
                error!("No operation was specified, see --help")
            }

        }

    }


    // DMR Cluster Link Remote
    if matches.is_present("dmr-cluster-link-remote") {

        // source subcommand args into matches
        if let Some(matches) = matches.subcommand_matches("dmr-cluster-link-remote") {

            // get all args within the subcommand
            let cluster_name = matches.value_of("cluster-name").unwrap_or("undefined");
            let remote_node_name = matches.value_of("remote-node-name").unwrap_or("undefined");
            let update_item = matches.is_present("update");
            let shutdown_item = matches.is_present("shutdown");
            let no_shutdown_item = matches.is_present("no-shutdown");
            let mut shutdown_ingress = matches.is_present("shutdown-ingress");
            let mut no_shutdown_ingress = matches.is_present("no-shutdown-ingress");
            let mut shutdown_egress = matches.is_present("shutdown-egress");
            let mut no_shutdown_egress = matches.is_present("no-shutdown-egress");
            let fetch = matches.is_present("fetch");
            let delete = matches.is_present("delete");

            if update_item || shutdown_item || no_shutdown_item || shutdown_egress || no_shutdown_egress || shutdown_ingress || no_shutdown_ingress || fetch || delete || matches.is_present("file") {

                // early shutdown if not provisioning new
//                if shutdown_item {
//                    shutdown_ingress = true;
//                    shutdown_egress = true;
//                }

//                if shutdown_ingress {
//                    MsgVpnReplayLogResponse::ingress(message_vpn, replay_log,
//                                                     false, &mut core, &client);
//                }
//
//                if shutdown_egress {
//                    MsgVpnReplayLogResponse::egress(message_vpn, replay_log,
//                                                    false, &mut core, &client);
//                }



                // if file is passed, it means either provision or update.
                if matches.is_present("file") {
                    let file_name = matches.value_of("file").unwrap();
////                    if update_item {
////                        DmrClusterResponse::update(message_vpn, file_name, "",
////                                                        &mut core, &client);
////                    } else {
                    DmrClusterLinkRemoteAddressResponse::provision_with_file("", "", file_name,
                                                            &mut core, &client);
////                    }
                }


                // late un-shutdown anything
//                if no_shutdown_item {
//                    no_shutdown_egress = true;
//                    no_shutdown_ingress = true;
//                }
//
//                if no_shutdown_ingress {
//                    MsgVpnReplayLogResponse::ingress(message_vpn, replay_log,
//                                                     true, &mut core, &client);
//                }
//
//                if no_shutdown_egress {
//                    MsgVpnReplayLogResponse::egress(message_vpn, replay_log,
//                                                    true, &mut core, &client);
//                }


                // finally if fetch is specified, we do this last."
                while fetch {
                    let data = DmrClusterLinkRemoteAddressesResponse::fetch(cluster_name,
                                                                            remote_node_name, "dmrClusterName",cluster_name, count, &*cursor.to_string(), select,
                                                              &mut core, &client);


                    match data {
                        Ok(item) => {

                            if write_fetch_files {
                                DmrClusterLinkRemoteAddressesResponse::save(output_dir, &item);
                            }

                            let cq = item.meta().paging();
                            match cq {
                                Some(paging) => {
                                    info!("cq: {:?}", paging.cursor_query());
                                    cursor = Cow::Owned(paging.cursor_query().clone());
                                },
                                _ => {
                                    break
                                }
                            }
                        },
                        Err(e) => {
                            error!("error: {}", e)
                        }
                    }


                }

//                if delete {
//                    info!("deleting dmr-bridge");
//                    MsgVpnReplayLogResponse::delete(message_vpn, replay_log, "", &mut core, &client);
//                }
            } else {
                error!("No operation was specified, see --help")
            }

        }

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
