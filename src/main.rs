use cascade::cascade;
use std::path::PathBuf;
use fuzzy_matcher::FuzzyMatcher;


use gtk::{CustomFilter, CustomSorter, FilterListModel, gio, IconSize, Image, SearchBar, SortListModel, Widget};

use gtk::{
    Application, ApplicationWindow, Label, ListView, PolicyType, ScrolledWindow,
    SignalListItemFactory, SingleSelection,
};

use gtk::gio::{DesktopAppInfo};
use glib::Object;
use gtk::glib;
use gtk::prelude::*;
use glib::clone;
use crate::entry::EntryObject;
use crate::gio::ListStore;
use fuzzy_matcher::skim::SkimMatcherV2;

mod entry;


// TODO : Css - https://github.com/gtk-rs/gtk4-rs/blob/master/book/listings/css/1/main.rs
// TODO : Focus
// TODO : Remove duplicates


fn main() {
    // Create a new application
    let app = Application::builder()
        .application_id("org.gtk-rs.example")
        .build();

    app.connect_activate(build_ui);

    // Run the application
    app.run();
}

fn get_desktop_entries(store: &ListStore) {
    let path = PathBuf::from("/usr/share/applications");
    destkop_entries_from_path(path, store);

    if let Some(data_dir) = dirs::data_dir() {
        destkop_entries_from_path(data_dir, store);
    }

    if let Some(data_dir) = dirs::data_local_dir() {
        destkop_entries_from_path(data_dir, store);
    }

    if let Some(desktop) = dirs::desktop_dir() {
        destkop_entries_from_path(desktop, store);
    }
}

fn destkop_entries_from_path(path: PathBuf, store: &ListStore) {
    for entry in path.read_dir().expect("Failed to open_dir") {
        let entry = entry.expect("Failed to read desktop entry");

        let is_desktop_entry = entry
            .path()
            .extension()
            .map(|ext| ext == "desktop")
            .unwrap_or(false);

        if is_desktop_entry {
            let widget = DesktopAppInfo::from_filename(entry.path());
            if let Some(entry) = widget {
                store.append(&EntryObject::from(entry));
            }
        }
    }
}

fn build_ui(app: &Application) {
    let model = ListStore::new(EntryObject::static_type());
    let factory = make_factory();
    let filter = CustomFilter::new(filter_fn("".into()));
    let filter_model = FilterListModel::new(Some(&model), Some(&filter));
    let sorter = make_sorter();
    let sort_model = SortListModel::new(Some(&filter_model), Some(&sorter));
    let selection_model = SingleSelection::new(Some(&sort_model));
    let list_view = ListView::new(Some(&selection_model), Some(&factory));


    list_view.connect_activate(|list_view, position| {
        let model = list_view.model().expect("The model has to exist.");
        let entry = model
            .item(position)
            .expect("The item has to exist.")
            .downcast::<EntryObject>()
            .expect("The item has to be an `EntryObject`.");

        entry.launch();
    });


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
    get_desktop_entries(&model);

}

fn filter_fn(term: String) -> impl Fn(&Object) -> bool {
    move |obj| {
        if term.is_empty() {
            return true;
        }

        let entry = obj
            .downcast_ref::<EntryObject>()
            .expect("The object needs to be of type `EntryObject`.");

        let name = entry.property::<String>("name");
        let matcher = SkimMatcherV2::default().ignore_case();
        let score = matcher.fuzzy_match(name.as_str(), term.as_str()).unwrap_or(0);
        entry.set_property("score", score);
        score.is_positive()
    }
}

fn make_sorter() -> CustomSorter {
    CustomSorter::new(move |obj1, obj2| {
        let entry_1 = obj1
            .downcast_ref::<EntryObject>()
            .expect("The object needs to be of type `EntryObject`.");

        let entry_2 = obj2
            .downcast_ref::<EntryObject>()
            .expect("The object needs to be of type `EntryObject`.");

        let score_1 = entry_1.property::<i64>("score");
        let score_2 = entry_2.property::<i64>("score");

        score_2.cmp(&score_1).into()
    })
}

fn make_factory() -> SignalListItemFactory {
    let factory = SignalListItemFactory::new();

    factory.connect_setup(move |_, list_item| {
        let image = Image::builder()
            .icon_size(IconSize::Large)
            .use_fallback(true)
            .build();

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
