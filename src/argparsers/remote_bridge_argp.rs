use crate::commandlineparser::CommandLineParser;
use solace_semp_client::models::{MsgVpn, MsgVpnResponse, MsgVpnsResponse, MsgVpnBridgeRemoteMsgVpn, MsgVpnBridgeRemoteMsgVpnResponse, MsgVpnBridgeRemoteMsgVpnsResponse};
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

impl CommandLineParser<MsgVpnBridgeRemoteMsgVpn> for MsgVpnBridgeRemoteMsgVpn {
    fn parse(matches: &ArgMatches, core: &mut Core, client: &APIClient<HttpsConnector<HttpConnector>>) {

        // cursor holder
        let (mut cursor, count, output_dir, select, write_fetch_files) = core_matches_args!(matches);

        // source subcommand args into matches
        if let Some(matches) = matches.subcommand_matches("remote-bridge") {

            // get all args within the subcommand
//            let message_vpn = matches.value_of("message-vpn").unwrap_or("undefined");
//            let bridge = matches.value_of("bridge").unwrap_or("undefined");
//            let virtual_router = matches.value_of("virtual-router").unwrap_or("undefined");
            let update_item = matches.is_present("update");
            let shutdown_item = matches.is_present("shutdown");
            let no_shutdown_item = matches.is_present("no-shutdown");
            let fetch = matches.is_present("fetch");
            let delete = matches.is_present("delete");

            if shutdown_item || no_shutdown_item || fetch || delete || matches.is_present("file") {

                if shutdown_item {
                    MsgVpnBridgeRemoteMsgVpnResponse::enabled(
                        matches.value_of("message-vpn").unwrap(),
                        matches.value_of("bridge").unwrap(),
                        vec![matches.value_of("virtual-router").unwrap()],
                        false,
                        core,
                        &client);
                }

                // if file is passed, it means either provision or update.
                if matches.is_present("file") {
                    let file_name = matches.value_of("file").unwrap();
                    if update_item {
                        MsgVpnBridgeRemoteMsgVpnResponse::update(
                            "",
                            file_name,
                            "",
                            core,
                            &client);
                    } else {
                        MsgVpnBridgeRemoteMsgVpnResponse::provision_with_file(
                            "",
                            "",
                            file_name,
                            core,
                            &client);
                    }
                }


                if no_shutdown_item {
                    MsgVpnBridgeRemoteMsgVpnResponse::enabled(
                        matches.value_of("message-vpn").unwrap(),
                        matches.value_of("bridge").unwrap(),
                        vec![matches.value_of("virtual-router").unwrap()],
                        true,
                        core,
                        &client);
                }

                // finally if fetch is sspecified, we do this last.
                while fetch {
                    let data = MsgVpnBridgeRemoteMsgVpnsResponse::fetch(
                        matches.value_of("message-vpn").unwrap(),
                        matches.value_of("bridge").unwrap(),
                        matches.value_of("virtual-router").unwrap(),
                        "",
                        count,
                        &*cursor.to_string(),
                        select,
                        core,
                        &client);

                    cursor = maybe_save_and_return_cursor!(MsgVpnBridgeRemoteMsgVpnsResponse, data, write_fetch_files, output_dir);


                }

                if delete {
                    info!("deleting authorization group");
                    MsgVpnBridgeRemoteMsgVpnResponse::delete(
                        matches.value_of("message-vpn").unwrap(),
                        matches.value_of("bridge").unwrap(),
                        matches.value_of("virtual-router").unwrap(),
                        core,
                        &client);
                }
            } else {
                error!("No operation was specified, see --help")
            }

        }
    }
}