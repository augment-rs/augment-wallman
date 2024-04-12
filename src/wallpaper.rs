use gtk::{prelude::*, *};
use gtk_layer_shell::*;

use crate::error::Error;

/// A layer that will display an image in the background.
pub struct Wallpaper<'a> {
    window: ApplicationWindow,
    wallpaper: Image,
    pixbuf: Option<&'a gdk_pixbuf::Pixbuf>,
}

impl<'a> Wallpaper<'a> {
    pub fn new(monitor: &gdk::Monitor, application: &Application) -> Self {
        let window = ApplicationWindow::new(application);

        // Set the window to be on the background layer.
        window.init_layer_shell();
        window.set_layer(Layer::Background);
        window.set_monitor(monitor);

        // Needed to make the window cover the entire monitor.
        window.set_exclusive_zone(1);

        let wallpaper = Image::new();

        window.add(&wallpaper);

        Self {
            window,
            wallpaper,
            pixbuf: None,
        }
    }

    /// Will fail if the pixbuf has not been set yet.
    pub fn run(&self) -> Result<(), Error> {
        if self.pixbuf.is_none() {
            return Err(Error::PixbufNotYetImplemented);
        }

        self.window.show_all();

        Ok(())
    }

    /// Set the wallpaper image.
    pub fn set_pixbuf(&mut self, pixbuf: &'a gdk_pixbuf::Pixbuf) {
        self.pixbuf = Some(pixbuf);

        self.wallpaper.set_from_pixbuf(self.pixbuf);
    }
}
