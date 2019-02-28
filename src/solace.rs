
mod tests {

    use solace_semp_client::models::MsgVpn;
    use crate::solace::solace::Provision;
    use solace_semp_client::models::MsgVpnQueue;

    #[test]
    fn it_works() {
        // create a new vpn, then test if our new traits and functions are bound
        let mvpn = MsgVpn::new();
        assert_eq!("msgvpn ok", mvpn.ping());
        assert_eq!("base", mvpn.base());
        assert_eq!("msgvpn provision", mvpn.pingprov());

    }
}


mod solace {
    use solace_semp_client::apis::client::APIClient;
    use solace_semp_client::models::MsgVpn;
    use tokio_core::reactor::Core;
    use hyper_tls::HttpsConnector;
    use hyper::client::HttpConnector;
    use solace_semp_client::models::MsgVpnBridge;
    use solace_semp_client::models::MsgVpnQueue;
    use solace_semp_client::models::MsgVpnAclProfile;
    use serde::Serialize;
    use colored::Colorize;
    use futures::future::Future;
    use futures::future::AndThen;
    use std::fs::File;
    use serde::Deserialize;
    use std::io::Error;


//    fn make_request(request: T, mut core: &Core, mut apiclient: &APIClient<HttpsConnector<HttpConnector>>) {
//        match core.run(resp) {
//            Ok(response) => {println!("ok")},
//            Err(e) => {println!("error: {:?}", e)}
//        }
//    }

    // shared base trait for all solace provisionable objects
    pub trait Provision<T> {
        fn provision(&self, file_name: &str, core: Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>);
        fn pingprov(&self) -> String;
//        fn fetch(&self, name: &str, mut core: Core, mut apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> String;
        fn ping(&self) -> String;
        fn base(&self) -> String {
            println!("base");
            String::from("base")
        }
        fn readfile(&self, file_name: &str) -> File {
            let file = std::fs::File::open(file_name).unwrap();
            file
        }
//        fn run(&self, mut core: Core, request: Future<Item, Error>) {
//            match core.run(request) {
//                Ok(response) => {println!("{}", "success".green())},
//                Err(e) => {println!("error: {:?}", e)}
//            }
//        }
    }

    // Since MsgVpn and the likes of have the Serializable trait, we can use that to
    // extend the MsgVpn resource with our own methods. fmstic.
    // example: let mvpn = MsgVpn::new().provision(...);
    impl<MsgVpn: Serialize> Provision<MsgVpn> for MsgVpn {

        fn provision(&self, file_name: &str, mut core: Core, apiclient: &APIClient<HttpsConnector<HttpConnector>>) {
            let file = &self.readfile(file_name);
            let vpn = serde_yaml::from_reader(file).unwrap();
            let resp = apiclient
                .default_api()
                .create_msg_vpn(vpn, Vec::new())
                .and_then(|vpn| {
                    futures::future::ok(())
                });
//            self.run(core, resp);
            match core.run(resp) {
                Ok(response) => {println!("{}", "success".green())},
                Err(e) => {println!("error: {:?}", e)}
            }
        }

        fn pingprov(&self) -> String {
            "msgvpn provision".to_string()
        }

//        fn fetch(&self, name: &str, mut core: &Core, mut apiclient: &APIClient<HttpsConnector<HttpConnector>>) -> String {

//            let mut wherevec: Vec<String> = Vec::new();
//            let whereitem = format!("msgVpnName=={}", fetch_vpn);
//            wherevec.push(String::from(whereitem));
//
//            // SEMP selector
//            let mut selectvec: Vec<String> = Vec::new();
//            selectvec.push(String::from(""));
//
//            let resp = apiclient
//                .msg_vpn_api()
//                .get_msg_vpns(count, cursor, wherevec, selectvec)
//                .and_then(|vpn| {
//                    let ser = serde_yaml::to_string(&vpn.data());
//                    futures::future::ok(());
//                    ser
//                });
//            make_request(resp, core, apiclient);
//            "ok".to_string()
//        }

        fn ping(&self) -> String {
            "msgvpn ok".to_string()
        }
    }

}