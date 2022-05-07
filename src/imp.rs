use std::cell::{RefCell};

use glib::{ParamFlags, ParamSpec, Value};
use gtk::glib;
use gtk::glib::ParamSpecString;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use once_cell::sync::Lazy;
use crate::gio;
use crate::glib::ParamSpecObject;


#[derive(Default)]
pub struct EntryObject {
    name: RefCell<String>,
    icon: RefCell<Option<gio::Icon>>,
    filename: RefCell<Option<String>>,
}

#[glib::object_subclass]
impl ObjectSubclass for EntryObject {
    const NAME: &'static str = "MyGtkAppEntryObject";
    type Type = super::EntryObject;
}

// ANCHOR: object_impl
// Trait shared by all GObjects
impl ObjectImpl for EntryObject {
    fn properties() -> &'static [ParamSpec] {
        static PROPERTIES: Lazy<Vec<ParamSpec>> = Lazy::new(|| {
            vec![
                ParamSpecString::new(
                    "name",
                    "name",
                    "name",
                    None,
                    ParamFlags::READWRITE,
                ),
                ParamSpecString::new(
                    "filename",
                    "filename",
                    "filename",
                    None,
                    ParamFlags::READWRITE,
                ),
                ParamSpecObject::new(
                    "icon",
                    "icon",
                    "icon",
                    gio::Icon::static_type(),
                    ParamFlags::READWRITE,
                ),
            ]
        });
        PROPERTIES.as_ref()
    }

    fn set_property(
        &self,
        _obj: &Self::Type,
        _id: usize,
        value: &Value,
        pspec: &ParamSpec,
    ) {
        match pspec.name() {
            "name" => {
                let name = value.get().expect("The value needs to be of type `String`.");
                self.name.replace(name);
            }
            "icon" => {
                let icon = value.get().expect("The value needs to be of type `String`.");
                self.icon.replace(icon);
            }
            "filename" => {
                let filename = value.get().expect("The value needs to be of type `String`.");
                self.filename.replace(filename);
            }
            _ => unimplemented!(),
        }
    }

    fn property(&self, _obj: &Self::Type, _id: usize, pspec: &ParamSpec) -> Value {
        match pspec.name() {
            "name" => self.name.borrow().to_value(),
            "icon" => self.icon.borrow().to_value(),
            "filename" => self.filename.borrow().to_value(),
            _ => unimplemented!(),
        }
    }
}