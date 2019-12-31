use rpc::Client;
use rpc::errors::*;
use std::collections::HashMap;
use std::option::Option;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

pub trait SimpleClient {
    fn tasks(&mut self) -> Result<Vec<rpc::models::Result>, Error>;
    fn projects(&mut self) -> Result<Vec<rpc::models::ProjectInfo>, Error>;
    fn populate(
        &mut self,
        hostname : &Option<String>
    ) -> (HashMap<String, Vec<rpc::models::Result>>, HashMap<String, String>);
}

impl SimpleClient for rpc::SimpleClient {
    fn tasks(&mut self) -> Result<Vec<rpc::models::Result>, Error> {
        return self.get_results(false);
    }

    fn projects(&mut self) -> Result<Vec<rpc::models::ProjectInfo>, Error> {
        return self.get_projects();
    }

    fn populate(
        &mut self,
        hostname : &Option<String>,
    ) -> (HashMap<String, Vec<rpc::models::Result>>, HashMap<String, String>) {
        let mut task_list : HashMap<String, Vec<rpc::models::Result>> = HashMap::new();
        let mut project_list : HashMap<String, String> = HashMap::new();

        let hostname = hostname.as_ref();

        // If the client returned some tasks, add them to the guarded task
        match self.tasks() {
            Ok(tasks) => {
                task_list.insert(hostname.unwrap().to_string(), tasks);
            },
            Err(_error) => {
                return (task_list, project_list);
            }
        }

        // If the client returned some projects, loop over them adding to
        // the guarded project
        match self.projects() {
            Ok(projects) => {
                for project in projects {
                    project_list.insert(project.url.unwrap(), project.name.unwrap());
                }
            },
            Err(_error) => {
                return (task_list, project_list);
            }
        }

        return (task_list, project_list);
    }

}
