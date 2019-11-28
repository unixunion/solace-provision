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
        let (mut cursor, count, output_dir, select, write_fetch_files) = core_matches_args!(matches);

        // source subcommand args into matches
        if let Some(matches) = matches.subcommand_matches("client-username") {

            // get all args within the subcommand
            let update_item = matches.is_present("update");
            let shutdown_item = matches.is_present("shutdown");
            let no_shutdown_item = matches.is_present("no-shutdown");
            let fetch = matches.is_present("fetch");
            let delete = matches.is_present("delete");

            if update_item || shutdown_item || no_shutdown_item || fetch || delete || matches.is_present("file") {

                // early shutdown if not provisioning new
                if shutdown_item && update_item && matches.is_present("client-username") && matches.is_present("message-vpn") {
                    MsgVpnClientUsernameResponse::enabled(
                        matches.value_of("message-vpn").unwrap(),
                        matches.value_of("client-username").unwrap(),
                        vec![],
                        false,
                        core,
                        &client);
                }

                // if file is passed, it means either provision or update.
                if matches.is_present("file") {
                    let file_name = matches.value_of("file");
                    if update_item {
                        MsgVpnClientUsernameResponse::update(
                            matches.value_of("message-vpn").unwrap(),
                            file_name.unwrap(),
                            "",
                            core,
                            &client);
                    } else {
                        MsgVpnClientUsernameResponse::provision_with_file(
                            matches.value_of("message-vpn").unwrap(),
                            "",
                            file_name.unwrap(),
                            core,
                            &client);
                    }
                }

                // late un-shutdown anything
                if no_shutdown_item {
                    MsgVpnClientUsernameResponse::enabled(
                        matches.value_of("message-vpn").unwrap(),
                        matches.value_of("client-username").unwrap(),
                        vec![],
                        true,
                        core,
                        &client);
                }

                // finally if fetch is specified, we do this last.
                while fetch {
                    let data = MsgVpnClientUsernamesResponse::fetch(
                        matches.value_of("message-vpn").unwrap(),
                        matches.value_of("client-username").unwrap(),
                        "clientUsername", matches.value_of("client-username").unwrap(),
                        count,
                        &*cursor.to_string(),
                        select,
                        core,
                        &client);

                    cursor = maybe_save_and_return_cursor!(MsgVpnClientUsernamesResponse, data, write_fetch_files, output_dir);

                }

                if delete {
                    info!("deleting client-username");
                    MsgVpnClientUsernameResponse::delete(
                        matches.value_of("message-vpn").unwrap(),
                        matches.value_of("client-username").unwrap(),
                        "",
                        core,
                        &client);
                }
            } else {
                error!("No operation was specified, see --help")
            }

        }
    }
}