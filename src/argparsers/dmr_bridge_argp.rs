use solace_semp_client::models::{MsgVpn, MsgVpnResponse, MsgVpnsResponse, MsgVpnAclProfile, MsgVpnAclProfileResponse, MsgVpnAclProfilesResponse, MsgVpnAclProfilePublishException, MsgVpnAclProfilePublishExceptionResponse, MsgVpnAclProfilePublishExceptionsResponse, MsgVpnBridge, MsgVpnBridgeResponse, MsgVpnBridgesResponse, MsgVpnReplayLog, MsgVpnReplayLogResponse, MsgVpnReplayLogsResponse, MsgVpnDmrBridge, MsgVpnDmrBridgeResponse, MsgVpnDmrBridgesResponse};
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

impl CommandLineParser<MsgVpnDmrBridge> for MsgVpnDmrBridge {
    fn parse(matches: &ArgMatches, core: &mut Core, client: &APIClient<HttpsConnector<HttpConnector>>) {

        // cursor holder
        let (mut cursor, count, output_dir, select, write_fetch_files) = core_matches_args!(matches);

        // source subcommand args into matches
        if let Some(matches) = matches.subcommand_matches("dmr-bridge") {

            let fetch = matches.is_present("fetch");
            let delete = matches.is_present("delete");

            if fetch || delete || matches.is_present("file") {

                // if file is passed, it means either provision or update.
                if matches.is_present("file") {
                    let file_name = matches.value_of("file").unwrap();
                    MsgVpnDmrBridgeResponse::provision_with_file(
                        "",
                        "",
                        file_name,
                        core,
                        &client);
                }


                // finally if fetch is specified, we do this last.
                while fetch {
                    let data = MsgVpnDmrBridgesResponse::fetch(
                        matches.value_of("message-vpn").unwrap(),
                        "",
                        "remoteMsgVpnName",
                        matches.value_of("remote-vpn").unwrap(),
                        count,
                        &*cursor.to_string(),
                        select,
                        core,
                        &client);


                    cursor = maybe_save_and_return_cursor!(MsgVpnDmrBridgesResponse, data, write_fetch_files, output_dir);


                }

                if delete {
                    info!("deleting dmr-bridge");
                    MsgVpnDmrBridgeResponse::delete(
                        matches.value_of("message-vpn").unwrap(),
                        matches.value_of("remote-node-name").unwrap(),
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