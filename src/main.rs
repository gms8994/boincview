extern crate encoding;
extern crate boinc_rpc as rpc;
extern crate gtk;
extern crate gio;
extern crate glib;
extern crate ini;

pub mod tasks;
pub mod client;

use client::*;
use gio::prelude::*;
use gtk::prelude::*;
use ini::Ini;
use std::rc::Rc;
use tasks::*;

#[repr(i32)]
enum Columns {
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
    let conf = match Ini::load_from_file("conf.ini") {
        Ok(config) => config,
        Err(error) => {
            panic!("Missing config file: {:?}", error)
        }
    };

    // Future work
    // for (sec, prop) in &conf {
    //     println!("Section: {:?}", sec);
    //     for (key, value) in prop {
    //         println!("{:?}:{:?}", key, value);
    //     }
    // }

    application.connect_activate(|app| {
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

        let model = Rc::new(create_model());
        let treeview = gtk::TreeView::new_with_model(&*model);
        treeview.set_vexpand(true);

        sw.add(&treeview);

        add_columns(&treeview);

        window.show_all();
    });

    application.run(&[]);
}

fn create_model() -> gtk::ListStore {
    let col_types: [glib::types::Type; 10] = [
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
    ];

    let mut client = rpc::SimpleClient::default();
    client.addr = std::net::SocketAddr::new(std::net::IpAddr::V4(std::net::Ipv4Addr::new(127, 0, 0, 1)), 31416);
    client.password = Some("".into());

    let tasks = client.tasks();
    let projects = client.projects();

    let store = gtk::ListStore::new(&col_types);

    let col_indices: [u32; 10] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];

    for (_, d) in tasks.iter().enumerate() {
        &d.time_left();

        let values: [&dyn ToValue; 10] = [
            &d.name,
            &d.platform,
            projects[&d.project_url].as_ref().unwrap(),
            &d.final_elapsed_time.unwrap(),
            &d.exit_status.unwrap(),
            &d.state(),
            &d.report_deadline.unwrap(),
            &d.received_time.unwrap(),
            &d.estimated_cpu_time_remaining.unwrap(),
            &d.completed_time.unwrap_or(0.0),
        ];

        store.set(&store.append(), &col_indices, &values);
    }

    return store;
}

fn add_columns(treeview: &gtk::TreeView) {
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
