//use gettextrs::gettext;
use gtk::{gdk, glib, graphene, gsk, pango, prelude::*, subclass::prelude::*};
use glib::Object;

const WIDTH: f32 = 360.0;
const HEIGHT: f32 = 360.0;


#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum VehicleGear {
    Park,
    Reverse,
    Neutral,
    Drive,
}

mod imp {
    use std::cell::{Cell, OnceCell, RefCell};

    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate)]
    #[template(string = r#"
    <interface>
      <template class="GearboxScale" parent="GtkScale">
        <property name="orientation">vertical</property>
        <property name="round-digits">0</property>
        <property name="show-fill-level">false</property>
        <property name="has-origin">false</property>
      </template>
    </interface>
    "#)]
    pub struct GearboxScale {
        pub(super) gear: OnceCell<VehicleGear>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for GearboxScale {
        const NAME: &'static str = "GearboxScale";
        type Type = gearbox_base::GearboxScale;
        type ParentType = gtk::Scale;

        fn class_init(klass: &mut Self::Class) {
            //klass.set_accessible_role(gtk::AccessibleRole::Scrollbar);
            klass.bind_template();
            //klass.bind_template_instance_callbacks();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for GearboxScale {}

    impl WidgetImpl for GearboxScale {}

    impl RangeImpl for GearboxScale {}

    impl ScaleImpl for GearboxScale {}


}

glib::wrapper! {
    pub struct GearboxScale(ObjectSubclass<imp::GearboxScale>)
        @extends gtk::Scale, gtk::Range, gtk::Widget,
        @implements gtk::Accessible, gtk::AccessibleRange, gtk::Buildable, gtk::ConstraintTarget, gtk::Orientable;
}

impl Default for GearboxScale {
    fn default() -> Self {
        glib::Object::new::<Self>()
    }
}

impl GearboxScale {
    pub fn new() -> Self {
        Object::builder().build()
    }

    pub fn with_range(min: f64, max: f64, step: f64) -> Self {
        let adjustment = gtk::Adjustment::default();
        adjustment.set_lower(min);
        adjustment.set_upper(max);
        adjustment.set_step_increment(step);
        Object::builder().property("adjustment", adjustment)
        .build()
    }
}
