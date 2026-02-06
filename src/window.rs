use gtk::prelude::*;
use gtk::subclass::prelude::*;
use libadwaita as adw;
use libadwaita::prelude::ActionRowExt;
use libadwaita::subclass::prelude::*;

use crate::about_view::AboutView;
use crate::falling_keys_game::FallingKeysGame;
use crate::lesson_view::LessonView;
use crate::preferences_view::PreferencesView;
use crate::scrolling_lanes_game::ScrollingLanesGame;
use crate::speed_test_view::SpeedTestView;
use crate::typing_row::TypingRow;

mod imp {
    use super::*;

    #[derive(Default, gtk::CompositeTemplate)]
    #[template(resource = "/io/github/nacho/mecalin/ui/window.ui")]
    pub struct MecalinWindow {
        #[template_child]
        pub header_bar: TemplateChild<adw::HeaderBar>,
        #[template_child]
        pub window_title: TemplateChild<adw::WindowTitle>,
        #[template_child]
        pub navigation_view: TemplateChild<adw::NavigationView>,
        #[template_child]
        pub lessons_row: TemplateChild<adw::ActionRow>,
        #[template_child]
        pub speed_test_row: TemplateChild<adw::ActionRow>,
        #[template_child]
        pub falling_keys_row: TemplateChild<adw::ActionRow>,
        #[template_child]
        pub scrolling_lanes_row: TemplateChild<adw::ActionRow>,
        #[template_child]
        pub preferences_row: TemplateChild<adw::ActionRow>,
        #[template_child]
        pub about_row: TemplateChild<adw::ActionRow>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for MecalinWindow {
        const NAME: &'static str = "MecalinWindow";
        type Type = super::MecalinWindow;
        type ParentType = adw::ApplicationWindow;

        fn class_init(klass: &mut Self::Class) {
            AboutView::ensure_type();
            LessonView::ensure_type();
            TypingRow::ensure_type();
            FallingKeysGame::ensure_type();
            ScrollingLanesGame::ensure_type();
            PreferencesView::ensure_type();
            SpeedTestView::ensure_type();
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for MecalinWindow {
        fn constructed(&self) {
            self.parent_constructed();
            self.setup_signals();
        }
    }
    impl WidgetImpl for MecalinWindow {}
    impl WindowImpl for MecalinWindow {}
    impl ApplicationWindowImpl for MecalinWindow {}
    impl AdwApplicationWindowImpl for MecalinWindow {}
}

glib::wrapper! {
    pub struct MecalinWindow(ObjectSubclass<imp::MecalinWindow>)
        @extends adw::ApplicationWindow, gtk::ApplicationWindow, gtk::Window, gtk::Widget,
        @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible, gtk::Buildable,
                    gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

impl MecalinWindow {
    pub fn new(app: &adw::Application) -> Self {
        glib::Object::builder().property("application", app).build()
    }

    pub fn show_lessons(&self) {
        let imp = self.imp();
        imp.navigation_view.push_by_tag("lessons");
    }

    pub fn show_game(&self) {
        let imp = self.imp();
        imp.navigation_view.push_by_tag("game");
    }

    pub fn show_lanes_game(&self) {
        let imp = self.imp();
        imp.navigation_view.push_by_tag("lanes_game");
    }

    pub fn show_speed_test(&self) {
        let imp = self.imp();
        imp.navigation_view.push_by_tag("speed_test");
    }

    pub fn show_about(&self) {
        let imp = self.imp();
        imp.navigation_view.push_by_tag("about");
    }

    pub fn show_preferences(&self) {
        let imp = self.imp();
        imp.navigation_view.push_by_tag("preferences");
    }

    pub fn set_title(&self, title: &str) {
        let imp = self.imp();
        imp.window_title.set_title(title);
    }

    pub fn set_subtitle(&self, subtitle: &str) {
        let imp = self.imp();
        imp.window_title.set_subtitle(subtitle);
    }

    pub fn load_window_state(&self) {
        let settings = gio::Settings::new("io.github.nacho.mecalin.state.window");

        let (width, height) = settings.get::<(i32, i32)>("size");
        self.set_default_size(width, height);

        if settings.boolean("maximized") {
            self.maximize();
        }

        self.connect_notify_local(Some("maximized"), move |window, _| {
            let settings = gio::Settings::new("io.github.nacho.mecalin.state.window");
            settings
                .set_boolean("maximized", window.is_maximized())
                .unwrap();
        });

        self.connect_notify_local(Some("default-width"), move |window, _| {
            let settings = gio::Settings::new("io.github.nacho.mecalin.state.window");
            if !window.is_maximized() {
                let size = (window.default_width(), window.default_height());
                settings.set("size", size).unwrap();
            }
        });

        self.connect_notify_local(Some("default-height"), move |window, _| {
            let settings = gio::Settings::new("io.github.nacho.mecalin.state.window");
            if !window.is_maximized() {
                let size = (window.default_width(), window.default_height());
                settings.set("size", size).unwrap();
            }
        });
    }
}

impl imp::MecalinWindow {
    fn setup_signals(&self) {
        let window = self.obj().downgrade();
        self.lessons_row.connect_activated(move |_| {
            if let Some(window) = window.upgrade() {
                window.show_lessons();
            }
        });

        let window = self.obj().downgrade();
        self.falling_keys_row.connect_activated(move |_| {
            if let Some(window) = window.upgrade() {
                window.show_game();
            }
        });

        let window = self.obj().downgrade();
        self.scrolling_lanes_row.connect_activated(move |_| {
            if let Some(window) = window.upgrade() {
                window.show_lanes_game();
            }
        });

        let window = self.obj().downgrade();
        self.speed_test_row.connect_activated(move |_| {
            if let Some(window) = window.upgrade() {
                window.show_speed_test();
            }
        });

        let window = self.obj().downgrade();
        self.about_row.connect_activated(move |_| {
            if let Some(window) = window.upgrade() {
                window.show_about();
            }
        });

        let window = self.obj().downgrade();
        self.preferences_row.connect_activated(move |_| {
            if let Some(window) = window.upgrade() {
                window.show_preferences();
            }
        });
    }
}
