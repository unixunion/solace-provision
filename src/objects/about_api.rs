use crate::fetch::Fetch;
use solace_semp_client::models::AboutApiResponse;
use tokio_core::reactor::Core;
use solace_semp_client::apis::client::APIClient;
use hyper_tls::HttpsConnector;
use hyper::client::HttpConnector;
use futures::future::Future;


impl Fetch<AboutApiResponse> for AboutApiResponse {

    /// fetch information about the broker
    ///
    /// # Arguments
    /// `core` - Core
    /// `apiclient` - APIClient
    ///
    fn fetch(unused_1: &str, unused_2: &str, select_key: &str, select_value: &str, count: i32, cursor: &str, selector: &str, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<AboutApiResponse, &'static str> {
        let request = apiclient
            .about_api()
            .get_about_api()
            .and_then(|about| {
                debug!("{:?}", about);
                futures::future::ok(about)
            });

        core_run!(request, core)

    }
}
