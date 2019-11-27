use solace_semp_client::models::{MsgVpn, MsgVpnResponse, MsgVpnsResponse, MsgVpnAclProfile, MsgVpnAclProfileResponse, MsgVpnAclProfilesResponse, MsgVpnAclProfilePublishException, MsgVpnAclProfilePublishExceptionResponse, MsgVpnAclProfilePublishExceptionsResponse, MsgVpnClientProfileResponse, MsgVpnClientProfilesResponse, MsgVpnClientProfile, MsgVpnClientUsername, MsgVpnClientUsernameResponse, MsgVpnClientUsernamesResponse};
use clap::ArgMatches;
use solace_semp_client::apis::client::APIClient;
use hyper_tls::HttpsConnector;
use hyper::client::HttpConnector;
use std::borrow::Cow;
use crate::update::Update;
use tokio_core::reactor::Core;
use crate::provision::Provision;
use crate::fetch::Fetch;
use crate::save::Save;
use crate::commandlineparser::CommandLineParser;

impl CommandLineParser<MsgVpnClientUsername> for MsgVpnClientUsername {
    fn parse(matches: &ArgMatches, core: &mut Core, client: &APIClient<HttpsConnector<HttpConnector>>) {

        // cursor holder
        let mut cursor = Cow::Borrowed("");
        let count = matches.value_of("count").unwrap().parse::<i32>().unwrap();
        let output_dir = matches.value_of("output").unwrap();
        let select = matches.value_of("select").unwrap();
        let mut write_fetch_files = matches.value_of("save").unwrap().parse::<bool>().unwrap();

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
                                                          false, core, &client);
                }


                // if file is passed, it means either provision or update.
                if matches.is_present("file") {
                    let file_name = matches.value_of("file");
                    if update_item {
                        MsgVpnClientUsernameResponse::update(message_vpn, file_name.unwrap(), "",
                                                             core, &client);
                    } else {
                        MsgVpnClientUsernameResponse::provision_with_file(message_vpn, "", file_name.unwrap(),
                                                                          core, &client);
                    }
                }

                // late un-shutdown anything
                if no_shutdown_item {
                    MsgVpnClientUsernameResponse::enabled(message_vpn, client_username, vec![],
                                                          true, core, &client);
                }

                // finally if fetch is specified, we do this last.
                while fetch {
                    let data = MsgVpnClientUsernamesResponse::fetch(message_vpn, client_username,
                                                                    "clientUsername", client_username,count,
                                                                    &*cursor.to_string(), select, core, &client);

                    match data {
                        Ok(response) => {
                            if write_fetch_files {
                                MsgVpnClientUsernamesResponse::save(output_dir, &response);
                            }

                            cursor = move_cursor!(response);
                        }
                        Err(e) => {
                            error!("error: {}", e)
                        }
                    }

                }

                if delete {
                    info!("deleting client-username");
                    MsgVpnClientUsernameResponse::delete(message_vpn, client_username, "", core, &client);
                }
            } else {
                error!("No operation was specified, see --help")
            }

        }
    }
}