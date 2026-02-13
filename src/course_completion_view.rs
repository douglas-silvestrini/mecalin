use gtk::subclass::prelude::*;
use libadwaita as adw;
use libadwaita::subclass::prelude::*;

mod imp {
    use super::*;

    #[derive(Default, gtk::CompositeTemplate)]
    #[template(resource = "/io/github/nacho/mecalin/ui/course_completion_view.ui")]
    pub struct CourseCompletionView {}

    #[glib::object_subclass]
    impl ObjectSubclass for CourseCompletionView {
        const NAME: &'static str = "CourseCompletionView";
        type Type = super::CourseCompletionView;
        type ParentType = adw::NavigationPage;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for CourseCompletionView {}
    impl WidgetImpl for CourseCompletionView {}
    impl NavigationPageImpl for CourseCompletionView {}
}

glib::wrapper! {
    pub struct CourseCompletionView(ObjectSubclass<imp::CourseCompletionView>)
        @extends adw::NavigationPage, gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

impl CourseCompletionView {
    pub fn new() -> Self {
        glib::Object::new()
    }
}

impl Default for CourseCompletionView {
    fn default() -> Self {
        Self::new()
    }
}
