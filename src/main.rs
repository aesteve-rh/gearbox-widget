// SPDX-FileCopyrightText: Red Hat, Inc.
// SPDX-License-Identifier: GPL-2.0-or-later

mod gearbox;

use gtk::prelude::*;
use gtk::{gdk, gio, glib};
use gvdb_macros::include_gresource_from_xml;

const CSS: &str = "
#selected-gear-label {
    font-size: 16px;
    color: #333;
}

#gear-scale {
    background-color: #4CAF50;
    color: white;
    border: none;
    padding: 10px;
    text-align: center;
    text-decoration: none;
    display: inline-block;
    font-size: 16px;
    margin: 4px 2px;
    cursor: pointer;
    border-radius: 4px;
}
";

fn on_activate(application: &gtk::Application) {
    //gtk::Scale::with_range(orientation, min, max, step)
    //let vbox = gtk::Box::new(gtk::Orientation::Horizontal, 10);
    let gearbox = gearbox::GearboxWidget::new(); //(0.0, 3.0, 1.0);
    //let gear_label = gtk::Label::new(Some("G"));
    //vbox.append(&gearbox);
    //vbox.append(&gear_label);
    //vbox.set_valign(gtk::Align::Center);
    //gearbox.set_orientation(gtk::Orientation::Vertical);
    //gearbox.set_digits(4);
    let window = gtk::ApplicationWindow::builder()
        .application(application)
        .default_width(360)
        .default_height(400)
        .decorated(false)
        .resizable(false)
        .title("Automatic Gearbox")
        .child(&gearbox)
        .build();
    //let rotary = gearbox::GearboxScale::default();
    //window.set_child(Some(&rotary));

    window.present();
}

fn on_startup(app: &gtk::Application) {
    let provider = gtk::CssProvider::new();
    //provider.load_from_data(include_str!("../data/style.css"));
    //provider.load_from_data(CSS);
    gtk::style_context_add_provider_for_display(
        &gdk::Display::default().unwrap(),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );

    let quit_action = gio::ActionEntry::builder("quit")
        .activate(move |app: &gtk::Application, _, _| app.quit())
        .build();
    app.add_action_entries([quit_action]);
    app.set_accels_for_action("app.quit", &["<primary>q"]);
}

//static GRESOURCE_BYTES: &[u8] = include_gresource_from_xml!("data/rotary.gresource.xml");

fn main() -> glib::ExitCode {
    let application = gtk::Application::builder()
        .application_id("com.alesgar.gearbox")
        .build();
    application.connect_activate(on_activate);
    //application.connect_startup(on_startup);
    application.run()
}
