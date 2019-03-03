
use solace_semp_client::apis::client::APIClient;
use solace_semp_client::apis::configuration::Configuration;
use hyper::Client;
use tokio_core::reactor::Core;
use std::prelude::v1::Vec;
use colored::*;
use futures::{Future};
use clap::{Arg, App, load_yaml};
use serde_yaml;
use log::{info, warn, error};
use std::process::exit;
use solace_semp_client::models::MsgVpn;
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

mod provision;
mod clientconfig;
mod helpers;
mod update;
mod fetch;

extern crate log;

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


fn configprinter(parameter: &str, val: &str) {
    if val != "undefined" {
        consoleprint(format!("{}: {}", parameter.to_owned().white(), val.to_owned().green()));
    }
}


fn consoleprint(data: String) {
    info!("{}", &*data.to_string());
}


fn main() {

    // load args.yaml
    let yaml = load_yaml!("args.yaml");
    let matches = App::from_yaml(yaml).get_matches();

    // get the config file name
    let config_file_name = matches.value_of("config").unwrap_or("default.yaml");
    let count_str = matches.value_of("count").unwrap_or("10");
    let count = count_str.parse::<i32>().unwrap();

    // future impl might use this.
    let cursor = "";
    let select = "";

    let message_vpn = matches.value_of("message-vpn").unwrap_or("default");

    // default emoji for OK / Error logging
    let mut ok_emoji = "👍";
    let mut err_emoji = "❌";

    // dump current config / args
    info!("config_file: {:?}", config_file_name);
    info!("count: {:?}", &*count.to_string());

    // configure the http client
    let mut core = Core::new().unwrap();
    let handle = core.handle();
    let hyperclient = Client::configure()
        .connector(hyper_tls::HttpsConnector::new(4, &handle)
            .unwrap()
        )
        .build(&handle);
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
        },
        Err(e) => error!("error reading config: {}", e)
    }

    // the API Client from swagger spec
    let client = APIClient::new(configuration);


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

            // early shutdown if not provisioning new
            if shutdown_item && update_item && matches.is_present("message-vpn"){
                MsgVpnResponse::enabled(message_vpn, message_vpn,
                                        false, &mut core, &client);
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
                                MsgVpnResponse::update( message_vpn, file_name, &mut core,
                                                        &client);
                            } else {
                                MsgVpnResponse::provision(message_vpn,  file_name,
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
                MsgVpnResponse::enabled(message_vpn, message_vpn,
                                        true, &mut core, &client);
            }

            // finally if fetch is specified, we do this last.
            if fetch {
                MsgVpnsResponse::fetch(message_vpn, message_vpn, count, cursor, select, &mut core, &client);
            }


            if delete {
                info!("deleting message vpn");
                MsgVpnResponse::delete(message_vpn, message_vpn, &mut core, &client);
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
            let fetch = matches.is_present("fetch");
            let delete = matches.is_present("delete");

            // early shutdown if not provisioning new
            if shutdown_item && update_item && matches.is_present("queue") && matches.is_present("message-vpn") {
                MsgVpnQueueResponse::enabled(message_vpn, queue,
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
                                MsgVpnQueueResponse::update( message_vpn, file_name, &mut core,
                                                        &client);
                            } else {
                                MsgVpnQueueResponse::provision(message_vpn,  file_name,
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
                MsgVpnQueueResponse::enabled(message_vpn, queue,
                                        true, &mut core, &client);
            }

            // finally if fetch is specified, we do this last.
            if fetch {
                MsgVpnQueuesResponse::fetch(message_vpn, queue, count, cursor,
                                            select, &mut core, &client);
            }

            if delete {
                info!("deleting queue");
                MsgVpnQueueResponse::delete(message_vpn, queue, &mut core, &client);
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
                                MsgVpnAclProfileResponse::update( message_vpn, file_name,
                                                                  &mut core, &client);
                            } else {
                                MsgVpnAclProfileResponse::provision(message_vpn,  file_name,
                                                               &mut core, &client);
                            }
                        },
                        _ => unimplemented!()
                    }
                },
                None => {}
            }

            // finally if fetch is specified
            if fetch {
                info!("fetching acl");
                MsgVpnAclProfilesResponse::fetch(message_vpn, acl, count, cursor,
                                            select, &mut core, &client);
            }

            if delete {
                info!("deleting acl");
                MsgVpnAclProfileResponse::delete(message_vpn, acl, &mut core, &client);
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
                                MsgVpnClientProfileResponse::update( message_vpn, file_name, &mut core,
                                                                  &client);
                            } else {
                                MsgVpnClientProfileResponse::provision(message_vpn,  file_name,
                                                                    &mut core, &client);
                            }
                        },
                        _ => unimplemented!()
                    }
                },
                None => {}
            }

            // finally if fetch is specified
            if fetch {
                info!("fetching client-profile");
                MsgVpnClientProfilesResponse::fetch(message_vpn, client_profile, count, cursor,
                                                 select, &mut core, &client);
            }

            if delete {
                info!("deleting client-profile");
                MsgVpnClientProfileResponse::delete(message_vpn, client_profile, &mut core, &client);
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

            // early shutdown if not provisioning new
            if shutdown_item && update_item && matches.is_present("client-username") && matches.is_present("message-vpn") {
                MsgVpnClientUsernameResponse::enabled(message_vpn, client_username,
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
                                MsgVpnClientUsernameResponse::update( message_vpn, file_name, &mut core,
                                                             &client);
                            } else {
                                MsgVpnClientUsernameResponse::provision(message_vpn,  file_name,
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
                MsgVpnClientUsernameResponse::enabled(message_vpn, client_username,
                                             true, &mut core, &client);
            }

            // finally if fetch is specified, we do this last.
            if fetch {
                MsgVpnClientUsernamesResponse::fetch(message_vpn, client_username, count, cursor,
                                            select, &mut core, &client);
            }

            if delete {
                info!("deleting client-username");
                MsgVpnClientUsernameResponse::delete(message_vpn, client_username, &mut core, &client);
            }

        }

    }

    match matches.subcommand_name() {
        None => {
            println!("Please specify a subcommand, --help for more info");
            exit(1)
        },
        _  => {}
    }

}
