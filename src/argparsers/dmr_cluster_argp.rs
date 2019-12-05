use solace_semp_client::models::{MsgVpn, MsgVpnResponse, MsgVpnsResponse, MsgVpnAclProfile, MsgVpnAclProfileResponse, MsgVpnAclProfilesResponse, MsgVpnAclProfilePublishException, MsgVpnAclProfilePublishExceptionResponse, MsgVpnAclProfilePublishExceptionsResponse, MsgVpnBridge, MsgVpnBridgeResponse, MsgVpnBridgesResponse, MsgVpnReplayLog, MsgVpnReplayLogResponse, MsgVpnReplayLogsResponse, DmrCluster, DmrClusterLinkResponse, DmrClusterLinksResponse, DmrClusterLink, DmrClusterResponse, DmrClusterLinkRemoteAddressResponse, DmrClusterLinkRemoteAddressesResponse, DmrClusterLinkRemoteAddress, DmrClustersResponse};
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

impl CommandLineParser<DmrCluster> for DmrCluster {

    fn parse(matches: &ArgMatches, core: &mut Core, client: &APIClient<HttpsConnector<HttpConnector>>) {

        // cursor holder
        let (mut cursor, count, output_dir, select, write_fetch_files) = core_matches_args!(matches);

        // source subcommand args into matches
        if let Some(matches) = matches.subcommand_matches("dmr-cluster") {

            // get the boolean args
            let update_item = matches.is_present("update");
            let shutdown_item = matches.is_present("shutdown");
            let no_shutdown_item = matches.is_present("no-shutdown");
            let fetch = matches.is_present("fetch");
            let delete = matches.is_present("delete");

            if update_item || shutdown_item || no_shutdown_item || fetch || delete || matches.is_present("file") {

                // if file is passed, it means either provision or update.
                if matches.is_present("file") && !delete {
                    let file_name = matches.value_of("file").unwrap();
                    if update_item {
                        DmrClusterResponse::update(
                            "",
                            "",
                            file_name,
                            core,
                            &client
                        );
                    } else {
                        DmrClusterResponse::provision_with_file(
                            "",
                            "",
                            file_name,
                            core,
                            &client
                        );
                    }
                }


                // finally if fetch is specified, we do this last."
                while fetch {
                    let data = DmrClustersResponse::fetch("",
                                                          "",
                                                          "dmrClusterName",
                                                          matches.value_of("cluster-name").unwrap(),
                                                          count,
                                                          &*cursor.to_string(),
                                                          select,
                                                          core,
                                                          &client
                    );

                    cursor = maybe_save_and_return_cursor!(DmrClustersResponse, data, write_fetch_files, output_dir);

                }

                if delete {
                    info!("deleting dmr-bridge");
                    DmrClusterResponse::delete(matches.value_of("cluster-name").unwrap(), "", "", core, &client);
                }
            } else {
                error!("No operation was specified, see --help")
            }

        }
    }
}