extern crate boinc_rpc as rpc;
extern crate encoding;
extern crate gio;
extern crate glib;
extern crate gtk;
extern crate ini;

pub mod client;
pub mod tasks;
pub mod ui;

use client::*;
use std::rc::Rc;
use std::cell::RefCell;
use gio::prelude::*;
use gtk::prelude::*;
use ini::Ini;
use std::collections::HashMap;
use std::str::FromStr;
use std::time::SystemTime;
use tasks::*;

const CONF_FILE_NAME: &str = "conf.ini";
const APPLICATION_NAME: &str = "com.github.gtk-rs.examples.basic";

fn main() {
    let application = gtk::Application::new(Some(APPLICATION_NAME), gio::ApplicationFlags::empty())
        .expect("Initialization failed...");

    application.connect_startup(move |app| {
        build_ui(app);
    });

    application.run(&[]);
}

fn create_model() -> gtk::ListStore {
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

fn get_data_for_model(store : &gtk::ListStore, clients : &mut HashMap<Option<String>, rpc::SimpleClient>, downed_clients : &mut HashMap<Option<String>, u64>) {
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

fn build_ui(application: &gtk::Application) {
    // If there's an error trying to load the conf.ini
    // then we should attempt to load the values from
    // the local config for BOINC directly, and then
    // immediately store them within the file, so that
    // we may then use the Ini values as necessary
    let conf = match Ini::load_from_file(CONF_FILE_NAME) {
        Ok(config) => config,
        Err(error) => {
            panic!("Missing config file: {:?}", error)
        }
    };

    let mut clients = HashMap::new();
    let mut downed_clients = HashMap::new();

    // Future work
    for (host, prop) in conf {
        let mut client = rpc::SimpleClient::default();
        for (key, value) in prop {
            match key.as_ref() {
                "host" => {
                    let addr = match std::net::Ipv4Addr::from_str(&value) {
                        Ok(address) => address,
                        Err(error) => panic!(error)
                    };

                    client.addr = std::net::SocketAddr::new(std::net::IpAddr::V4(addr), 31416);
                },
                "password" => {
                    client.password = value.into();
                },
                _ => panic!("Unhandled {}", key)
            }
        }

        clients.insert(host, client);
    }

    let model = Rc::new(RefCell::new(create_model()));

    let window = gtk::ApplicationWindow::new(application);
    window.set_title("BOINCView");
    window.set_default_size(1024, 768);

    let paned_window = gtk::Paned::new(gtk::Orientation::Horizontal);

    // Set all of the items on the host frame
    let host_frame = gtk::Frame::new(None);
    gtk::FrameExt::set_shadow_type(&host_frame, gtk::ShadowType::In);
    gtk::Paned::pack1(&paned_window, &host_frame, true, true);
    gtk::WidgetExt::set_size_request(&host_frame, 200, -1);

    let host_frame_box = gtk::Box::new(gtk::Orientation::Vertical, 8);
    host_frame.add(&host_frame_box);

    let host_scrollable_window = gtk::ScrolledWindow::new(None::<&gtk::Adjustment>, None::<&gtk::Adjustment>);
    host_scrollable_window.set_shadow_type(gtk::ShadowType::EtchedIn);
    host_scrollable_window.set_policy(gtk::PolicyType::Automatic, gtk::PolicyType::Automatic);
    host_frame_box.add(&host_scrollable_window);

    // let host_treeview = gtk::TreeView::new_with_model(&*model.borrow());
    // host_treeview.set_vexpand(true);

    // host_scrollable_window.add(&host_treeview);
    // End setting data on the data frame

    // Set all of the items on the data frame
    let data_frame = gtk::Frame::new(None);
    gtk::FrameExt::set_shadow_type(&data_frame, gtk::ShadowType::In);
    gtk::Paned::pack2(&paned_window, &data_frame, true, true);
    gtk::WidgetExt::set_size_request(&data_frame, 568, -1);

    let data_frame_box = gtk::Box::new(gtk::Orientation::Vertical, 8);
    data_frame.add(&data_frame_box);

    let data_scrollable_window = gtk::ScrolledWindow::new(None::<&gtk::Adjustment>, None::<&gtk::Adjustment>);
    data_scrollable_window.set_shadow_type(gtk::ShadowType::EtchedIn);
    data_scrollable_window.set_policy(gtk::PolicyType::Automatic, gtk::PolicyType::Automatic);
    data_frame_box.add(&data_scrollable_window);

    let data_treeview = gtk::TreeView::new_with_model(&*model.borrow());
    data_treeview.set_vexpand(true);

    data_scrollable_window.add(&data_treeview);

    ui::add_data_columns(&data_treeview);

    get_data_for_model(&model.borrow(), &mut clients, &mut downed_clients);
    // End setting data on the data frame

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

    Some(gtk::timeout_add(
        30000,
        move || {
            println!("Updating models");
            get_data_for_model(&model.borrow(), &mut clients, &mut downed_clients);
            println!("Done updating models");

            glib::Continue(true)
        }
    ));

    window.add(&paned_window);
    application.connect_activate(move |_| {
        window.show_all();
        // filter_entry.hide();
        window.present();
    });
}

fn get_now() -> u64 {
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .expect("couldn't get start time")
        .as_secs()
}
