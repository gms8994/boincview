extern crate gtk;

use gtk::prelude::*;

pub fn add_data_columns(treeview: &gtk::TreeView) {
    let mut columns: Vec<gtk::TreeViewColumn> = Vec::new();

    /* 0 */ append_column("Host", &mut columns, &treeview, None, None);
    /* 1 */ append_column("Project", &mut columns, &treeview, None, None);
    /* 2 */ append_column("Name", &mut columns, &treeview, None, None);
    /* 3 */ append_column("Progress %", &mut columns, &treeview, None, Some(true));
    /* 4 */ append_column("Elapsed Time", &mut columns, &treeview, None, None);
    /* 5 */ append_column("Time Left", &mut columns, &treeview, None, None);
    /* 6 */ append_column("State", &mut columns, &treeview, None, None);
    /* 7 */ append_column("Report Deadline", &mut columns, &treeview, None, None);
    /* 8 */ append_column("Received Time", &mut columns, &treeview, None, None);
    /* 9 */ append_column("Completed Time", &mut columns, &treeview, None, None);
    /* 10 */ append_column("Platform", &mut columns, &treeview, None, None);
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
    _as_progress: Option<bool>,
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
