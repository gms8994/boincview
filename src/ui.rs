extern crate gtk;
extern crate glib;

use gio::prelude::*;
use gtk::prelude::*;

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

pub fn add_columns(treeview: &gtk::TreeView) {
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
