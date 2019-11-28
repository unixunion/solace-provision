use solace_semp_client::models::{MsgVpn, MsgVpnResponse, MsgVpnsResponse, MsgVpnAclProfile, MsgVpnAclProfileResponse, MsgVpnAclProfilesResponse, MsgVpnAclProfilePublishException, MsgVpnAclProfilePublishExceptionResponse, MsgVpnAclProfilePublishExceptionsResponse, MsgVpnClientProfileResponse, MsgVpnClientProfilesResponse, MsgVpnClientProfile, MsgVpnClientUsername, MsgVpnClientUsernameResponse, MsgVpnClientUsernamesResponse, MsgVpnQueueSubscriptionResponse, MsgVpnQueueSubscriptionsResponse, MsgVpnQueueSubscription};
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

impl CommandLineParser<MsgVpnQueueSubscription> for MsgVpnQueueSubscription {
    fn parse(matches: &ArgMatches, core: &mut Core, client: &APIClient<HttpsConnector<HttpConnector>>) {

        let (mut cursor, count, output_dir, select, write_fetch_files) = core_matches_args!(matches);

        // source subcommand args into matches
        if let Some(matches) = matches.subcommand_matches("queue-subscription") {

            // get all args within the subcommand
            let delete = matches.is_present("delete");
            let fetch = matches.is_present("fetch");
            let mut subscription = "";

            if matches.is_present("subscription") {
                subscription = matches.value_of("subscription").expect("error interpreting subscription");
                info!("subscription is: {}", subscription);
            }

            if fetch || delete || matches.is_present("file") {

                // if file is passed, it means either provision or update.
                if matches.is_present("file") {
                    let file_name = matches.value_of("file");
                    MsgVpnQueueSubscriptionResponse::provision_with_file(matches.value_of("message-vpn").unwrap(), "", file_name.unwrap(),
                                                                         core, &client);
                }

                // finally if fetch is specified, we do this last.
                while fetch {
                    let data = MsgVpnQueueSubscriptionsResponse::fetch(matches.value_of("message-vpn").unwrap(), matches.value_of("queue").unwrap(), "queueName", matches.value_of("queue").unwrap(), count, &*cursor.to_string(),
                                                                       select, core, &client);

                    cursor = maybe_save_and_return_cursor!(MsgVpnQueueSubscriptionsResponse, data, write_fetch_files, output_dir);

                }

                if delete {
                    info!("deleting queue-subscription");
                    MsgVpnQueueSubscriptionResponse::delete(matches.value_of("message-vpn").unwrap(), matches.value_of("queue").unwrap(), subscription, core, &client);
                }
            } else {
                error!("No operation was specified, see --help")
            }

        }

    }
}
