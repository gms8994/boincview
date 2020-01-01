use rpc::Client;
use rpc::errors::*;
use std::collections::HashMap;
use std::option::Option;

pub struct PopulatedResults {
    pub tasks: HashMap<String, Vec<rpc::models::Result>>,
    pub projects: HashMap<String, String>,
}

pub trait SimpleClient {
    fn tasks(&mut self) -> Result<Vec<rpc::models::Result>, Error>;
    fn projects(&mut self) -> Result<Vec<rpc::models::ProjectInfo>, Error>;
    fn populate(&mut self, hostname : &Option<String>) -> Result<PopulatedResults, Error>;
}

impl SimpleClient for rpc::SimpleClient {
    fn tasks(&mut self) -> Result<Vec<rpc::models::Result>, Error> {
        let results = self.get_results(false);
        return results;
    }

    fn projects(&mut self) -> Result<Vec<rpc::models::ProjectInfo>, Error> {
        return self.get_projects();
    }

    fn populate(&mut self, hostname : &Option<String>) -> Result<PopulatedResults, Error> {
        let mut task_list : HashMap<String, Vec<rpc::models::Result>> = HashMap::new();
        let mut project_list : HashMap<String, String> = HashMap::new();

        let hostname = hostname.as_ref();

        // If the client returned some tasks, add them to the guarded task
        match self.tasks() {
            Ok(tasks) => {
                task_list.insert(hostname.unwrap().to_string(), tasks);
            },
            Err(error) => {
                return Err(error);
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
            Err(error) => {
                return Err(error);
            }
        }

        Ok(PopulatedResults {
            tasks: task_list,
            projects: project_list
        })
    }

}
