
use solace_semp_client::apis::client::APIClient;
use solace_semp_client::apis::configuration::Configuration;
use hyper::Client;
use tokio_core::reactor::Core;
use std::prelude::v1::Vec;
use colored::*;
use futures::{Future};
use clap::{Arg, App, load_yaml};
use serde_yaml;
use log::{info};
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
//use crate::consoleprint;

mod solace;
mod clientconfig;
mod helpers;


mod test {
    use solace_semp_client::models::MsgVpn;

    #[test]
    fn new_event_threshold_empty() {
        // create a new vpn, then test if our new traits and functions are bound
        let mvpn = MsgVpn::new();
        let x = serde_yaml::to_string(&mvpn.event_connection_count_threshold());
        match x {
            Ok(svpn) => {
                println!("{}", svpn);
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
    println!("{}", &*data.to_string());
}


fn main() {

    // load args.yaml
    let yaml = load_yaml!("args.yaml");
    let matches = App::from_yaml(yaml).get_matches();

    // get the config file name
    let config_file_name = matches.value_of("config").unwrap_or("default.yaml");
    let vpn_file = matches.value_of("vpn").unwrap_or("undefined");
    let queue_file = matches.value_of("queue").unwrap_or("undefined");
    let acl_profile_file = matches.value_of("acl").unwrap_or("undefined");
    let client_profile_file = matches.value_of("client-profile").unwrap_or("undefined");
    let client_username_file = matches.value_of("client-username").unwrap_or("undefined");

    let update_mode = matches.is_present("update");

    // fetchers
    let fetch_vpn = matches.value_of("fetch-vpn").unwrap_or("undefined");
    let fetch_queue = matches.value_of("fetch-queue").unwrap_or("undefined");
    let fetch_acl_profile = matches.value_of("fetch-acl-profile").unwrap_or("undefined");
    let fetch_client_profile = matches.value_of("fetch-client-profile").unwrap_or("undefined");
    let fetch_client_username = matches.value_of("fetch-client-username").unwrap_or("undefined");

    let shutdown = matches.is_present("shutdown");
    let count_str = matches.value_of("count").unwrap_or("10");
    let count = count_str.parse::<i32>().unwrap();

    let cursor = "";
    let select = "";

    let message_vpn = matches.value_of("message-vpn").unwrap_or("default");

    // default emoji for OK / Error logging
    let mut ok_emoji = "👍";
    let mut err_emoji = "❌";

    // dump current config / args
    configprinter("config_file", config_file_name);

    configprinter("vpn_file", vpn_file);
    configprinter("queue_file", queue_file);
    configprinter("acl_profile_file", acl_profile_file);
    configprinter("client_profile_file", client_profile_file);
    configprinter("client_username_file", client_username_file);

    configprinter("update_mode", &*update_mode.to_string());

    configprinter("fetch_vpn", fetch_vpn);
    configprinter("fetch_queue", fetch_queue);
    configprinter("fetch_acl_profile", fetch_acl_profile);
    configprinter("fetch_client_username", fetch_client_username);
    configprinter("shutdown", &*shutdown.to_string());
    configprinter("count", &*count.to_string());
    configprinter("message_vpn", message_vpn);

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
        Err(e) => println!("error reading config: {}", e)
    }

    // the API Client from swagger spec
    let client = APIClient::new(configuration);


    //
    // VPN FETCH / PROVISION / UPDATE
    //

    // Fetch VPN
    if fetch_vpn != "undefined" {

        let mut wherevec: Vec<String> = Vec::new();
        let whereitem = format!("msgVpnName=={}", fetch_vpn);
        wherevec.push(String::from(whereitem));

        // SEMP selector
        let mut selectvec: Vec<String> = Vec::new();
        selectvec.push(String::from(select));

        println!("{}: {}", "Fetch VPN".white(), fetch_vpn.green());
        let resp = client
            .msg_vpn_api()
            .get_msg_vpns(count, cursor, wherevec, selectvec)
            .and_then(|vpn| {

                match serde_yaml::to_string(&vpn.data().unwrap()) {
                    Ok(v) => {
                        println!("{}", format!("{}", v));
                    },
                    Err(_) => {
                        println!("Error deserializing");
                    }
                }

                futures::future::ok(())
            });


        println!("{}", "Making request".green());
        match core.run(resp) {
            Ok(response) => {println!("{} {}", ok_emoji, "success".green())},
            Err(e) => {println!("{} error: {:?}", err_emoji, e)}
        }
        println!("{}", "Requests made".yellow());
    }

    // VPN provision if file is passed
    if vpn_file.to_owned() != "undefined" {

        // read in the file
        let file = std::fs::File::open(vpn_file).unwrap();
        let deserialized: Option<MsgVpn> = serde_yaml::from_reader(file).unwrap();

        match deserialized {
            Some(mut item) => {

                if message_vpn != "undefined" {
                    item.set_msg_vpn_name(message_vpn.to_owned());
                }

                if update_mode {

                    if shutdown {
                        consoleprint(format!("{}", "disabling".red()));
                        item.set_enabled(false);
                    } else {
                        consoleprint(format!("{}", "enabling".green()));
                        item.set_enabled(true);
                    }

                    let vpn_name = &item.msg_vpn_name();
                    let resp = client
                        .default_api()
                        .update_msg_vpn(&vpn_name.unwrap().to_owned(),
                                        item,
                                        Vec::new())
                        .and_then(|vpn| {
                            futures::future::ok(())
                        });

                    match core.run(resp) {
                        Ok(response) => { println!("{} {}", ok_emoji, "success".green()) },
                        Err(e) => { println!("{} error: {:?}", err_emoji, e) }
                    }
                } else {
                    let resp = client
                        .default_api()
                        .create_msg_vpn(item, Vec::new())
                        .and_then(|vpn| {
                            futures::future::ok(())
                        });
                    match core.run(resp) {
                        Ok(response) => {println!("{} {}", ok_emoji, "success".green())},
                        Err(e) => {println!("{} error: {:?}", err_emoji, e)}
                    }
                }

            },
            _ => unimplemented!()
        }
    }


    //
    // QUEUE FETCH / PROVISION / UPDATE
    //

    if fetch_queue != "undefined" {

        let mut wherevec: Vec<String> = Vec::new();
        let whereitem = format!("queueName=={}", fetch_queue);
        wherevec.push(String::from(whereitem));

        // SEMP selector
        let mut selectvec: Vec<String> = Vec::new();
        selectvec.push(String::from(select));

        println!("{}: {} in vpn: {}", "Fetch QUEUE".white(), fetch_queue.green(), message_vpn.blue());
        let resp = client
            .msg_vpn_api()
            .get_msg_vpn_queues(message_vpn, count, cursor, wherevec, selectvec )
            .and_then(|item| {

                match serde_yaml::to_string(&item.data().unwrap()) {
                    Ok(v) => {
                        println!("{}", format!("{}", v));
                    },
                    Err(_) => {
                        println!("Error deserializing");
                    }
                }

                futures::future::ok(())
            });


        println!("{}", "Making request".green());
        match core.run(resp) {
            Ok(response) => {println!("👍 {}", "success".green())},
            Err(e) => {println!("Error: {:?}", e)}
        }
        println!("{}", "Requests made".yellow());
    }


    // Provision Queue from file
    if queue_file.to_owned() != "undefined" {
        // read in the file
        let file = std::fs::File::open(queue_file).unwrap();

        let deserialized: Option<MsgVpnQueue> = serde_yaml::from_reader(file).unwrap();

        match deserialized {
            Some(mut item) => {

                if message_vpn != "undefined" {
                    item.set_msg_vpn_name(message_vpn.to_owned());
                }

                if update_mode {

                    let vpn_name = &item.msg_vpn_name();
                    let queue_name = &item.queue_name();

                    let resp = client
                        .default_api()
                        .update_msg_vpn_queue(&vpn_name.unwrap().to_owned(),
                                              &queue_name.unwrap().to_owned(),
                                              item, Vec::new())
                        .and_then(|item| {
                            futures::future::ok(())
                        });
                    match core.run(resp) {
                        Ok(response) => {println!("{} {}", ok_emoji, "success".green())},
                        Err(e) => {println!("{} error: {:?}", err_emoji, e)}
                    }
                } else {
                    let vpn_name = &item.msg_vpn_name();

                    let resp = client
                        .default_api()
                        .create_msg_vpn_queue(&vpn_name.unwrap().to_owned(), item, Vec::new())
                        .and_then(|item| {
                            futures::future::ok(())
                        });
                    match core.run(resp) {
                        Ok(response) => {println!("{} {}", ok_emoji, "success".green())},
                        Err(e) => {println!("{} error: {:?}", err_emoji, e)}
                    }
                }
            },
            _ => unimplemented!()
        }

    }


    //
    // ACL PROFILE FETCH / PROVISION / UPDATE
    //

    if fetch_acl_profile != "undefined" {

        let mut wherevec: Vec<String> = Vec::new();
        let whereitem = format!("aclProfileName=={}", fetch_acl_profile);
        wherevec.push(String::from(whereitem));

        // SEMP selector
        let mut selectvec: Vec<String> = Vec::new();
        selectvec.push(String::from(select));

        println!("{}: {} in vpn: {}", "Fetch acl".white(), fetch_acl_profile.green(), message_vpn.blue());
        let resp = client
            .msg_vpn_api()
            .get_msg_vpn_acl_profiles(message_vpn, count, cursor, wherevec, selectvec)
            .and_then(|item| {

                match serde_yaml::to_string(&item.data().unwrap()) {
                    Ok(v) => {
                        println!("{}", format!("{}", v));
                    },
                    Err(_) => {
                        println!("Error deserializing");
                    }
                }

                futures::future::ok(())
            });


        println!("{}", "Making request".green());
        match core.run(resp) {
            Ok(response) => {println!("👍 {}", "success".green())},
            Err(e) => {println!("Error: {:?}", e)}
        }
        println!("{}", "Requests made".yellow());
    }


    // Provision ACL profile from file
    if acl_profile_file.to_owned() != "undefined" {
        // read in the file
        let file = std::fs::File::open(acl_profile_file).unwrap();

        let deserialized: Option<MsgVpnAclProfile> = serde_yaml::from_reader(file).unwrap();

        match deserialized {
            Some(mut item) => {

                if message_vpn != "undefined" {
                    item.set_msg_vpn_name(message_vpn.to_owned());
                }

                if update_mode {

                    let vpn_name = &item.msg_vpn_name();
                    let item_name = &item.acl_profile_name();

                    let resp = client
                        .default_api()
                        .update_msg_vpn_acl_profile(&vpn_name.unwrap().to_owned(),
                                                    &item_name.unwrap().to_owned(),
                                                    item, Vec::new())
                        .and_then(|item| {
                            futures::future::ok(())
                        });
                    match core.run(resp) {
                        Ok(response) => {println!("{} {}", ok_emoji, "success".green())},
                        Err(e) => {println!("{} error: {:?}", err_emoji, e)}
                    }
                } else {
                    let vpn_name = &item.msg_vpn_name();

                    let resp = client
                        .default_api()
                        .create_msg_vpn_acl_profile(&vpn_name.unwrap().to_owned(), item, Vec::new())
                        .and_then(|item| {
                            futures::future::ok(())
                        });
                    match core.run(resp) {
                        Ok(response) => {println!("{} {}", ok_emoji, "success".green())},
                        Err(e) => {println!("{} error: {:?}", err_emoji, e)}
                    }
                }
            },
            _ => unimplemented!()
        }

    }


    //
    // CLIENT PROFILE FETCH / PROVISION / UPDATE
    //

    if fetch_client_profile != "undefined" {

        let mut wherevec: Vec<String> = Vec::new();
        let whereitem = format!("clientProfileName=={}", fetch_client_profile);
        wherevec.push(String::from(whereitem));

        // SEMP selector
        let mut selectvec: Vec<String> = Vec::new();
        selectvec.push(String::from(select));

        println!("{}: {} in vpn: {}", "Fetch client-profile".white(), fetch_client_profile.green(), message_vpn.blue());
        let resp = client
            .msg_vpn_api()
            .get_msg_vpn_client_profiles(message_vpn, count, cursor, wherevec, selectvec)
            .and_then(|item| {

                match serde_yaml::to_string(&item.data().unwrap()) {
                    Ok(v) => {
                        println!("{}", format!("{}", v));
                    },
                    Err(_) => {
                        println!("Error deserializing");
                    }
                }

                futures::future::ok(())
            });


        println!("{}", "Making request".green());
        match core.run(resp) {
            Ok(response) => {println!("👍 {}", "success".green())},
            Err(e) => {println!("Error: {:?}", e)}
        }
        println!("{}", "Requests made".yellow());
    }


    // Provision client profile from file
    if client_profile_file.to_owned() != "undefined" {
        // read in the file
        let file = std::fs::File::open(client_profile_file).unwrap();

        let deserialized: Option<MsgVpnClientProfile> = serde_yaml::from_reader(file).unwrap();

        match deserialized {
            Some(mut item) => {

                if message_vpn != "undefined" {
                    item.set_msg_vpn_name(message_vpn.to_owned());
                }

                if update_mode {

                    let vpn_name = &item.msg_vpn_name();
                    let item_name = &item.client_profile_name();

                    let resp = client
                        .default_api()
                        .update_msg_vpn_client_profile(&vpn_name.unwrap().to_owned(),
                                                       &item_name.unwrap().to_owned(),
                                                       item, Vec::new())
                        .and_then(|item| {
                            futures::future::ok(())
                        });
                    match core.run(resp) {
                        Ok(response) => {println!("{} {}", ok_emoji, "success".green())},
                        Err(e) => {println!("{} error: {:?}", err_emoji, e)}
                    }
                } else {
                    let vpn_name = &item.msg_vpn_name();

                    let resp = client
                        .default_api()
                        .create_msg_vpn_client_profile(&vpn_name.unwrap().to_owned(), item, Vec::new())
                        .and_then(|item| {
                            futures::future::ok(())
                        });
                    match core.run(resp) {
                        Ok(response) => {println!("{} {}", ok_emoji, "success".green())},
                        Err(e) => {println!("{} error: {:?}", err_emoji, e)}
                    }
                }

            },
            _ => unimplemented!()
        }

    }


    //
    // CLIENT USERNAME FETCH / PROVISION / UPDATE
    //

    if fetch_client_username != "undefined" {

        let mut wherevec: Vec<String> = Vec::new();
        let whereitem = format!("clientUsername=={}", fetch_client_username);
        wherevec.push(String::from(whereitem));

        // SEMP selector
        let mut selectvec: Vec<String> = Vec::new();
        selectvec.push(String::from(select));

        println!("{}: {} in vpn: {}", "Fetch client-username".white(), fetch_client_profile.green(), message_vpn.blue());
        let resp = client
            .msg_vpn_api()
            .get_msg_vpn_client_usernames(message_vpn, count, cursor, wherevec, selectvec)
            .and_then(|item| {

                match serde_yaml::to_string(&item.data().unwrap()) {
                    Ok(v) => {
                        println!("{}", format!("{}", v));
                    },
                    Err(_) => {
                        println!("Error deserializing");
                    }
                }

                futures::future::ok(())
            });


        println!("{}", "Making request".green());
        match core.run(resp) {
            Ok(response) => {println!("👍 {}", "success".green())},
            Err(e) => {println!("Error: {:?}", e)}
        }
        println!("{}", "Requests made".yellow());
    }

    if client_username_file.to_owned() != "undefined" {
        // read in the file
        let file = std::fs::File::open(client_username_file).unwrap();

        let deserialized: Option<MsgVpnClientUsername> = serde_yaml::from_reader(file).unwrap();

        match deserialized {
            Some(mut item) => {
                if message_vpn != "undefined" {
                    item.set_msg_vpn_name(message_vpn.to_owned());
                }

                if update_mode {

                    if shutdown {
                        consoleprint(format!("{}", "disabling".red()));
                        item.set_enabled(false);
                    } else {
                        consoleprint(format!("{}", "enabling".green()));
                        item.set_enabled(true);
                    }

                    let vpn_name = &item.msg_vpn_name();
                    let item_name = &item.client_username();

                    let resp = client
                        .default_api()
                        .update_msg_vpn_client_username(&vpn_name.unwrap().to_owned(),
                                                        &item_name.unwrap().to_owned(),
                                                        item, Vec::new())
                        .and_then(|item| {
                            futures::future::ok(())
                        });
                    match core.run(resp) {
                        Ok(response) => {println!("{} {}", ok_emoji, "success".green())},
                        Err(e) => {println!("{} error: {:?}", err_emoji, e)}
                    }
                } else {
                    let vpn_name = &item.msg_vpn_name();

                    let resp = client
                        .default_api()
                        .create_msg_vpn_client_username(&vpn_name.unwrap().to_owned(), item, Vec::new())
                        .and_then(|item| {
                            futures::future::ok(())
                        });
                    match core.run(resp) {
                        Ok(response) => {println!("{} {}", ok_emoji, "success".green())},
                        Err(e) => {println!("{} error: {:?}", err_emoji, e)}
                    }
                }

            },
            _ => unimplemented!()
        }

    }


}
