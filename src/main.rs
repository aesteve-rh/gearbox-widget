// SPDX-FileCopyrightText: Red Hat, Inc.
// SPDX-License-Identifier: GPL-3.0-or-later

mod gearbox;

use gtk::{gdk, gio, glib, prelude::*};

const WIDTH: i32 = 180;
const HEIGHT: i32 = 280;

fn on_activate(application: &gtk::Application) {
    let gearbox = gearbox::GearboxWidget::default();
    let window = gtk::ApplicationWindow::builder()
        .application(application)
        .default_width(WIDTH)
        .default_height(HEIGHT)
        .decorated(false)
        .resizable(false)
        .title("Automatic Gearbox")
        .child(&gearbox)
        .build();

    window.present();
}

fn on_startup(app: &gtk::Application) {
    let provider = gtk::CssProvider::new();
    provider.load_from_string(include_str!("../data/style.css"));
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

fn main() -> glib::ExitCode {
    let application = gtk::Application::builder()
        .application_id("com.alesgar.gearbox")
        .build();
    application.connect_activate(on_activate);
    application.connect_startup(on_startup);
    application.run()
}
