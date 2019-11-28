use crate::commandlineparser::CommandLineParser;
use solace_semp_client::models::{MsgVpnQueue, MsgVpnQueueResponse, MsgVpnQueuesResponse};
use clap::ArgMatches;
use tokio_core::reactor::Core;
use solace_semp_client::apis::client::APIClient;
use hyper_tls::HttpsConnector;
use hyper::client::HttpConnector;
use std::borrow::Cow;
use crate::update::Update;
use crate::provision::Provision;
use crate::save::Save;
use crate::fetch::Fetch;

impl CommandLineParser<MsgVpnQueue> for MsgVpnQueue {
    fn parse(matches: &ArgMatches, core: &mut Core, client: &APIClient<HttpsConnector<HttpConnector>>) {

        // cursor holder
        let mut cursor = Cow::Borrowed("");
        let count = matches.value_of("count").unwrap().parse::<i32>().unwrap();
        let output_dir = matches.value_of("output").unwrap();
        let select = matches.value_of("select").unwrap();
        let mut write_fetch_files = matches.value_of("save").unwrap().parse::<bool>().unwrap();

        // source subcommand args into matches
        if let Some(matches) = matches.subcommand_matches("queue") {

            // get all args within the subcommand
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
                    MsgVpnQueueResponse::ingress(matches.value_of("message-vpn").unwrap(),
                                                 matches.value_of("queue").unwrap(),
                                                 false, core, &client);
                }

                if shutdown_egress {
                    MsgVpnQueueResponse::egress(matches.value_of("message-vpn").unwrap(),
                                                matches.value_of("queue").unwrap(),
                                                false, core, &client);
                }

                // if file is passed, it means either provision or update.
                if (matches.is_present("file")) {
                    let file_name = matches.value_of("file");

                    if update_item {
                        MsgVpnQueueResponse::update(matches.value_of("message-vpn").unwrap(),
                                                    matches.value_of("file").unwrap(),
                                                    "",
                                                    core,
                                                    &client
                        );
                    } else {
                        if (matches.is_present("message-vpn")) {
                            info!("message-vpn specified");
                            MsgVpnQueueResponse::provision_with_file(matches.value_of("message-vpn").unwrap(),
                                                                     "",
                                                                     matches.value_of("file").unwrap(),
                                                                     core,
                                                                     &client
                            );
                        } else {
                            info!("no message-vpn specified, using file as SoT");
                            MsgVpnQueueResponse::provision_with_file("",
                                                                     "",
                                                                     matches.value_of("file").unwrap(),
                                                                     core,
                                                                     &client
                            );
                        }
                    }

                }


                // late un-shutdown anything
                if no_shutdown_item {
                    no_shutdown_egress = true;
                    no_shutdown_ingress = true;
                }

                if no_shutdown_ingress {
                    MsgVpnQueueResponse::ingress(matches.value_of("message-vpn").unwrap(),
                                                 matches.value_of("queue").unwrap(),
                                                 true, core, &client);
                }

                if no_shutdown_egress {
                    MsgVpnQueueResponse::egress(matches.value_of("message-vpn").unwrap(),
                                                matches.value_of("queue").unwrap(),
                                                true, core, &client);
                }


                // finally if fetch is specified, we do this last.
                while fetch {

                    let data = MsgVpnQueuesResponse::fetch(
                        matches.value_of("message-vpn").unwrap(),
                        "",
                        "queueName",
                        matches.value_of("queue").unwrap(),
                        count,
                        &*cursor.to_string(),
                        select,
                        core,
                        &client
                    );

//                    cursor = maybe_save_and_return_cursor!(MsgVpnQueuesResponse, data, &matches);
//                    match data {
//                        Ok(item) => {
//                            if write_fetch_files {
//                                MsgVpnQueuesResponse::save(output_dir, &item);
//                            }
//
//                            cursor = move_cursor!(item);
//                        },
//                        Err(e) => {
//                            error!("error: {}", e)
//                        }
//                    }


                }

                if delete {
                    info!("deleting queue");
                    MsgVpnQueueResponse::delete(
                        matches.value_of("message-vpn").unwrap(),
                        matches.value_of("queue").unwrap(),
                        "",
                        core,
                        &client
                    );
                }
            } else {
                error!("No operation was specified, see --help")
            }

        }
    }
}