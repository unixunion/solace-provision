
use solace_semp_client::models::{MsgVpn, MsgVpnResponse, MsgVpnsResponse, MsgVpnAclProfile, MsgVpnAclProfileResponse, MsgVpnAclProfilesResponse, MsgVpnAclProfilePublishException, MsgVpnAclProfilePublishExceptionResponse, MsgVpnAclProfilePublishExceptionsResponse, MsgVpnClientProfileResponse, MsgVpnClientProfilesResponse, MsgVpnClientProfile, MsgVpnClientUsername, MsgVpnClientUsernameResponse, MsgVpnClientUsernamesResponse, MsgVpnQueueSubscriptionResponse, MsgVpnQueueSubscriptionsResponse, MsgVpnQueueSubscription, MsgVpnSequencedTopicResponse, MsgVpnSequencedTopicsResponse, MsgVpnSequencedTopic};
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

impl CommandLineParser<MsgVpnSequencedTopic> for MsgVpnSequencedTopic {
    fn parse(matches: &ArgMatches, core: &mut Core, client: &APIClient<HttpsConnector<HttpConnector>>) {

        let (mut cursor, count, output_dir, select, write_fetch_files) = core_matches_args!(matches);

        // source subcommand args into matches
        if let Some(matches) = matches.subcommand_matches("sequenced-topic") {

            // get all args within the subcommand
            let delete = matches.is_present("delete");
            let fetch = matches.is_present("fetch");
            let mut sequenced_topic = "";

            if matches.is_present("sequenced-topic") {
                sequenced_topic = matches.value_of("sequenced-topic").unwrap_or("*");
                info!("sequenced-topic is: {}", sequenced_topic);
            }

            if fetch || delete || matches.is_present("file") {

                // if file is passed, it means either provision or update.
                if matches.is_present("file") {
                    let file_name = matches.value_of("file");
                    MsgVpnSequencedTopicResponse::provision_with_file(
                        matches.value_of("message-vpn").unwrap(),
                        "",
                        file_name.unwrap(),
                        core,
                        &client);
                }

                // finally if fetch is specified, we do this last.
                while fetch {
                    let data = MsgVpnSequencedTopicsResponse::fetch(
                        matches.value_of("message-vpn").unwrap(),
                        "",
                        "sequencedTopic",
                        matches.value_of("sequenced-topic").unwrap(),
                        count,
                        &*cursor.to_string(),
                        select,
                        core,
                        &client);

                    cursor = maybe_save_and_return_cursor!(MsgVpnSequencedTopicsResponse, data, write_fetch_files, output_dir);

                }

                if delete {
                    info!("deleting sequence-topic");
                    MsgVpnSequencedTopicResponse::delete(
                        matches.value_of("message-vpn").unwrap(),
                        matches.value_of("sequenced-topic").unwrap(),
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