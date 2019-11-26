use clap::ArgMatches;
use tokio_core::reactor::Core;
use solace_semp_client::apis::client::APIClient;
use hyper_tls::HttpsConnector;
use hyper::client::HttpConnector;

pub trait CommandLineParser<T> {
    fn parse(matches: &ArgMatches, core: &mut Core, client: &APIClient<HttpsConnector<HttpConnector>>);
}