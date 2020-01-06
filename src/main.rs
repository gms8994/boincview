extern crate boinc_rpc as rpc;
extern crate encoding;
extern crate gio;
extern crate glib;
extern crate gtk;
extern crate ini;

pub mod client;
pub mod tasks;
pub mod ui;
pub mod data_fetcher;

use std::rc::Rc;
use std::cell::RefCell;
use gio::prelude::*;
use gtk::prelude::*;
use ini::Ini;
use std::collections::HashMap;
use std::str::FromStr;

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

    let model = Rc::new(RefCell::new(data_fetcher::create_model()));

    let window = gtk::ApplicationWindow::new(application);
    window.set_title("BOINCView");
    window.set_default_size(1024, 768);

    let paned_window = gtk::Paned::new(gtk::Orientation::Horizontal);

    // Set up both of the panes in the window
    let (paned_window, host) = ui::Window::new(paned_window, true, 200);
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

    data_fetcher::get_data_for_model(&model.borrow(), &mut clients, &mut downed_clients);
    Some(gtk::timeout_add(
        30000,
        move || {
            data_fetcher::get_data_for_model(&model.borrow(), &mut clients, &mut downed_clients);

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
