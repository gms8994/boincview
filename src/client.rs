use rpc::Client;
use std::option::Option;
use rpc::errors::*;

pub trait SimpleClient {
    fn tasks(&mut self) -> Result<Vec<rpc::models::Result>, Error>;
    fn projects(&mut self) -> Result<Vec<rpc::models::ProjectInfo>, Error>;
}

impl SimpleClient for rpc::SimpleClient {
    fn tasks(&mut self) -> Result<Vec<rpc::models::Result>, Error> {
        return self.get_results(false);
    }

    fn projects(&mut self) -> Result<Vec<rpc::models::ProjectInfo>, Error> {
        return self.get_projects();
    }
}
