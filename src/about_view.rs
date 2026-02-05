use gtk::glib;
use gtk::subclass::prelude::*;
use libadwaita as adw;
use libadwaita::prelude::ActionRowExt;
use libadwaita::subclass::prelude::*;

mod imp {
    use super::*;

    #[derive(Default, gtk::CompositeTemplate)]
    #[template(resource = "/io/github/nacho/mecalin/ui/about_view.ui")]
    pub struct AboutView {
        #[template_child]
        pub version_row: gtk::TemplateChild<adw::ActionRow>,
        #[template_child]
        pub app_icon: gtk::TemplateChild<gtk::Image>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for AboutView {
        const NAME: &'static str = "MecalinAboutView";
        type Type = super::AboutView;
        type ParentType = adw::PreferencesPage;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
            klass.bind_template_callbacks();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    #[gtk::template_callbacks]
    impl AboutView {
        #[template_callback]
        fn open_uri(&self, row: &adw::ActionRow) {
            if let Some(subtitle) = row.subtitle() {
                let uri = subtitle.as_str();
                if uri.starts_with("http://") || uri.starts_with("https://") {
                    gtk::UriLauncher::new(uri).launch(
                        gtk::Window::NONE,
                        gtk::gio::Cancellable::NONE,
                        |_| {},
                    );
                }
            }
        }
    }

    impl ObjectImpl for AboutView {
        fn constructed(&self) {
            self.parent_constructed();
            let version = format!("Version {}", crate::config::VERSION);
            self.version_row.set_subtitle(&version);
            self.app_icon
                .set_icon_name(Some(crate::config::APPLICATION_ID));
        }
    }

    impl WidgetImpl for AboutView {}
    impl PreferencesPageImpl for AboutView {}
}

glib::wrapper! {
    pub struct AboutView(ObjectSubclass<imp::AboutView>)
        @extends gtk::Widget, adw::PreferencesPage;
}

impl AboutView {
    pub fn new() -> Self {
        glib::Object::new()
    }
}

impl Default for AboutView {
    fn default() -> Self {
        Self::new()
    }
}
