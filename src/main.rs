extern crate gtk;
extern crate glib;

use tokio;
use boinc_rpc::models::ProjectInfo;

mod lib;
use crate::lib::ModifiedResult;
use crate::lib::Host;

use gio::prelude::*;
use gtk::prelude::*;
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;

const APPLICATION_NAME: &str = "com.github.gtk-rs.examples.basic";

fn main() {
    let application = gtk::Application::new(Some(APPLICATION_NAME), gio::ApplicationFlags::empty())
        .expect("Initialization failed...");

    application.connect_startup(move |app| {
        build_ui(app);
    });

    application.run(&[]);
}

fn get_necessary_data_from_hosts(mut project_list : &mut HashMap<Option<String>, ProjectInfo>) -> Vec<Host>
{
    let mut hosts = Vec::new();

    hosts.push(Host::new("127.0.0.1:31416", Some("1033644eaad1ea7d91bc48a749f1620b")));
    hosts.push(Host::new("192.168.1.113:31416", Some("95405a40b449164295bba46fa405cc1b")));
    // hosts.push(Host::new("192.168.1.108:31416", Some("5e09d64108b3871cae6ef4bd0c599c69")));

    for mut host in hosts.iter() {
        // Fetch all of the tasks, then fetch the projects
        tokio::runtime::Runtime::new().unwrap().block_on(async {
            update_tasks_for_host(&mut host).await;
        });
    }

    // tokio::runtime::Runtime::new().unwrap().block_on(async {
    //     update_projects_list(&hosts, &mut project_list).await;
    // });

    hosts
}

async fn update_projects_list(hosts : &Vec<Host>, mut project_list : &mut HashMap<Option<String>, ProjectInfo>)
{
    for (_idx, host) in hosts.into_iter().enumerate() {
        // Here we want to check to see if we need to fetch
        // the projects - at this point, we should have the
        // list of tasks, and so we can check to see if any
        // of the tasks *don't* have an entry in the projects
        // list - if so, we need to fetch the proejcts list
        // from the host

        let mut has_missing_projects : bool = false;

        if let Some(results) = host.results.clone() {
            for (_, result) in results.into_iter().enumerate() {
                if ! project_list.contains_key(&result.project_url) {
                    has_missing_projects = true;
                }
            }
        }

        if ! has_missing_projects {
            return;
        }

        println!("Some projects are missing - we need to fetch project list");

        let transport = boinc_rpc::Transport::new(host.addr, host.password);
        let mut client = boinc_rpc::Client::new(transport);

        let client_projects = match client.get_projects().await {
            Ok(t) => Some(t),
            Err(t) => panic!(t),
        };

        if let Some(projects) = client_projects.clone() {
            for (_, project) in projects.into_iter().enumerate() {
                let my_proj = project.clone();
                project_list.insert(project.url, my_proj);
            }
        }
    }
}

async fn update_tasks_for_host(mut host : &mut Host)
{
    println!("Fetching tasks for {:?}", host.addr);

    let transport = boinc_rpc::Transport::new(host.addr, host.password);
    let mut client = boinc_rpc::Client::new(transport);

    let client_tasks = match client.get_results(false).await {
        Ok(t) => t,
        Err(t) => panic!(t),
    };

    host.results = Some(client_tasks);
}

pub fn add_columns(treeview: &gtk::TreeView) {
    let mut columns: Vec<gtk::TreeViewColumn> = Vec::new();

    append_column("Host", &mut columns, &treeview, None);
    append_column("Project", &mut columns, &treeview, None);
    append_column("Name", &mut columns, &treeview, None);
    append_column("Progress %", &mut columns, &treeview, None);
    append_column("Elapsed Time", &mut columns, &treeview, None);
    append_column("Time Left", &mut columns, &treeview, None);
    append_column("State", &mut columns, &treeview, None);
    append_column("Report Deadline", &mut columns, &treeview, None);
    append_column("Received Time", &mut columns, &treeview, None);
    append_column("Completed Time", &mut columns, &treeview, None);
    append_column("Platform", &mut columns, &treeview, None);
    // There are four additional columns here with float values
    // to allow the sorting columns below

    // Reminder, the below are zero-indexed
    // Clicking the "Progress %" column the order should be defined by the float value
    columns[3].set_sort_column_id(11);
    // Clicking the "Elapsed Time" column the order should be defined by the float value
    columns[4].set_sort_column_id(12);
    // Clicking the "Time Left" column the order should be defined by the float value
    columns[5].set_sort_column_id(13);
}

fn append_column(
    title: &str,
    v: &mut Vec<gtk::TreeViewColumn>,
    treeview: &gtk::TreeView,
    max_width: Option<i32>,
) -> i32 {
    let id = v.len() as i32;
    let renderer = gtk::CellRendererText::new();

    let column = gtk::TreeViewColumn::new();
    column.set_title(title);
    column.set_resizable(true);
    if let Some(max_width) = max_width {
        column.set_max_width(max_width);
        column.set_expand(true);
    }
    column.set_min_width(10);
    column.pack_start(&renderer, true);
    column.add_attribute(&renderer, "text", id);
    column.set_clickable(true);
    column.set_sort_column_id(id);
    column.set_resizable(true);
    treeview.append_column(&column);
    v.push(column);

    return id;
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

fn build_ui(application: &gtk::Application) {
    let model = Rc::new(RefCell::new(create_model()));
    let mut project_list : HashMap<Option<String>, ProjectInfo> = HashMap::new();

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

    add_columns(&treeview);

    // get_data_for_model(&model.borrow());

    // Every 5 seconds, we'll update the data
    Some(gtk::timeout_add(
        5000,
        move || {
            get_data_for_model(&model.borrow(), &mut project_list);

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

    application.connect_activate(move |_| {
        window.show_all();
        // filter_entry.hide();
        window.present();
    });
}

fn get_data_for_model(store : &gtk::ListStore, mut project_list : &mut HashMap<Option<String>, ProjectInfo>) {
    dbg!(&store);
    store.clear();

    let col_indices: [u32; 14] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13];
    let hosts = get_necessary_data_from_hosts(&mut project_list);

    for host in hosts {
        if let Some(results) = host.results {
            for (_, result) in results.into_iter().enumerate() {
                let values: [&dyn ToValue; 14] = [
                    &host.addr,
                    &project_list.get(&result.project_url).unwrap().name,
                    &result.name,
                    &format!("{0:.2} %", result.progress()),
                    &result.elapsed_as_string(),
                    &result.remaining_as_string(),
                    &result.state(),
                    &result.report_deadline.unwrap(),
                    &result.received_time.unwrap(),
                    &result.completed_time.unwrap_or(0.0),
                    &result.platform,
                    &result.progress(),
                    &result.elapsed(),
                    &result.remaining(),
                ];

                store.set(&store.append(), &col_indices, &values);
            }
        }
    }
}
