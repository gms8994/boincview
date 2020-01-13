extern crate ini;

use ini::Ini;
use std::collections::HashMap;
use std::str::FromStr;

const CONF_FILE_NAME: &str = "conf.ini";

#[derive(Debug)]
pub struct Endpoints {
    pub checkable : HashMap<Option<String>, Endpoint>,
}

impl Endpoints {
    fn new(&self) {

    }
}

#[derive(Debug, Clone)]
pub struct Endpoint {
    pub name: Option<String>,
    host: Option<String>,
    password: Option<String>,
    pub last_checked: Option<u64>,
    pub is_down: Option<bool>,
}

impl Endpoint {
    pub fn host(&self) -> Option<String> {
        Some(self.host.as_ref().unwrap().to_string())
    }
    pub fn password(&self) -> Option<&str> {
        Some(self.password.as_ref().unwrap().as_str())
    }
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

        for (key, mut value) in prop {
            match key.as_ref() {
                "host" => {
                    let mut host = &mut value;
                    host.push_str(":31416");
                    
                    endpoint.host = Some(host.to_string());
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

fn load() -> Ini {
    // If there's an error trying to load the conf.ini
    // then we should attempt to load the values from
    // the local config for BOINC directly, and then
    // immediately store them within the file, so that
    // we may then use the Ini values as necessary
    return Ini::load_from_file(CONF_FILE_NAME).unwrap();
}
