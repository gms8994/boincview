use gtk::prelude::*;
use std::collections::HashMap;
use std::time::SystemTime;

use crate::client::*;
use crate::tasks::*;

pub fn get_data_for_model(store : &gtk::ListStore, clients : &mut HashMap<Option<String>, rpc::SimpleClient>, downed_clients : &mut HashMap<Option<String>, u64>) {
    store.clear();

    let mut task_list : HashMap<String, Vec<rpc::models::Result>> = HashMap::new();
    let mut project_list : HashMap<String, String> = HashMap::new();

    for (hostname, client) in clients {
        println!("Going to update host {:?}", hostname);
        if downed_clients.contains_key(hostname) {
            if get_now() - downed_clients.get(hostname).unwrap() <= 90 {
                println!("No need to update host {:?} - it's down", hostname);
                continue;
            }

            println!("Host {:?} has been down more than 90s - rechecking for up", hostname);
            downed_clients.remove(hostname);
        }

        let (client_tasks, client_projects);
        let population_result = client.populate(&hostname);
        match population_result {
            Ok(result) => {
                client_tasks = result.tasks;
                client_projects = result.projects;
            },
            Err(error) => {
                println!("Host {:?} responded with {:?}", hostname, error);
                let start_time = get_now();

                downed_clients.insert(Some(hostname.as_ref().unwrap().to_string()), start_time);
                continue;
            }
        }

        println!("Host {:?} successfully updated", hostname);

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
