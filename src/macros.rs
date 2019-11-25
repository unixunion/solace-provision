#[macro_export]

// macro for performing a run on core, with a future response
macro_rules! core_run {
    ($request: expr, $core: expr) => {
        match $core.run($request) {
            Ok(response) => {
                println!("{}",format!("{}", serde_yaml::to_string(&response.data().unwrap()).unwrap()));
                Ok(response)
            },
            Err(e) => {
                error!("error in response: {:?}", e);
                panic!("response error: {:?}", e);
                Err("error")
            }
        }
    };
    ($request: expr, $core: expr, $panic: expr) => {
        match $core.run($request) {
            Ok(response) => {
                println!("{}",format!("{}", serde_yaml::to_string(&response.data().unwrap()).unwrap()));
                Ok(response)
            },
            Err(e) => {
                error!("error in response: {:?}", e);
                panic!("response error: {:?}", e);
                if ($panic) {
                    system::exit(126);
                }
                Err("error")
            }
        }
    }
}

// macro for run on core when sempMeta response without a "data" method
macro_rules! core_run_meta {
    ($request: expr, $core: expr) => {
        match $core.run($request) {
            Ok(response) => {
                Ok(response)
            },
            Err(e) => {
                error!("error fetching: {:?}", e);
                Err("fetch error")
            }
        }
    };
    ($request: expr, $core: expr, $panic: expr) => {
        match $core.run($request) {
            Ok(response) => {
                Ok(response)
            },
            Err(e) => {
                error!("error fetching: {:?}", e);
                if ($panic) {
                    exit(126);
                }
                Err("fetch error")
            }
        }
    }
}

// tweaks the enabled bit on item with a set_enabled method
macro_rules! enabled {
    ($obj: expr, $value: expr) => {
        $obj.set_enabled($value.clone());
    }
}


macro_rules! solace_connect {
    () => {{

        // configure the http client
        let mut core = Core::new().unwrap();
        let handle = core.handle();

        let mut http = HttpConnector::new(4, &handle);
        http.enforce_http(false);

        let mut tls = TlsConnector::builder().unwrap();

        let hyperclient = Client::configure()
            .connector(hyper_tls::HttpsConnector::from((http, tls.build().unwrap()))).build(&handle);

        let auth = helpers::gencred("admin".to_owned(), "admin".to_owned());

        // the configuration for the APIClient
        let mut configuration = Configuration {
            base_path: "http://localhost:8080/SEMP/v2/config".to_owned(),
            user_agent: Some("solace-provision".to_owned()),
            client: hyperclient,
            basic_auth: Some(auth),
            oauth_access_token: None,
            api_key: None,
        };

        let client = APIClient::new(configuration);
        (core, client)
    }}
}

macro_rules! deserialize_file_into_type {
    ($file: expr, $type: ty) => {{
        let file = std::fs::File::open($file).unwrap();
        let deserialized: Option<$type> = serde_yaml::from_reader(file).unwrap();
        deserialized
    }}
}


macro_rules! maybe_override_msg_vpn_name {
    ($passed_value: expr, $body_value: expr, $parameter_setter: tt) => {{
        let mut output = $item;
        let object_parameter = $parameter_getter;

        if (&output != &"") {
            info!("overriding parameter to: {}", output);
            &$parameter_setter(output.to_owned());
        } else {
            info!("using parameter from file");
            output = &*object_parameter;
        }
        output
    }}
}


//macro_rules! maybe_set_vpn_name {
//    ($item: expr, $vpn_name) = {
//        if (&vpn_name != &"") {
//            &item.set_msg_vpn_name(vpn_name.to_owned());
//        }
//    }
//}

// example
macro_rules! call_on_self {
    ($self:ident, $F:ident) => {
        $self.$F()
    }
}