use gtk::{gio::ApplicationFlags, prelude::*, *};
use serde::{Deserialize, Serialize};
use wallpaper::Wallpaper;

mod config;
mod error;
mod image;
mod wallpaper;

#[derive(Serialize, Deserialize)]
enum DisplayType {
    Fit,
    Crop,
    Resize,
}

pub fn init() {
    let application = Application::new(
        Some("ind.nanashi.augment.wallman"),
        ApplicationFlags::default(),
    );

    application.connect_activate(run);

    application.run();
}

fn run(application: &Application) {
    let number_of_monitor = gdk::Display::default().unwrap().n_monitors();
    let config = config::Config::new();

    for i in 0..number_of_monitor {
        let monitor = gdk::Display::default().unwrap().monitor(i).unwrap();

        let mut wallpaper = Wallpaper::new(&monitor, application);

        let mut image = image::Image::new(
            xdg::BaseDirectories::new()
                .unwrap()
                .get_config_file("background"),
        )
        .unwrap();

        match config.display_type {
            DisplayType::Fit => image.fit(monitor.geometry().width(), monitor.geometry().height()),
            DisplayType::Crop => {
                image.crop(monitor.geometry().width(), monitor.geometry().height())
            }
            DisplayType::Resize => {
                image.resize(monitor.geometry().width(), monitor.geometry().height())
            }
        }

        wallpaper.set_pixbuf(image.get_pixbuf());
        wallpaper.run().unwrap();
    }
}
