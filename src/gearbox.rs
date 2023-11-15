// SPDX-FileCopyrightText: Red Hat, Inc.
// SPDX-License-Identifier: GPL-3.0-or-later

use gtk::{glib, prelude::*, subclass::prelude::*};
use log::warn;
use num_enum::TryFromPrimitive;
use vhal_emulator as ve;

const START_YPOS: f64 = 30.0;
const END_YPOS: f64 = 200.0;
const SCALE_XPOS: f64 = 50.0;
const LABEL_XPOS: f64 = 95.0;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, TryFromPrimitive)]
#[repr(u32)]
pub enum VehicleGear {
    Park,
    Reverse,
    Neutral,
    Drive,
}

impl VehicleGear {
    fn to_vhal(self) -> ve::vhal_consts_2_0::VehicleGear {
        use ve::vhal_consts_2_0::VehicleGear::*;
        match self {
            Self::Park => GEAR_PARK,
            Self::Reverse => GEAR_REVERSE,
            Self::Neutral => GEAR_NEUTRAL,
            Self::Drive => GEAR_DRIVE,
        }
    }
}

mod imp {
    use std::cell::OnceCell;

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
                  <property name="height-request">200</property>
                  <signal name="value-changed" handler="on_gear_change" swapped="true" />
              </object>
            </child>
            <child>
              <object class="GtkLabel" id="label_park">
                <property name="label">- P</property>
              </object>
            </child>
            <child>
              <object class="GtkLabel" id="label_reverse">
                <property name="label">- R</property>
              </object>
            </child>
            <child>
              <object class="GtkLabel" id="label_neutral">
                <property name="label">- N</property>
              </object>
            </child>
            <child>
              <object class="GtkLabel" id="label_drive">
                <property name="label">- D</property>
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
        #[template_child]
        pub(super) label_park: TemplateChild<gtk::Label>,
        #[template_child]
        pub(super) label_reverse: TemplateChild<gtk::Label>,
        #[template_child]
        pub(super) label_neutral: TemplateChild<gtk::Label>,
        #[template_child]
        pub(super) label_drive: TemplateChild<gtk::Label>,
        pub(super) vhal: OnceCell<ve::Vhal>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for GearboxWidget {
        const NAME: &'static str = "GearboxWidget";
        type Type = super::GearboxWidget;
        type ParentType = gtk::Widget;

        fn class_init(klass: &mut Self::Class) {
            klass.set_accessible_role(gtk::AccessibleRole::Button);
            klass.bind_template();
            klass.bind_template_instance_callbacks();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for GearboxWidget {
        fn constructed(&self) {
            self.parent_constructed();
            let adjustment = gtk::Adjustment::builder().lower(0.0).upper(3.0).build();
            self.scale.set_adjustment(&adjustment);
            self.fixed.move_(&*self.scale, SCALE_XPOS, START_YPOS);
            self.fixed
                .move_(&*self.label_park, LABEL_XPOS, START_YPOS - 5.0);
            self.fixed
                .move_(&*self.label_reverse, LABEL_XPOS, START_YPOS + 55.0);
            self.fixed
                .move_(&*self.label_neutral, LABEL_XPOS, START_YPOS + 115.0);
            self.fixed
                .move_(&*self.label_drive, LABEL_XPOS, END_YPOS + 5.0);
        }

        fn dispose(&self) {
            self.fixed.unparent();
        }
    }

    impl WidgetImpl for GearboxWidget {
        fn measure(&self, orientation: gtk::Orientation, for_size: i32) -> (i32, i32, i32, i32) {
            self.fixed.measure(orientation, for_size)
        }

        fn size_allocate(&self, width: i32, height: i32, baseline: i32) {
            self.fixed
                .size_allocate(&gtk::Allocation::new(10, 10, width, height), baseline);
        }
    }
}

glib::wrapper! {
    pub struct GearboxWidget(ObjectSubclass<imp::GearboxWidget>)
        @extends gtk::Widget;
}

impl Default for GearboxWidget {
    fn default() -> Self {
        glib::Object::new::<Self>().init_vhal()
    }
}

#[gtk::template_callbacks]
impl GearboxWidget {
    pub fn init_vhal(self) -> Self {
        let vhal = ve::Vhal::new(ve::adb_port_forwarding().unwrap()).unwrap();
        self.imp().vhal.set(vhal).unwrap();
        self
    }

    fn vhal(&self) -> &ve::Vhal {
        self.imp().vhal.get().unwrap()
    }

    #[template_callback]
    async fn on_gear_change(&self, scale: &gtk::Scale) {
        if let Ok(gear) = VehicleGear::try_from(scale.value() as u32) {
            self.vhal().set_gear_selection(gear.to_vhal()).unwrap();
            if !self.vhal().recv_cmd().is_ok_and(|cmd| {
                cmd.has_status() && cmd.status() == ve::VehicleHalProto::Status::RESULT_OK
            }) {
                warn!("Gear selection message failed");
            }
        }
    }
}
