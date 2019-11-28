use solace_semp_client::models::{MsgVpn, MsgVpnResponse, MsgVpnsResponse, MsgVpnAclProfile, MsgVpnAclProfileResponse, MsgVpnAclProfilesResponse, MsgVpnAclProfilePublishException, MsgVpnAclProfilePublishExceptionResponse, MsgVpnAclProfilePublishExceptionsResponse, MsgVpnAclProfileSubscribeException, MsgVpnAclProfileSubscribeExceptionResponse, MsgVpnAclProfileSubscribeExceptionsResponse};
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

impl CommandLineParser<MsgVpnAclProfileSubscribeException> for MsgVpnAclProfileSubscribeException {
    fn parse(matches: &ArgMatches, core: &mut Core, client: &APIClient<HttpsConnector<HttpConnector>>) {

        // cursor holder
        let (mut cursor, count, output_dir, select, write_fetch_files) = core_matches_args!(matches);

        // source subcommand args into matches
        if let Some(matches) = matches.subcommand_matches("acl-profile-subscribe-exception") {

            // get all args within the subcommand
            let update_item = matches.is_present("update");
            let fetch = matches.is_present("fetch");
            let delete = matches.is_present("delete");

            if update_item || fetch || delete || matches.is_present("file") {

                // if file is passed, it means either provision or update.
                if matches.is_present("file") {
                    let file_name = matches.value_of("file").unwrap();

                    MsgVpnAclProfileSubscribeExceptionResponse::provision_with_file(
                        matches.value_of("message-vpn").unwrap(),
                        "",
                        file_name,
                        core,
                        &client);
                }

                // finally if fetch is specified
                while fetch {
                    info!("fetching acl-profile-subscribe-exception");

                    let data = MsgVpnAclProfileSubscribeExceptionsResponse::fetch(
                        matches.value_of("message-vpn").unwrap(),
                        matches.value_of("acl-profile").unwrap(),
                        "aclProfileName",
                        matches.value_of("acl-profile").unwrap(),
                        count,
                        &*cursor.to_string(),
                        select,
                        core,
                        &client);

                    cursor = maybe_save_and_return_cursor!(MsgVpnAclProfileSubscribeExceptionsResponse, data, write_fetch_files, output_dir);

                }

                if delete {
                    info!("deleting acl subscribe exception");

                    MsgVpnAclProfileSubscribeExceptionResponse::delete_by_sub_item(
                        matches.value_of("message-vpn").unwrap(),
                        matches.value_of("acl-profile").unwrap(),
                        matches.value_of("topic-syntax").unwrap(),
                        matches.value_of("topic").unwrap(),
                        core,
                        &client);
                }
            } else {
                error!("No operation was specified, see --help")
            }

        }

    }
}