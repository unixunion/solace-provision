use solace_semp_client::models::{MsgVpn, MsgVpnResponse, MsgVpnsResponse, MsgVpnAclProfile, MsgVpnAclProfileResponse, MsgVpnAclProfilesResponse, MsgVpnAclProfilePublishException, MsgVpnAclProfilePublishExceptionResponse, MsgVpnAclProfilePublishExceptionsResponse, MsgVpnBridge, MsgVpnBridgeResponse, MsgVpnBridgesResponse, MsgVpnReplayLog, MsgVpnReplayLogResponse, MsgVpnReplayLogsResponse, DmrCluster, DmrClusterLinkResponse, DmrClusterLinksResponse, DmrClusterLink, DmrClusterResponse, DmrClusterLinkRemoteAddressResponse, DmrClusterLinkRemoteAddressesResponse, DmrClusterLinkRemoteAddress};
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

impl CommandLineParser<DmrClusterLinkRemoteAddress> for DmrClusterLinkRemoteAddress {
    fn parse(matches: &ArgMatches, core: &mut Core, client: &APIClient<HttpsConnector<HttpConnector>>) {

        // cursor holder
        let (mut cursor, count, output_dir, select, write_fetch_files) = core_matches_args!(matches);

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
                    DmrClusterLinkRemoteAddressResponse::provision_with_file(
                        "",
                        "",
                        file_name,
                        core,
                        &client);
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
                    let data = DmrClusterLinkRemoteAddressesResponse::fetch(
                        matches.value_of("cluster-name").unwrap(),
                        matches.value_of("remote-node-name").unwrap(),
                        "dmrClusterName",
                        matches.value_of("cluster-name").unwrap(),
                        count,
                        &*cursor.to_string(),
                        select,
                        core,
                        &client);


                    cursor = maybe_save_and_return_cursor!(DmrClusterLinkRemoteAddressesResponse, data, write_fetch_files, output_dir);


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
}