extern crate encoding;
extern crate boinc_rpc as rpc;
extern crate gtk;
extern crate gio;
extern crate glib;
extern crate ini;

pub mod tasks;
pub mod client;

use std::sync::Mutex;
use std::sync::Arc;
use std::time::Duration;
use std::thread;
use std::str::FromStr;
use std::collections::HashMap;
use client::*;
use gio::prelude::*;
use gtk::prelude::*;
use ini::Ini;
use std::rc::Rc;
use tasks::*;

const CONF_FILE_NAME: &str = "conf.ini";

#[repr(i32)]
enum Columns {
    Host,
    Name,
    Platform,
    Project,
    ElapsedTime,
    ExitStatus,
    State,
    ReportDeadline,
    ReceivedTime,
    EstimatedTimeRemaining,
    CompletedTime,
    Progress,
    ProgressString,
}

fn main() {
    let application = gtk::Application::new(
        Some("com.github.gtk-rs.examples.basic"),
        Default::default(),
    ).expect("failed to initialize GTK application");

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

    let model = Rc::new(create_model(clients));

    application.connect_activate(move |app| {
        let window = gtk::ApplicationWindow::new(app);
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

        let sw = gtk::ScrolledWindow::new(None::<&gtk::Adjustment>, None::<&gtk::Adjustment>);
        sw.set_shadow_type(gtk::ShadowType::EtchedIn);
        sw.set_policy(gtk::PolicyType::Automatic, gtk::PolicyType::Automatic);
        vbox.add(&sw);

        let treeview = gtk::TreeView::new_with_model(&*model);
        treeview.set_vexpand(true);

        sw.add(&treeview);

        add_columns(&treeview);

        window.show_all();
    });

    application.run(&[]);
}

fn create_model(clients : HashMap<Option<String>, rpc::SimpleClient>) -> gtk::ListStore {
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

    let store = get_data_for_model(gtk::ListStore::new(&col_types), clients);

    return store;
}

