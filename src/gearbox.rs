//use gettextrs::gettext;
use glib::Object;
use gtk::{gdk, glib, graphene, gsk, pango, prelude::*, subclass::prelude::*};

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
      <template class="GearboxWidget" parent="GtkWidget">
        <child>
          <object class="GtkFixed" id="fixed">
            <child>
              <object class="GtkScale" id="scale">
                  <property name="orientation">vertical</property>
                  <property name="round-digits">0</property>
                  <property name="show-fill-level">false</property>
                  <property name="has-origin">false</property>
              </object>
            </child>
          </object>
        </child>
      </template>
    </interface>
    "#)]
    pub struct GearboxWidget {
        #[template_child]
        pub(super) fixed: TemplateChild<gtk::Fixed>,
        #[template_child]
        pub(super) scale: TemplateChild<gtk::Scale>,
        pub(super) gear: OnceCell<VehicleGear>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for GearboxWidget {
        const NAME: &'static str = "GearboxWidget";
        type Type = super::GearboxWidget;
        type ParentType = gtk::Widget;

        fn class_init(klass: &mut Self::Class) {
            klass.set_accessible_role(gtk::AccessibleRole::Button);
            klass.bind_template();
            //klass.bind_template_instance_callbacks();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for GearboxWidget {
        fn constructed(&self) {
            self.parent_constructed();
            let adjustment = gtk::Adjustment::default();
            adjustment.set_lower(0.0);
            adjustment.set_upper(3.0);
            adjustment.set_step_increment(1.0);
            self.scale.set_adjustment(&adjustment);
            self.fixed.move_(&*self.scale, 10.0, 10.0);
        }
    }

    impl WidgetImpl for GearboxWidget {
        fn request_mode(&self) -> gtk::SizeRequestMode {
            gtk::SizeRequestMode::ConstantSize
        }

        fn measure(&self, _orientation: gtk::Orientation, _for_size: i32) -> (i32, i32, i32, i32) {
            (WIDTH as i32, HEIGHT as i32, -1, -1)
        }

        fn size_allocate(&self, width: i32, height: i32, baseline: i32) {
            //self.parent_size_allocate(width, height, baseline);
            self.fixed
                .size_allocate(&gtk::Allocation::new(0, 0, width, height), baseline);
            //self.scale
            //    .size_allocate(&gtk::Allocation::new(0, 0, width, height), baseline);
        }
    }
}

glib::wrapper! {
    pub struct GearboxWidget(ObjectSubclass<imp::GearboxWidget>)
        @extends gtk::Widget;
        //@implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

impl Default for GearboxWidget {
    fn default() -> Self {
        glib::Object::new::<Self>()
    }
}

impl GearboxWidget {
    pub fn new() -> Self {
        /*let fixed = gtk::Fixed::new();
        let adjustment = gtk::Adjustment::default();
        adjustment.set_lower(0.0);
        adjustment.set_upper(3.0);
        adjustment.set_step_increment(1.0);
        let scale = gtk::Scale::new(gtk::Orientation::Vertical, Some(&adjustment));
        scale.set_range(0.0, 3.0);
        scale.set_inverted(true); // Invert the scale for a more intuitive UI
        scale.set_digits(0); // Set the number of decimal places to zero
        fixed.put(&scale, 0.0, 0.0);*/
        Object::builder().build()
    }

    pub fn with_range(min: f64, max: f64, step: f64) -> Self {
        let adjustment = gtk::Adjustment::default();
        adjustment.set_lower(min);
        adjustment.set_upper(max);
        adjustment.set_step_increment(step);
        Object::builder().property("adjustment", adjustment).build()
    }
}
