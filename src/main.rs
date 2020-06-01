use tokio;
use std::collections::HashMap;
use boinc_rpc::{models};

fn main() {
    let mut hosts = Vec::new();
    // Project Identifier -> Project Info
    let mut project_list: HashMap<String, boinc_rpc::models::ProjectInfo> = HashMap::new();

    let host = Host::new("127.0.0.1:31416", Some("1033644eaad1ea7d91bc48a749f1620b"));
    hosts.push(host);

    tokio::runtime::Runtime::new().unwrap().block_on(async {
        update_projects_list(&mut project_list, &mut hosts).await;
    });

    // We have all of the projects across all of the hosts
    // Now we can start fetching tasks and then mapping
    // them back to the correct project information
    tokio::runtime::Runtime::new().unwrap().block_on(async {
        update_task_list(&mut hosts).await;
    });
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

async fn update_projects_list(projects : &mut HashMap<String, boinc_rpc::models::ProjectInfo>, hosts : &mut Vec<Host>)
{
    for (_idx, host) in hosts.into_iter().enumerate() {
        let transport = boinc_rpc::Transport::new(host.addr, host.password);
        let mut client = boinc_rpc::Client::new(transport);

        let client_projects = match client.get_projects().await {
            Ok(t) => Some(t),
            Err(t) => panic!(t),
        };

        for (_pidx, project) in client_projects.unwrap().into_iter().enumerate() {
            let c_project = project.clone();

            if let Some(project_identifier) = project.url {
                if ! projects.contains_key(&project_identifier) {
                    projects.insert(project_identifier, c_project);
                }
            }
        }
    }
}

async fn update_task_list(hosts : &mut Vec<Host>)
{
    for (_idx, host) in hosts.into_iter().enumerate() {
        let transport = boinc_rpc::Transport::new(host.addr, host.password);
        let mut client = boinc_rpc::Client::new(transport);

        let client_tasks = match client.get_results(false).await {
            Ok(t) => t,
            Err(t) => panic!(t),
        };

        host.results = Some(client_tasks);

        dbg!(&host.results);
    }
}
