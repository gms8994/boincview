use gtk::prelude::*;
use gtk::{Application, ApplicationWindow};
use std::rc::Rc;
use std::cell::RefCell;

mod lib;
use crate::lib::Host;

const APPLICATION_NAME: &str = "com.github.gtk-rs.examples.basic";

fn main() {
    let app = Application::builder().application_id(APPLICATION_NAME).build();
    app.connect_activate(build_ui);
    app.run();
}

// fn get_necessary_data_from_hosts(host_list : &mut Vec<Host>, project_list : &mut HashMap<Option<String>, ProjectInfo>)
// {
//     // Fetch all of the tasks, then fetch the projects
//     tokio::runtime::Runtime::new().unwrap().block_on(async {
//         update_task_list(&mut host_list).await;
//     });
// 
//     tokio::runtime::Runtime::new().unwrap().block_on(async {
//         update_projects_list(&host_list, &mut project_list).await;
//     });
// }

// async fn update_projects_list(hosts : &Vec<Host>, project_list : &mut HashMap<Option<String>, ProjectInfo>)
// {
//     for mut host in hosts.iter() {
//         // Here we want to check to see if we need to fetch
//         // the projects - at this point, we should have the
//         // list of tasks, and so we can check to see if any
//         // of the tasks *don't* have an entry in the projects
//         // list - if so, we need to fetch the proejcts list
//         // from the host
// 
//         let mut has_missing_projects : bool = false;
// 
//         if let Some(results) = host.results.clone() {
//             for (_, result) in results.into_iter().enumerate() {
//                 if ! project_list.contains_key(&result.project_url) {
//                     has_missing_projects = true;
//                 }
//             }
//         }
// 
//         if ! has_missing_projects {
//             return;
//         }
// 
//         println!("Some projects are missing - we need to fetch project list");
// 
//         let mut client = host.connect();
// 
//         let client_projects = match client.get_projects().await {
//             Ok(t) => Some(t),
//             Err(t) => panic!(t),
//         };
// 
//         if let Some(projects) = client_projects.clone() {
//             for (_, project) in projects.into_iter().enumerate() {
//                 let my_proj = project.clone();
//                 project_list.insert(project.url, my_proj);
//             }
//         }
//     }
// }
// 
// async fn update_task_list(hosts : &mut Vec<Host>)
// {
//     for (_idx, host) in hosts.into_iter().enumerate() {
//         println!("Fetching tasks for {:?}", host.addr);
// 
//         let mut client = host.connect();
// 
//         let client_tasks = match client.get_results(false).await {
//             Ok(t) => t,
//             Err(t) => panic!(t),
//         };
// 
//         host.results = Some(client_tasks);
//     }
// }

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

fn build_ui(app: &Application) {
    // let mut project_list : HashMap<Option<String>, ProjectInfo> = HashMap::new();
    let mut host_list = Vec::new();

    host_list.push(Host::new("127.0.0.1:31416", Some("1033644eaad1ea7d91bc48a749f1620b")));
    // hosts.push(Host::new("192.168.1.108:31416", Some("5e09d64108b3871cae6ef4bd0c599c69")));
    host_list.push(Host::new("192.168.1.113:31416", Some("95405a40b449164295bba46fa405cc1b")));

    let window = ApplicationWindow::builder()
        .application(app)
        .title("My GTK App")
        .build();

    window.set_title("BOINCView");
    window.set_default_size(1024, 768);

    let paned_window = gtk::Paned::new(gtk::Orientation::Horizontal);
    let host_frame = gtk::Frame::new(None);
    let data_frame = gtk::Frame::new(None);

    host_frame.set_shadow_type(gtk::ShadowType::In);
    data_frame.set_shadow_type(gtk::ShadowType::In);

    host_frame.set_size_request(200, -1);
    data_frame.set_size_request(568, -1);

    let vbox = gtk::Box::new(gtk::Orientation::Vertical, 8);

    let scrolled_window = gtk::ScrolledWindow::new(None::<&gtk::Adjustment>, None::<&gtk::Adjustment>);
    scrolled_window.set_shadow_type(gtk::ShadowType::EtchedIn);
    scrolled_window.set_policy(gtk::PolicyType::Automatic, gtk::PolicyType::Automatic);

    let model = Rc::new(RefCell::new(
        gtk::ListStore::new(&[
            String::static_type()
        ])
    ));
    let treeview = gtk::TreeView::with_model(&*model.borrow());

    treeview.set_vexpand(true);

    scrolled_window.add(&treeview);

    add_columns(&treeview);

    // Every 5 seconds, we'll update the data
    glib::timeout_add_seconds_local(5, move || {
        glib::MainContext::default().spawn_local(get_data_for_model(&model.borrow(), &mut host_list));

        glib::Continue(true)
    });
 
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

    vbox.add(&scrolled_window);
    data_frame.add(&vbox);

    paned_window.pack1(&host_frame, true, true);
    paned_window.pack2(&data_frame, true, true);

    window.add(&paned_window);
    window.show_all();
}

fn create_model() -> gtk::ListStore {
    return gtk::ListStore::new(&[
       String::static_type()
    ]);
}

async fn get_data_for_model(store : &gtk::ListStore, host_list : &mut Vec<Host>) {
    store.clear();

    for _host in host_list.iter() {
        let entries = &["Michel", "Sara", "Liam", "Zelda", "Neo", "Octopus master"];
        for (_i, entry) in entries.iter().enumerate() {
            store.insert_with_values(None, &[(0, &entry)]);
        }
    }
}
