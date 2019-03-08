#[macro_use]
extern crate log;
extern crate env_logger;

use solace_semp_client::apis::client::APIClient;
use solace_semp_client::apis::configuration::Configuration;
use hyper::Client;
use tokio_core::reactor::Core;
use std::prelude::v1::Vec;
use colored::*;
use futures::{Future};
use clap::{Arg, App, load_yaml};
use serde_yaml;
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

mod provision;
mod clientconfig;
mod helpers;
mod update;
mod fetch;
mod save;


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




fn maybe_write_to_file<T: Serialize>(output_dir: &str, vpn_name: &str, item_name: &str, data: T) -> Result<(), Box<dyn std::error::Error + 'static>> where T: Serialize {

    if !Path::new(&format!("{}/{}", output_dir, vpn_name)).exists() {
        match fs::create_dir_all(format!("{}/{}", output_dir, vpn_name)) {
            Ok(gt) => info!("Created dir"),
            Err(e) => error!("error")
        }

    }

    let data = serde_yaml::to_string(&data);
    match data {
        Ok(_data) => {
            info!("saving");
            let mut f = File::create(format!("{}/{}/{}.yaml", output_dir, vpn_name, item_name));
            match f {
                Ok(mut _f) => {
                    _f.write(_data.as_ref());
                },
                Err(e) => error!("{}", format!("error saving {}/{}/{}.yaml, error={}", output_dir, vpn_name, item_name, e))
            }
            info!("saved: {}", format!("{}/{}/{}.yaml", output_dir, vpn_name, item_name));
        },
        Err(e) => error!("error: {}", e)
    }

    Ok(())
}


fn main() {

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
    let cursor = "";
    let select = "*";

    let message_vpn = matches.value_of("message-vpn").unwrap_or("default");

    // default emoji for OK / Error logging
    let mut ok_emoji = "👍";
    let mut err_emoji = "❌";

    // dump current config / args


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
    //  PRE CHECKS
    //

//    let request = client.
//        .and_then(|info| {
//            futures::future::ok(info)
//        });
//    match core.run(request) {
//        Ok(response) => {
//            println!("{}", response.data().un)
//        },
//        Err(e) => {
//            println!("error getting system information: {:?}", e);
//            exit(126);
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
                let data = MsgVpnsResponse::fetch(message_vpn, message_vpn, count, cursor, select, &mut core, &client);
                if write_fetch_files {
                    info!("saving: {}", message_vpn);
                    match data {
                        Ok(item) => {
                            MsgVpnsResponse::save(output_dir, &item);
                        },
                        Err(e) => {
                            error!("error: {}", e)
                        }
                    }
                }
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
                let data = MsgVpnQueuesResponse::fetch(message_vpn, queue, count, cursor,
                                            select, &mut core, &client);
                if write_fetch_files {
                    info!("saving: {}", message_vpn);
                    match data {
                        Ok(item) => {
                            MsgVpnQueuesResponse::save(output_dir, &item);
                        },
                        Err(e) => {
                            error!("error: {}", e)
                        }
                    }
                }
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
                let data = MsgVpnAclProfilesResponse::fetch(message_vpn, acl, count, cursor,
                                            select, &mut core, &client);
                if write_fetch_files {
                    info!("saving: {}", message_vpn);
                    match data {
                        Ok(item) => {
//                            maybe_write_to_file(output_dir,message_vpn, acl, item);
                            MsgVpnAclProfilesResponse::save(output_dir,&item);
                        },
                        Err(e) => {
                            error!("error: {}", e)
                        }
                    }
                }
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
                let data = MsgVpnClientProfilesResponse::fetch(message_vpn, client_profile, count, cursor,
                                                 select, &mut core, &client);
                if write_fetch_files {
                    info!("saving: {}", message_vpn);
                    match data {
                        Ok(item) => {
//                            maybe_write_to_file(output_dir,message_vpn, client_profile, item);
                            MsgVpnClientProfilesResponse::save(output_dir,&item);
                        },
                        Err(e) => {
                            error!("error: {}", e)
                        }
                    }
                }
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
                let data = MsgVpnClientUsernamesResponse::fetch(message_vpn, client_username, count, cursor,
                                            select, &mut core, &client);
                if write_fetch_files {
                    info!("saving: {}", message_vpn);
                    match data {
                        Ok(item) => {
//                            maybe_write_to_file(output_dir,message_vpn, client_username, item);
                            MsgVpnClientUsernamesResponse::save(output_dir,&item);
                        },
                        Err(e) => {
                            error!("error: {}", e)
                        }
                    }
                }
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
