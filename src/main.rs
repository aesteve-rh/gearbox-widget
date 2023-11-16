// SPDX-FileCopyrightText: Red Hat, Inc.
// SPDX-License-Identifier: GPL-3.0-or-later

mod gearbox;

use clap::Parser;
use gtk::{gdk, gio, glib, prelude::*};
use vhal_emulator as ve;

const WIDTH: i32 = 180;
const HEIGHT: i32 = 280;

#[derive(Parser)]
struct Cli {
    #[clap(short, long, help = "Specify the local port for the VHAL socket")]
    local_port: Option<u16>,
}

fn on_activate(application: &gtk::Application) {
    let args = Cli::parse();
    let port = match args.local_port {
        Some(port) => port,
        None => ve::adb_port_forwarding().unwrap(),
    };
    let gearbox = gearbox::GearboxWidget::with_port(port as u64);
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
    application.run_with_args(&Vec::<String>::new())
}
