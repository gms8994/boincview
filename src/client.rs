use rpc::Client;
use std::collections::HashMap;
use std::option::Option;

pub trait SimpleClient {
    fn tasks(&mut self) -> Vec<rpc::models::Result>;
    fn projects(&mut self) -> HashMap<Option<String>, Option<String>>;
}

impl SimpleClient for rpc::SimpleClient {
    fn tasks(&mut self) -> Vec<rpc::models::Result> {
        let tasks = match self.get_results(false) {
            Ok(tasks) => tasks,
            Err(error) => {
                panic!("There was a problem connecting to BOINC: {:?}", error)
            }
        };

        return tasks;
    }

    fn projects(&mut self) -> HashMap<Option<String>, Option<String>> {
        let mut project_list = HashMap::new();

        let projects = self.get_projects();

        let projects = match projects {
             Ok(list) => list,
             Err(error) => {
                 panic!("There was a problem connecting to BOINC: {:?}", error)
             }
        };

        for project in projects {
            project_list.insert(project.url, project.name);
        }

        return project_list;
    }
}
