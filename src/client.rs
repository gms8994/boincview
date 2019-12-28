use rpc::Client;
use std::option::Option;

pub trait SimpleClient {
    fn tasks(&mut self) -> Vec<rpc::models::Result>;
    fn projects(&mut self) -> Vec<rpc::models::ProjectInfo>;
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

    fn projects(&mut self) -> Vec<rpc::models::ProjectInfo> {
        let projects = self.get_projects();

        return match projects {
             Ok(list) => list,
             Err(error) => {
                 panic!("There was a problem connecting to BOINC: {:?}", error)
             }
        };
    }
}
