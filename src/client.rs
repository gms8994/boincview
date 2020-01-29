use rpc::Client;
use rpc::errors::*;
use std::collections::HashMap;
use std::option::Option;
use futures::future::Future;
use tokio::prelude::*;

pub struct PopulatedResults {
    pub tasks: HashMap<String, Vec<rpc::models::TaskResult>>,
    pub projects: HashMap<String, String>,
}

pub trait SimpleClient {
    fn tasks(&mut self) -> Result<Vec<rpc::models::TaskResult>, Error>;
    fn projects(&mut self) -> Result<Vec<rpc::models::ProjectInfo>, Error>;
    fn populate(&mut self, hostname : &Option<String>) -> Result<PopulatedResults, Error>;
}

impl SimpleClient for rpc::Client {
    #[tokio::main]
    async fn tasks(&mut self) -> Result<Vec<rpc::models::TaskResult>, Error> {
        let get_results_task = self.get_results(false);

        return futures::executor::block_on(async {
            let results = get_results_task.await;
            println!("tasks {:?}", results);
            results
        });
    }

    #[tokio::main]
    async fn projects(&mut self) -> Result<Vec<rpc::models::ProjectInfo>, Error> {
        let get_projects_task = self.get_projects();

        return futures::executor::block_on(async {
            let results = get_projects_task.await;
            println!("projects {:?}", results);
            results
        });
    }

    fn populate(&mut self, hostname : &Option<String>) -> Result<PopulatedResults, Error> {
        let mut task_list : HashMap<String, Vec<rpc::models::TaskResult>> = HashMap::new();
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
