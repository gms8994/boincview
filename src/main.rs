use tokio;
use boinc_rpc::{models};

fn main() {
    println!("program started");

    let mut hosts = Vec::new();

    let host = Host::new("127.0.0.1:31416", Some("1033644eaad1ea7d91bc48a749f1620b"));
    hosts.push(host);

    tokio::runtime::Runtime::new().unwrap().block_on(async {
        fetch_from_hosts(&mut hosts).await;
    });

    dbg!(&hosts);

    println!("program finished");
}

#[derive(Debug)]
pub struct Host {
    addr: &'static str,
    password: Option<&'static str>,
    projects: Option<Vec<models::ProjectInfo>>,
    results: Option<Vec<models::TaskResult>>,
}

impl Host {
    pub fn new(addr : &'static str, password : Option<&'static str>) -> Self {
        Host {
            addr : addr,
            password : password,
            projects : None,
            results : None,
        }
    }
}

async fn fetch_from_hosts(hosts : &mut Vec<Host>)
{
    for (idx, host) in hosts.into_iter().enumerate() {
        let transport = boinc_rpc::Transport::new(host.addr, host.password);
        let mut client = boinc_rpc::Client::new(transport);

        let projects = match client.get_projects().await {
            Ok(t) => Some(t),
            Err(t) => panic!(t),
        };

        host.projects = projects;

        let results = match client.get_results(false).await {
            Ok(t) => Some(t),
            Err(t) => panic!(t),
        };

        host.results = results;
    }
}
