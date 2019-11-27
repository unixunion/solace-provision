use solace_semp_client::models::{MsgVpn, MsgVpnResponse, MsgVpnsResponse, MsgVpnAclProfile, MsgVpnAclProfileResponse, MsgVpnAclProfilesResponse, MsgVpnAclProfilePublishException, MsgVpnAclProfilePublishExceptionResponse, MsgVpnAclProfilePublishExceptionsResponse};
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

impl CommandLineParser<MsgVpnAclProfilePublishException> for MsgVpnAclProfilePublishException {

    fn parse(matches: &ArgMatches, core: &mut Core, client: &APIClient<HttpsConnector<HttpConnector>>) {

        // cursor holder
        let mut cursor = Cow::Borrowed("");
        let count = matches.value_of("count").unwrap().parse::<i32>().unwrap();
        let output_dir = matches.value_of("output").unwrap();
        let select = matches.value_of("select").unwrap();
        let mut write_fetch_files = matches.value_of("save").unwrap().parse::<bool>().unwrap();

        // source subcommand args into matches
        if let Some(matches) = matches.subcommand_matches("acl-profile-publish-exception") {

            // get all args within the subcommand
            let message_vpn = matches.value_of("message-vpn").unwrap_or("undefined");
            let acl = matches.value_of("acl-profile").unwrap_or("undefined");
            let topic_syntax = matches.value_of("topic-syntax").unwrap_or("undefined");
            let topic = matches.value_of("topic").unwrap_or("undefined");
            let update_item = matches.is_present("update");
            let fetch = matches.is_present("fetch");
            let delete = matches.is_present("delete");

            if update_item || fetch || delete || matches.is_present("file") {

                // if file is passed, it means either provision or update.
                if matches.is_present("file") {
                    let file_name = matches.value_of("file").unwrap();
                    MsgVpnAclProfilePublishExceptionResponse::provision_with_file(message_vpn, "", file_name,
                                                                                  core, &client);
                }


                // finally if fetch is specified
                while fetch {
                    info!("fetching acl-profile-publish-exception");
                    let data = MsgVpnAclProfilePublishExceptionsResponse::fetch(message_vpn,
                                                                                acl, "aclProfileName",acl, count,
                                                                                &*cursor.to_string(), select, core, &client);
                    match data {
                        Ok(response) => {
                            if write_fetch_files {
                                MsgVpnAclProfilePublishExceptionsResponse::save(output_dir, &response);
                            }

                            cursor = move_cursor!(response);
                        }
                        Err(e) => {
                            error!("error: {}", e)
                        }
                    }
                }

                if delete {
                    info!("deleting acl publish exception");
                    MsgVpnAclProfilePublishExceptionResponse::delete_by_sub_item(message_vpn, acl, topic_syntax, topic , core, &client);
                }
            } else {
                error!("No operation was specified, see --help")
            }

        }
    }
}