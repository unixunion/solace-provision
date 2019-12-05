use solace_semp_client::models::{MsgVpn, MsgVpnResponse, MsgVpnsResponse, MsgVpnAclProfile, MsgVpnAclProfileResponse, MsgVpnAclProfilesResponse, MsgVpnAclProfilePublishException, MsgVpnAclProfilePublishExceptionResponse, MsgVpnAclProfilePublishExceptionsResponse, MsgVpnBridge, MsgVpnBridgeResponse, MsgVpnBridgesResponse, MsgVpnReplayLog, MsgVpnReplayLogResponse, MsgVpnReplayLogsResponse};
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

impl CommandLineParser<MsgVpnReplayLog> for MsgVpnReplayLog {
    fn parse(matches: &ArgMatches, core: &mut Core, client: &APIClient<HttpsConnector<HttpConnector>>) {
        // cursor holder
        let (mut cursor, count, output_dir, select, write_fetch_files) = core_matches_args!(matches);

        // source subcommand args into matches
        if let Some(matches) = matches.subcommand_matches("replay-log") {

            // get all args within the subcommand
//            let message_vpn = matches.value_of("message-vpn").unwrap();
//            let replay_log = matches.value_of("replay-log").unwrap();
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
                    MsgVpnReplayLogResponse::ingress(
                        matches.value_of("message-vpn").unwrap(),
                        matches.value_of("replay-log").unwrap(),
                        false,
                        core,
                        &client);
                }

                if shutdown_egress {
                    MsgVpnReplayLogResponse::egress(
                        matches.value_of("message-vpn").unwrap(),
                        matches.value_of("replay-log").unwrap(),
                        false,
                        core,
                        &client);
                }



                // if file is passed, it means either provision or update.
                if matches.is_present("file") {
                    let file_name = matches.value_of("file").unwrap();
                    if update_item {
                        MsgVpnReplayLogResponse::update(
                            "",
                            "",
                            file_name,
                            core,
                            &client);
                    } else {
                        MsgVpnReplayLogResponse::provision_with_file(
                            "",
                            "",
                            file_name,
                            core,
                            &client);
                    }
                }


                // late un-shutdown anything
                if no_shutdown_item {
                    no_shutdown_egress = true;
                    no_shutdown_ingress = true;
                }

                if no_shutdown_ingress {
                    MsgVpnReplayLogResponse::ingress(
                        matches.value_of("message-vpn").unwrap(),
                        matches.value_of("replay-log").unwrap(),
                        true, core, &client);
                }

                if no_shutdown_egress {
                    MsgVpnReplayLogResponse::egress(
                        matches.value_of("message-vpn").unwrap(),
                        matches.value_of("replay-log").unwrap(),
                        true, core, &client);
                }


                // finally if fetch is specified, we do this last.
                while fetch {
                    let data = MsgVpnReplayLogsResponse::fetch(
                        matches.value_of("message-vpn").unwrap(),
                        matches.value_of("replay-log").unwrap(),
                        "","",
                        count,
                        &*cursor.to_string(),
                        select,
                        core, &client);


                    cursor = maybe_save_and_return_cursor!(MsgVpnReplayLogsResponse, data, write_fetch_files, output_dir);


                }

                if delete {
                    info!("deleting topic endpoint");
                    MsgVpnReplayLogResponse::delete(
                        matches.value_of("message-vpn").unwrap(),
                        matches.value_of("replay-log").unwrap(),
                        "",
                        core, &client);
                }
            } else {
                error!("No operation was specified, see --help")
            }

        }

    }
}