use gtk::prelude::*;
use std::collections::HashMap;
use std::time::SystemTime;

use crate::client::*;
use crate::tasks::*;
use crate::config::*;

pub fn get_data_for_model(store : &gtk::ListStore, endpoints : &mut Endpoints) {
    store.clear();

    let mut task_list : HashMap<String, Vec<rpc::models::TaskResult>> = HashMap::new();
    let mut project_list : HashMap<String, String> = HashMap::new();

    for (hostname, mut endpoint) in endpoints.checkable.clone() {
        println!("Going to update host {:?}", hostname);
        if endpoint.is_down.unwrap() {
            if get_now() - endpoint.last_checked.unwrap() <= 90 {
                println!("No need to update host {:?} - it's down", hostname);
                continue;
            }

            println!("Host {:?} has been down more than 90s - rechecking for up", hostname);
            endpoint.is_down = Some(false);
        }

        let (client_tasks, client_projects);

        println!("Instantiating client");
        let mut client = match crate::config::client(&endpoint) {
            Ok(client) => client,
            Err(error) => panic!(error),
        };
        println!("Done instantiating client");

        let population_result = client.populate(&hostname);
        match population_result {
            Ok(result) => {
                client_tasks = result.tasks;
                client_projects = result.projects;
                endpoints.checkable.get_mut(&hostname).unwrap().last_checked = Some(get_now());
            },
            Err(error) => {
                println!("Host {:?} responded with {:?} - last_checked {:?}", hostname, error, endpoints.checkable.get(&hostname).unwrap().last_checked);

                endpoint.is_down = Some(true);
                endpoint.last_checked = Some(get_now());
                continue;
            }
        }

        println!("Host {:?} successfully updated: last_checked {:?}", hostname, endpoints.checkable.get(&hostname).unwrap().last_checked);

        task_list.extend(client_tasks);
        project_list.extend(client_projects);
    }

    let col_indices: [u32; 14] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13];

    // Seems like this isn't actually looping?
    for (hostname, tasks) in task_list {
        for (_, d) in tasks.iter().enumerate() {
            let values: [&dyn ToValue; 14] = [
                &hostname,
                &project_list[d.project_url.as_ref().unwrap()],
                &d.name,
                &format!("{0:.2} %", d.progress()),
                &d.elapsed_as_string(),
                &d.remaining_as_string(),
                &d.state(),
                &d.report_deadline.unwrap(),
                &d.received_time.unwrap(),
                &d.completed_time.unwrap_or(0.0),
                &d.platform,
                &d.progress(),
                &d.elapsed(),
                &d.remaining(),
            ];

            store.set(&store.append(), &col_indices, &values);
        }
    }
}

pub fn create_model() -> gtk::ListStore {
    let col_types: [glib::types::Type; 14] = [
        glib::types::Type::String,
        glib::types::Type::String,
        glib::types::Type::String,
        glib::types::Type::String,
        glib::types::Type::String,
        glib::types::Type::String,
        glib::types::Type::String,
        glib::types::Type::F64,
        glib::types::Type::F64,
        glib::types::Type::F64,
        glib::types::Type::String,
        glib::types::Type::F64,
        glib::types::Type::F64,
        glib::types::Type::F64,
    ];

    return gtk::ListStore::new(&col_types);
}

fn get_now() -> u64 {
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .expect("couldn't get start time")
        .as_secs()
}
