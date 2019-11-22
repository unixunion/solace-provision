use crate::fetch::Fetch;
use solace_semp_client::models::AboutApiResponse;
use tokio_core::reactor::Core;
use solace_semp_client::apis::client::APIClient;
use hyper_tls::HttpsConnector;
use hyper::client::HttpConnector;
use futures::future::Future;

// fetch about
impl Fetch<AboutApiResponse> for AboutApiResponse {
    fn fetch(in_vpn: &str, sub_item: &str, select_key: &str, select_value: &str, count: i32, cursor: &str, selector: &str, core: &mut Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> Result<AboutApiResponse, &'static str> {
        let request = apiclient
            .about_api()
            .get_about_api()
            .and_then(|about| {
                debug!("{:?}", about);
                futures::future::ok(about)
            });

        core_run!(request, core)
//        match core.run(request) {
//            Ok(response) => {
//                let t = format!("{}", serde_yaml::to_string(&response.data().unwrap()).unwrap());
//                println!("{}", &t);
//                Ok(response)
//            },
//            Err(e) => {
//                error!("error fetching: {:?}", e);
//                panic!("fetch error: {:?}", e);
//                Err("fetch error")
//            }
//        }
    }
}
