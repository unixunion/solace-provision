#[macro_export]
macro_rules! core_run {
    ($request: expr, $core: expr) => {
        match $core.run($request) {
            Ok(response) => {
                println!("{}",format!("{}", serde_yaml::to_string(&response.data().unwrap()).unwrap()));
                Ok(response)
            },
            Err(e) => {
                error!("error fetching: {:?}", e);
                panic!("fetch error: {:?}", e);
                Err("fetch error")
            }
        }
    }
}

macro_rules! enabled {
    ($obj: expr, $value: expr) => {
        $obj.set_enabled($value.clone());
    }
}



macro_rules! call_on_self {
    ($self:ident, $F:ident) => {
        $self.$F()
    }
}