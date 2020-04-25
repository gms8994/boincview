//!
#![warn(missing_debug_implementations, rust_2018_idioms, missing_docs)]

fn main() {
    tokio::runtime::Runtime::new().unwrap().block_on(async {
        let transport = boinc_rpc::Transport::new("127.0.0.1:31416", Some("77c858a8407c88f53d261e4849967cdd"));
        let mut client = boinc_rpc::Client::new(transport);

        let projects = match client.get_projects().await {
            Ok(t) => t,
            Err(t) => panic!("Can't connect to 127.0.0.1:31416"),
        };

        let results = match client.get_results(false).await {
            Ok(t) => t,
            Err(t) => panic!("Can't connect to 127.0.0.1:31416"),
        };

        for i in 0..projects.len() {
            let project = &projects[i];

            println!("{:?}\n", project);
        }

        for i in 0..results.len() {
            let result = &results[i];

            println!("{:?}\n", result);
        }
    })
}
