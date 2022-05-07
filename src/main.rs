use closure::closure;
use cascade::cascade;
use std::path::PathBuf;


use gtk::{CustomFilter, CustomSorter, FilterChange, FilterListModel, gio, Image, SearchBar, SorterChange, SortListModel, Widget};

use gtk::{
    Application, ApplicationWindow, Label, ListView, PolicyType, ScrolledWindow,
    SignalListItemFactory, SingleSelection,
};

use gtk::gio::{DesktopAppInfo};
use glib::Object;
use gtk::glib;
use gtk::prelude::*;
use glib::clone;
use gtk::gdk::AppLaunchContext;

mod imp;

glib::wrapper! {
    pub struct EntryObject(ObjectSubclass<imp::EntryObject>);
}

impl EntryObject {
    pub fn new(name: &str, icon: &str) -> Self {
        Object::new(&[("name", &name), ("icon", &icon)]).expect("Failed to create `IntegerObject`.")
    }
}

impl From<DesktopAppInfo> for EntryObject {
    fn from(entry: DesktopAppInfo) -> Self {
        let name = entry.name();
        let icon = entry.icon();
        let filename = entry.filename()
            .and_then(|p| p.to_str().map(ToString::to_string));

        Object::new(&[("name", &name.as_str()), ("icon", &icon), ("filename", &filename)]).expect("Failed to create `IntegerObject`.")
    }
}

fn main() {
    // Create a new application
    let app = Application::builder()
        .application_id("org.gtk-rs.example")
        .build();

    app.connect_activate(build_ui);
    app.set_accels_for_action("win.close", &["<Ctrl>W"]);    // Connect to "activate" signal of `app`

    // Run the application
    app.run();
}

fn get_desktop_entries() -> Vec<EntryObject> {
    let apps = PathBuf::from("/usr/share/applications");
    let mut entries = vec![];
    for entry in apps.read_dir().expect("Failed to open_dir") {
        let entry = entry.expect("Failed to read desktop entry");

        let is_desktop_entry = entry
            .path()
            .extension()
            .map(|ext| ext == "desktop")
            .unwrap_or(false);

        if is_desktop_entry {
            let widget = DesktopAppInfo::from_filename(entry.path());
            if let Some(entry) = widget {
                entries.push(EntryObject::from(entry));
            }
        }
    }

    entries
}

fn build_ui(app: &Application) {
    let entries = get_desktop_entries();
    let model = gio::ListStore::new(EntryObject::static_type());
    model.splice(0, 0, &entries);

    let factory = make_factory();


    let filter = CustomFilter::new(filter_fn("Intel".into()));
    let filter_model = FilterListModel::new(Some(&model), Some(&filter));
    let sorter = make_sorter();
    let sort_model = SortListModel::new(Some(&filter_model), Some(&sorter));
    let selection_model = SingleSelection::new(Some(&sort_model));
    let list_view = ListView::new(Some(&selection_model), Some(&factory));


    list_view.connect_activate(closure!(clone filter, |list_view, position| {
        let model = list_view.model().expect("The model has to exist.");
        let entry = model
            .item(position)
            .expect("The item has to exist.")
            .downcast::<EntryObject>()
            .expect("The item has to be an `EntryObject`.");

        let filename = entry.property::<String>("filename");

        DesktopAppInfo::from_filename(&filename)
            .unwrap()
            .launch_uris(&[], AppLaunchContext::NONE)
            .expect("failed to launch application");

        filter.changed(FilterChange::Different);
        sorter.changed(SorterChange::Different);
    }));


    let scrolled_window = ScrolledWindow::builder()
        .hscrollbar_policy(PolicyType::Never) // Disable horizontal scrolling
        .valign(gtk::Align::Fill)
        .vexpand(true)
        .min_content_width(360)
        .child(&list_view)
        .build();

    let container = gtk::Box::new(gtk::Orientation::Vertical, 1);

    let window = ApplicationWindow::builder()
        .application(app)
        .title("My GTK App")
        .default_width(600)
        .default_height(300)
        .child(&container)
        .build();

    let entry = cascade! {
            gtk::SearchEntry::new();
            ..set_hexpand(true);
            ..connect_search_changed(clone!(@weak filter => move |entry| {
                filter.set_filter_func(filter_fn(entry.text().to_string()));
            }));
    };

    let search_bar = cascade! {
        SearchBar::new();
        ..set_valign(gtk::Align::Start);
        ..set_vexpand(false);
        ..set_key_capture_widget(Some(&window));
        ..set_child(Some(&entry));
    };


    container.append(&search_bar);
    container.append(&scrolled_window);

    window.set_child(Some(&container));

    window.present();
}

fn filter_fn(term: String) -> impl Fn(&Object) -> bool {
    move |obj| {
        // Get `IntegerObject` from `glib::Object`
        let entry = obj
            .downcast_ref::<EntryObject>()
            .expect("The object needs to be of type `EntryObject`.");

        // Get property "number" from `IntegerObject`
        let name = entry.property::<String>("name");
        name.contains(term.as_str())
    }
}

fn make_sorter() -> CustomSorter {
    CustomSorter::new(move |obj1, obj2| {
        // Get `IntegerObject` from `glib::Object`
        let entry_1 = obj1
            .downcast_ref::<EntryObject>()
            .expect("The object needs to be of type `IntegerObject`.");

        let entry_2 = obj2
            .downcast_ref::<EntryObject>()
            .expect("The object needs to be of type `IntegerObject`.");

        // Get property "number" from `IntegerObject`
        let name_1 = entry_1.property::<String>("name");
        let name_2 = entry_2.property::<String>("name");

        // Reverse sorting order -> large numbers come first
        name_2.cmp(&name_1).into()
    })
}

fn make_factory() -> SignalListItemFactory {
    let factory = SignalListItemFactory::new();

    factory.connect_setup(move |_, list_item| {
        let image = Image::default();
        let entry = gtk::Box::default();
        let label = Label::default();

        list_item
            .property_expression("item")
            .chain_property::<EntryObject>("icon")
            .bind(&image, "gicon", Widget::NONE);

        list_item
            .property_expression("item")
            .chain_property::<EntryObject>("name")
            .bind(&label, "label", Widget::NONE);

        entry.append(&image);
        entry.append(&label);

        list_item.set_child(Some(&entry));
    });

    factory
}
