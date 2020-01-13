use rpc::errors::*;
use std::collections::HashMap;
use std::option::Option;

pub struct PopulatedResults {
    pub tasks: HashMap<String, Vec<rpc::models::TaskResult>>,
    pub projects: HashMap<String, String>,
}

pub async fn populate(client : &mut rpc::Client, hostname : &Option<String>) -> Result<PopulatedResults, Error> {
    let mut task_list : HashMap<String, Vec<rpc::models::TaskResult>> = HashMap::new();
    let mut project_list : HashMap<String, String> = HashMap::new();

    let hostname = hostname.as_ref();

    // If the client returned some tasks, add them to the guarded task
    task_list.insert(hostname.unwrap().to_string(), client.get_results(false).await.unwrap());

    // If the client returned some projects, loop over them adding to
    // the guarded project
    for project in client.get_projects().await.unwrap() {
        project_list.insert(project.url.unwrap(), project.name.unwrap());
    }

    Ok(PopulatedResults {
        tasks: task_list,
        projects: project_list
    })
}
