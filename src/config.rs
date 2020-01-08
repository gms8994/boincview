extern crate ini;

use ini::Ini;
use std::collections::HashMap;
use std::str::FromStr;

const CONF_FILE_NAME: &str = "conf.ini";

#[derive(Debug)]
pub struct Endpoints {
    pub checkable : HashMap<Option<String>, Endpoint>,
}

#[derive(Debug, Clone)]
pub struct Endpoint {
    name: Option<String>,
    host: Option<std::net::SocketAddr>,
    password: Option<String>,
    pub last_checked: Option<u64>,
    pub is_down: Option<bool>,
}

pub fn get_endpoints() -> Endpoints {
    let conf = load();

    let mut endpoints : HashMap<Option<String>, Endpoint> = HashMap::new();

    // Future work
    for (host, prop) in conf {
        let mut endpoint = Endpoint {
            name: host.clone(),
            host: None,
            password: None,
            last_checked: None,
            is_down: Some(false),
        };

        for (key, value) in prop {
            match key.as_ref() {
                "host" => {
                    let addr = match std::net::Ipv4Addr::from_str(&value) {
                        Ok(address) => address,
                        Err(error) => panic!(error)
                    };

                    endpoint.host = Some(std::net::SocketAddr::new(std::net::IpAddr::V4(addr), 31416));
                },
                "password" => {
                    endpoint.password = Some(value);
                },
                _ => panic!("Unhandled {}", key)
            }
        }

        endpoints.insert(host, endpoint);
    }

    Endpoints {
        checkable : endpoints,
    }
}

pub fn get_client(endpoints : &Endpoints, endpoint : &Endpoint) -> Result<rpc::SimpleClient, String> {
    for (_name, checkable_endpoint) in &endpoints.checkable {
        if endpoint.name == checkable_endpoint.name {
            let mut client = rpc::SimpleClient::default();
            client.addr = checkable_endpoint.host.unwrap();
            client.password = checkable_endpoint.password.clone();

            return Ok(client);
        }
    }

    return Err("No host found".to_string());
}

fn load() -> Ini {
    // If there's an error trying to load the conf.ini
    // then we should attempt to load the values from
    // the local config for BOINC directly, and then
    // immediately store them within the file, so that
    // we may then use the Ini values as necessary
    return Ini::load_from_file(CONF_FILE_NAME).unwrap();
}
