use libadwaita as adw;

use adw::{gtk, glib, gio};
use adw::prelude::*;
use adw::{Application, ApplicationWindow, AboutWindow, PreferencesWindow};
use gtk::Builder;
use gio::ActionEntry;

fn main() -> glib::ExitCode {
  gio::resources_register_include!("resources.gresource").unwrap();
  let app = Application::builder()
    .application_id("floppa.wallet")
    .build();

  app.connect_activate(|app| {
    let template = Builder::from_resource("/ui/window.ui");
    let window: ApplicationWindow = template.object("window").unwrap();
    window.set_application(Some(app));
    window.add_action_entries([
      ActionEntry::builder("about")
        .activate(|win, _, _| {
          AboutWindow::builder()
            .application_name("Floppa Wallet")
            .application_icon("help-about")
            .version(env!("CARGO_PKG_VERSION"))
            .transient_for(win)
            .build()
            .show();
        })
        .build(),
      ActionEntry::builder("prefs")
        .activate(|win, _, _| {
          let window = PreferencesWindow::builder().transient_for(win).build();
          window.show();
        })
        .build(),
    ]);
    window.show();
  });

  app.run()
}
