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
    let col_types: [glib::types::Type; 13] = [
        glib::types::Type::String,
        glib::types::Type::String,
        glib::types::Type::String,
        glib::types::Type::String,
        glib::types::Type::F64,
        glib::types::Type::I64,
        glib::types::Type::String,
        glib::types::Type::F64,
        glib::types::Type::F64,
        glib::types::Type::F64,
        glib::types::Type::F64,
        glib::types::Type::F64,
        glib::types::Type::String,
    ];

    return gtk::ListStore::new(&col_types);
}

fn get_data_for_model(store : &gtk::ListStore, clients : &mut HashMap<Option<String>, rpc::SimpleClient>) {
    store.clear();

    let mut task_list : HashMap<String, Vec<rpc::models::Result>> = HashMap::new();
    let mut project_list : HashMap<String, String> = HashMap::new();

    for (hostname, client) in clients {
        let (client_tasks, client_projects);
        let population_result = client.populate(&hostname);
        match population_result {
            Ok(result) => {
                client_tasks = result.tasks;
                client_projects = result.projects;
            },
            Err(_error) => {
                println!("I want to remove {:?} as a host from the list as it's down", hostname);

                continue;
            }
        }

        task_list.extend(client_tasks);
        project_list.extend(client_projects);
    }

    let col_indices: [u32; 13] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];

    // Seems like this isn't actually looping?
    for (hostname, tasks) in task_list {
        for (_, d) in tasks.iter().enumerate() {
            let values: [&dyn ToValue; 13] = [
                &hostname,
                &d.name,
                &d.platform,
                &project_list[d.project_url.as_ref().unwrap()],
                &d.final_elapsed_time.unwrap(),
                &d.exit_status.unwrap(),
                &d.state(),
                &d.report_deadline.unwrap(),
                &d.received_time.unwrap(),
                &d.estimated_cpu_time_remaining.unwrap(),
                &d.completed_time.unwrap_or(0.0),
                &d.progress(),
                &format!("{0:.2} %", d.progress()),
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
    let host_frame = gtk::Frame::new(None);
    let data_frame = gtk::Frame::new(None);

    gtk::FrameExt::set_shadow_type(&host_frame, gtk::ShadowType::In);
    gtk::FrameExt::set_shadow_type(&data_frame, gtk::ShadowType::In);

    gtk::Paned::pack1(&paned_window, &host_frame, true, true);
    gtk::WidgetExt::set_size_request(&host_frame, 200, -1);

    gtk::Paned::pack2(&paned_window, &data_frame, true, true);
    gtk::WidgetExt::set_size_request(&data_frame, 568, -1);

    window.add(&paned_window);

    let vbox = gtk::Box::new(gtk::Orientation::Vertical, 8);
    data_frame.add(&vbox);

    let scrolled_window = gtk::ScrolledWindow::new(None::<&gtk::Adjustment>, None::<&gtk::Adjustment>);
    scrolled_window.set_shadow_type(gtk::ShadowType::EtchedIn);
    scrolled_window.set_policy(gtk::PolicyType::Automatic, gtk::PolicyType::Automatic);
    vbox.add(&scrolled_window);

    let treeview = gtk::TreeView::new_with_model(&*model.borrow());
    treeview.set_vexpand(true);

    scrolled_window.add(&treeview);

    ui::add_columns(&treeview);

    get_data_for_model(&model.borrow(), &mut clients);

    Some(gtk::timeout_add(
        30000,
        move || {
            println!("Updating models");
            get_data_for_model(&model.borrow(), &mut clients);
            println!("Done updating models");

            glib::Continue(true)
        }
    ));

    application.connect_activate(move |_| {
        window.show_all();
        // filter_entry.hide();
        window.present();
    });
}
