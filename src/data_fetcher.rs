use gtk::prelude::*;
use std::collections::HashMap;
use std::time::SystemTime;

use crate::tasks::*;
use crate::config::*;

pub async fn get_data_for_model(store : &gtk::ListStore, endpoints : &mut Endpoints) {
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

        let mut client = rpc::Client::connect(
            endpoint.host().expect("No host provided"),
            endpoint.password()
        ).await.unwrap();

        task_list.insert(hostname.as_ref().unwrap().to_string(), client.get_results(false).await.unwrap());

        for project in client.get_projects().await.unwrap() {
            project_list.insert(project.url.unwrap(), project.name.unwrap());
        }

        println!("Host {:?} successfully updated: last_checked {:?}", hostname, endpoints.checkable.get(&hostname).unwrap().last_checked);
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
