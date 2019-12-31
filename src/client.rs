use rpc::Client;
use std::option::Option;
use rpc::errors::*;
use std::sync::Mutex;
use std::sync::Arc;
use std::collections::HashMap;
use std::thread;

pub trait SimpleClient {
    fn tasks(&mut self) -> Result<Vec<rpc::models::Result>, Error>;
    fn projects(&mut self) -> Result<Vec<rpc::models::ProjectInfo>, Error>;
    fn populate(
        &mut self,
        hostname : &Option<String>,
        task_list : &mut HashMap<String, Vec<rpc::models::Result>>,
        project_list : &mut HashMap<String, String>
    );
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
        task_list : &mut HashMap<String, Vec<rpc::models::Result>>,
        project_list : &mut HashMap<String, String>
    ) {
        let hostname = hostname.as_ref();

        // If the client returned some tasks, add them to the guarded task
        match self.tasks() {
            Ok(tasks) => {
                task_list.insert(hostname.unwrap().to_string(), tasks);
            },
            Err(_error) => {
                return;
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
                return;
            }
        }
    }

}