fn get_data_for_model(store : gtk::ListStore, clients : HashMap<Option<String>, rpc::SimpleClient>) -> gtk::ListStore {
    store.clear();

    let task_list : Arc<Mutex<HashMap<String, Vec<rpc::models::Result>>>> = Arc::new(Mutex::new(HashMap::new()));
    let project_list : Arc<Mutex<HashMap<String, String>>> = Arc::new(Mutex::new(HashMap::new()));

    for (hostname, mut client) in clients {
        let task_list = task_list.clone();
        let project_list = project_list.clone();

        thread::spawn(move || {
            let mut guarded_task = task_list.lock().unwrap();
            let mut guarded_project = project_list.lock().unwrap();

            let hostname = hostname.as_ref();

            println!("Connecting to {:?} to gather data", hostname);
            let tasks = client.tasks();
            let projects = client.projects();
            println!("Done connecting to {:?} to gather data", hostname);

            guarded_task.insert(hostname.unwrap().to_string(), tasks);

            for project in projects {
                guarded_project.insert(project.url.unwrap(), project.name.unwrap());
            }

        });
    }

    let col_indices: [u32; 13] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];

    let projects = project_list.lock().unwrap();

    println!("{:?}", task_list.lock().unwrap());

    // Seems like this isn't actually looping?
    for (hostname, tasks) in &*task_list.lock().unwrap() {
        for (_, d) in tasks.iter().enumerate() {
            &d.time_left();

            let values: [&dyn ToValue; 13] = [
                &hostname,
                &d.name,
                &d.platform,
                &d.project_url,
                // projects[&d.project_url].as_ref().unwrap(),
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

    return store;
}

fn add_columns(treeview: &gtk::TreeView) {
    {
        let renderer = gtk::CellRendererText::new();
        let column = gtk::TreeViewColumn::new();
        column.pack_start(&renderer, true);
        column.set_title("Host");
        column.add_attribute(&renderer, "text", Columns::Host as i32);
        column.set_sort_column_id(Columns::Host as i32);
        column.set_min_width(50);
        column.set_alignment(0.0);
        column.set_resizable(true);
        column.set_reorderable(true);
        treeview.append_column(&column);
    }
    {
        let renderer = gtk::CellRendererText::new();
        let column = gtk::TreeViewColumn::new();
        column.pack_start(&renderer, true);
        column.set_title("Project");
        column.add_attribute(&renderer, "text", Columns::Project as i32);
        column.set_sort_column_id(Columns::Project as i32);
        column.set_min_width(50);
        column.set_alignment(0.0);
        column.set_resizable(true);
        column.set_reorderable(true);
        treeview.append_column(&column);
    }
    {
        let renderer = gtk::CellRendererText::new();
        let column = gtk::TreeViewColumn::new();
        column.pack_start(&renderer, true);
        column.set_title("Name");
        column.add_attribute(&renderer, "text", Columns::Name as i32);
        column.set_sort_column_id(Columns::Name as i32);
        column.set_min_width(50);
        column.set_alignment(0.0);
        column.set_resizable(true);
        column.set_reorderable(true);
        treeview.append_column(&column);
    }
    {
        let renderer = gtk::CellRendererText::new();
        let column = gtk::TreeViewColumn::new();
        column.pack_start(&renderer, true);
        treeview.append_column(&column);
    }
    {
        let renderer = gtk::CellRendererText::new();
        let column = gtk::TreeViewColumn::new();
        column.pack_start(&renderer, true);
        column.set_title("Progress");
        column.add_attribute(&renderer, "text", Columns::ProgressString as i32);
        column.set_sort_column_id(Columns::Progress as i32);
        column.set_min_width(50);
        column.set_alignment(0.0);
        column.set_resizable(true);
        column.set_reorderable(true);
        treeview.append_column(&column);
    }
    {
        let renderer = gtk::CellRendererText::new();
        let column = gtk::TreeViewColumn::new();
        column.pack_start(&renderer, true);
        column.set_title("Platform");
        column.add_attribute(&renderer, "text", Columns::Platform as i32);
        column.set_sort_column_id(Columns::Platform as i32);
        column.set_min_width(50);
        column.set_alignment(0.0);
        column.set_resizable(true);
        column.set_reorderable(true);
        treeview.append_column(&column);
    }
    {
        let renderer = gtk::CellRendererText::new();
        let column = gtk::TreeViewColumn::new();
        column.pack_start(&renderer, true);
        column.set_title("Elapsed Time");
        column.add_attribute(&renderer, "text", Columns::ElapsedTime as i32);
        column.set_sort_column_id(Columns::ElapsedTime as i32);
        column.set_min_width(50);
        column.set_alignment(0.0);
        column.set_resizable(true);
        column.set_reorderable(true);
        treeview.append_column(&column);
    }
    {
        let renderer = gtk::CellRendererText::new();
        let column = gtk::TreeViewColumn::new();
        column.pack_start(&renderer, true);
        column.set_title("Time Left");
        column.add_attribute(&renderer, "text", Columns::EstimatedTimeRemaining as i32);
        column.set_sort_column_id(Columns::EstimatedTimeRemaining as i32);
        column.set_min_width(50);
        column.set_alignment(0.0);
        column.set_resizable(true);
        column.set_reorderable(true);
        treeview.append_column(&column);
    }
    {
        let renderer = gtk::CellRendererText::new();
        let column = gtk::TreeViewColumn::new();
        column.pack_start(&renderer, true);
        column.set_title("ExitStatus");
        column.add_attribute(&renderer, "text", Columns::ExitStatus as i32);
        column.set_sort_column_id(Columns::ExitStatus as i32);
        column.set_min_width(50);
        column.set_alignment(0.0);
        column.set_resizable(true);
        column.set_reorderable(true);
        treeview.append_column(&column);
    }
    {
        let renderer = gtk::CellRendererText::new();
        let column = gtk::TreeViewColumn::new();
        column.pack_start(&renderer, true);
        column.set_title("State");
        column.add_attribute(&renderer, "text", Columns::State as i32);
        column.set_sort_column_id(Columns::State as i32);
        column.set_min_width(50);
        column.set_alignment(0.0);
        column.set_resizable(true);
        column.set_reorderable(true);
        treeview.append_column(&column);
    }
    {
        let renderer = gtk::CellRendererText::new();
        let column = gtk::TreeViewColumn::new();
        column.pack_start(&renderer, true);
        column.set_title("ReportDeadline");
        column.add_attribute(&renderer, "text", Columns::ReportDeadline as i32);
        column.set_sort_column_id(Columns::ReportDeadline as i32);
        column.set_min_width(50);
        column.set_alignment(0.0);
        column.set_resizable(true);
        column.set_reorderable(true);
        treeview.append_column(&column);
    }
    {
        let renderer = gtk::CellRendererText::new();
        let column = gtk::TreeViewColumn::new();
        column.pack_start(&renderer, true);
        column.set_title("ReceivedTime");
        column.add_attribute(&renderer, "text", Columns::ReceivedTime as i32);
        column.set_sort_column_id(Columns::ReceivedTime as i32);
        column.set_min_width(50);
        column.set_alignment(0.0);
        column.set_resizable(true);
        column.set_reorderable(true);
        treeview.append_column(&column);
    }
    {
        let renderer = gtk::CellRendererText::new();
        let column = gtk::TreeViewColumn::new();
        column.pack_start(&renderer, true);
        column.set_title("CompletedTime");
        column.add_attribute(&renderer, "text", Columns::CompletedTime as i32);
        column.set_sort_column_id(Columns::CompletedTime as i32);
        column.set_min_width(50);
        column.set_alignment(0.0);
        column.set_resizable(true);
        column.set_reorderable(true);
        treeview.append_column(&column);
    }
}
