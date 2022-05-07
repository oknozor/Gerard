use gtk::gdk::AppLaunchContext;
use gtk::gio::{DesktopAppInfo, glib::{self, Object}};
use gtk::prelude::AppInfoExt;
use crate::gio::subclass::prelude::ObjectSubclassExt;

glib::wrapper! {
    pub struct EntryObject(ObjectSubclass<imp::EntryObject>);
}

impl EntryObject {
    pub fn new(name: &str, icon: &str) -> Self {
        Object::new(&[("name", &name), ("icon", &icon)]).expect("Failed to create `IntegerObject`.")
    }

    pub fn launch(&self) {
        let entry = imp::EntryObject::from_instance(&self);
        let filename = &*entry.filename.borrow();

        let info = DesktopAppInfo::from_filename(&filename).unwrap();

        info.launch(&[], AppLaunchContext::NONE)
            .expect("failed to launch application");

        std::process::exit(0);
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

mod imp {
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
        icon: RefCell<Option<gio::Icon>>,
        pub name: RefCell<String>,
        pub filename: RefCell<String>,
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
}