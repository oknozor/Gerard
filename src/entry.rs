use crate::gio::subclass::prelude::ObjectSubclassExt;
use gtk::gdk::AppLaunchContext;
use gtk::gio::{
    glib::{self, Object},
    DesktopAppInfo,
};
use gtk::prelude::AppInfoExt;

glib::wrapper! {
    pub struct EntryObject(ObjectSubclass<imp::EntryObject>);
}

impl EntryObject {
    pub fn new(name: &str, icon: &str) -> Self {
        Object::new(&[("name", &name), ("icon", &icon)]).expect("Failed to create `IntegerObject`.")
    }

    pub fn launch(&self) {
        let entry = imp::EntryObject::from_instance(&self);
        let entry = &*entry.desktop_entry.borrow();

        entry
            .as_ref()
            .expect("desktop entry not found")
            .launch(&[], AppLaunchContext::NONE)
            .expect("failed to launch application");

        std::process::exit(0);
    }
}

impl From<DesktopAppInfo> for EntryObject {
    fn from(entry: DesktopAppInfo) -> Self {
        let name = entry.name();
        let icon = entry.icon();

        Object::new(&[
            ("name", &name.as_str()),
            ("icon", &icon),
            ("desktop-entry", &entry),
        ])
        .expect("Failed to create `IntegerObject`.")
    }
}

mod imp {
    use std::cell::{Cell, RefCell};

    use crate::gio;
    use crate::glib::ParamSpecObject;
    use glib::{ParamFlags, ParamSpec, Value};
    use gtk::gio::DesktopAppInfo;
    use gtk::glib;
    use gtk::glib::{ParamSpecInt64, ParamSpecString};
    use gtk::prelude::*;
    use gtk::subclass::prelude::*;
    use once_cell::sync::Lazy;

    #[derive(Default)]
    pub struct EntryObject {
        icon: RefCell<Option<gio::Icon>>,
        score: Cell<i64>,
        pub name: RefCell<String>,
        pub desktop_entry: RefCell<Option<DesktopAppInfo>>,
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
                    ParamSpecInt64::new(
                        "score",
                        "score",
                        "score",
                        i64::MIN,
                        i64::MAX,
                        0,
                        ParamFlags::READWRITE,
                    ),
                    ParamSpecString::new("name", "name", "name", None, ParamFlags::READWRITE),
                    ParamSpecObject::new(
                        "desktop-entry",
                        "desktop-entry",
                        "desktop-entry",
                        DesktopAppInfo::static_type(),
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

        fn set_property(&self, _obj: &Self::Type, _id: usize, value: &Value, pspec: &ParamSpec) {
            match pspec.name() {
                "score" => {
                    let score = value.get().expect("The value needs to be of type `i64`.");
                    self.score.replace(score);
                }
                "name" => {
                    let name = value
                        .get()
                        .expect("The value needs to be of type `String`.");
                    self.name.replace(name);
                }
                "icon" => {
                    let icon = value
                        .get()
                        .expect("The value needs to be of type `String`.");
                    self.icon.replace(icon);
                }
                "desktop-entry" => {
                    let desktop_entry = value
                        .get()
                        .expect("The value needs to be of type `DesktopAppInfo`.");
                    self.desktop_entry.replace(desktop_entry);
                }
                _ => unimplemented!(),
            }
        }

        fn property(&self, _obj: &Self::Type, _id: usize, pspec: &ParamSpec) -> Value {
            match pspec.name() {
                "score" => self.score.get().to_value(),
                "name" => self.name.borrow().to_value(),
                "icon" => self.icon.borrow().to_value(),
                "desktop-entry" => self.desktop_entry.borrow().to_value(),
                _ => unimplemented!(),
            }
        }
    }
}
