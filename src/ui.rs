extern crate gtk;
extern crate glib;

use gio::prelude::*;
use gtk::prelude::*;

pub fn add_columns(treeview: &gtk::TreeView) {
    let mut columns: Vec<gtk::TreeViewColumn> = Vec::new();

    append_column("Host", &mut columns, &treeview, None);
    append_column("Project", &mut columns, &treeview, None);
    append_column("Name", &mut columns, &treeview, None);
    append_column("Progress %", &mut columns, &treeview, None);
    append_column("Elapsed Time", &mut columns, &treeview, None);
    append_column("Time Left", &mut columns, &treeview, None);
    append_column("Exit Status", &mut columns, &treeview, None);
    append_column("State", &mut columns, &treeview, None);
    append_column("Report Deadline", &mut columns, &treeview, None);
    append_column("Received Time", &mut columns, &treeview, None);
    append_column("Completed Time", &mut columns, &treeview, None);
    append_column("Platform", &mut columns, &treeview, None);

    // Clicking the "Progress %" column the order should be defined by the float value
    // reminder that both of these are zero indexed
    columns[3].set_sort_column_id(12);
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
