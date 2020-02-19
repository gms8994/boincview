#![feature(async_closure)]

extern crate boinc_rpc as rpc;
extern crate encoding;
extern crate gio;
extern crate glib;
extern crate gtk;
extern crate ini;

pub mod tasks;
pub mod ui;
pub mod config;

use crate::config::*;
use crate::tasks::*;
use std::rc::Rc;
use std::cell::RefCell;
use gio::prelude::*;
use gtk::prelude::*;
use std::collections::HashMap;
use std::time::SystemTime;
use rpc::errors::Error;

const APPLICATION_NAME: &str = "com.github.gtk-rs.examples.basic";

pub struct PopulatedResults {
    pub tasks: HashMap<String, Vec<rpc::models::TaskResult>>,
    pub projects: HashMap<String, String>,
}

#[tokio::main]
async fn main() {
    let application = gtk::Application::new(Some(APPLICATION_NAME), gio::ApplicationFlags::empty())
        .expect("Initialization failed...");

    application.connect_startup(async move |app| {
        build_ui(app).await;
    });

    application.run(&[]);
}

async fn build_ui(application: &gtk::Application) {
    let mut endpoints = config::get_endpoints();
    let model = Rc::new(RefCell::new(create_model()));

    let window = gtk::ApplicationWindow::new(application);
    window.set_title("BOINCView");
    window.set_default_size(1024, 768);

    let paned_window = gtk::Paned::new(gtk::Orientation::Horizontal);

    // Set up both of the panes in the window
    let (paned_window, _host) = ui::Window::new(paned_window, true, 200);
    let (paned_window, data) = ui::Window::new(paned_window, false, 568);

    // Set all of the items on the host frame
    // let host_treeview = gtk::TreeView::new_with_model(&*model.borrow());
    // host_treeview.set_vexpand(true);

    // host_scrollable_window.add(&host_treeview);
    // End setting data on the data frame

    // Set up the data-feed on the data pane
    let data_treeview = gtk::TreeView::new_with_model(&*model.borrow());
    data_treeview.set_vexpand(true);
    data.scrolled_window.add(&data_treeview);

    ui::add_data_columns(&data_treeview);

    get_data_for_model(&model.borrow(), &mut endpoints).await;
    Some(gtk::timeout_add(
        30000,
        async move || {
            get_data_for_model(&model.borrow(), &mut endpoints).await;

            glib::Continue(true)
        }
    ));

    // Need another timeout_add that simply iterates the model and increments
    // or decrements values as appropriate
    // Some(gtk::timeout_add(
    //     30000,
    //     move || {
    //         println!("Updating models");
    //         get_data_for_model(&model.borrow(), &mut clients, &mut downed_clients);
    //         println!("Done updating models");

    //         glib::Continue(true)
    //     }
    // ));


    window.add(&paned_window);

    application.connect_activate(move |_| {
        window.show_all();
        window.present();
    });
}

async fn get_data_for_model(store : &gtk::ListStore, endpoints : &mut Endpoints) {
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

        println!("Instantiating client {:?}", hostname);
        let mut client = boinc_rpc::Client::connect(endpoint.host().unwrap(), endpoint.password()).await.unwrap();
        println!("Done instantiating client");

        let population_result = populate(&client, &hostname).await;

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

fn get_now() -> u64 {
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .expect("couldn't get start time")
        .as_secs()
}

async fn populate(client : &rpc::Client, hostname : &Option<String>) -> Result<PopulatedResults, Error> {
    let mut task_list : HashMap<String, Vec<rpc::models::TaskResult>> = HashMap::new();
    let mut project_list : HashMap<String, String> = HashMap::new();

    let hostname = hostname.as_ref();

    // If the client returned some tasks, add them to the guarded task
    match client.get_results(false).await {
        Ok(tasks) => {
            task_list.insert(hostname.unwrap().to_string(), tasks);
        },
        Err(error) => {
            return Err(error);
        }
    }

    // If the client returned some projects, loop over them adding to
    // the guarded project
    match client.get_projects().await {
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
