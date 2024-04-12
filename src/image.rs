use crate::error::Error;
use gtk::gdk_pixbuf::{self, Pixbuf};
use std::{fs::File, io::Write, path::Path};

/// A wrapper around `gdk_pixbuf::Pixbuf` that provides some useful methods.
pub struct Image {
    pixbuf: Pixbuf,
}

impl Image {
    pub fn new<T: AsRef<Path>>(path: T) -> Result<Self, Error> {
        let pixbuf = match Pixbuf::from_file(&path) {
            Ok(pixbuf) => pixbuf,
            Err(_) => {
                let default_wallpaper = include_bytes!("../assets/default_wallpaper.png");

                let mut wallpaper_file = File::create(&path).unwrap();
                wallpaper_file.write_all(default_wallpaper).unwrap();

                Pixbuf::from_file(&path).unwrap()
            }
        };
        Ok(Self { pixbuf })
    }

    pub fn get_pixbuf(&self) -> &Pixbuf {
        &self.pixbuf
    }

    /// Crop the image to fit the given dimensions.
    pub fn crop(&mut self, width: i32, height: i32) {
        let (width, height) =
            compute_crop(self.pixbuf.width(), self.pixbuf.height(), width, height);

        self.pixbuf = self
            .pixbuf
            .scale_simple(width, height, gdk_pixbuf::InterpType::Bilinear)
            .unwrap();
    }

    /// Resize the image to fit the given dimensions. The aspect ratio is not preserved, so the image may be distorted.
    pub fn resize(&mut self, width: i32, height: i32) {
        self.pixbuf = self
            .pixbuf
            .scale_simple(width, height, gdk_pixbuf::InterpType::Bilinear)
            .unwrap();
    }

    /// Fit the image to the given dimensions.
    pub fn fit(&mut self, width: i32, height: i32) {
        let (width, height) = compute_fit(self.pixbuf.width(), self.pixbuf.height(), width, height);

        self.pixbuf = self
            .pixbuf
            .scale_simple(width, height, gdk_pixbuf::InterpType::Bilinear)
            .unwrap();
    }
}

fn compute_crop(mut width1: i32, mut height1: i32, width2: i32, height2: i32) -> (i32, i32) {
    let aspect_ratio = width1 as f64 / height1 as f64;

    if width2 as f64 / height2 as f64 >= aspect_ratio {
        width1 = width2;
        height1 = (width1 as f64 / aspect_ratio).ceil() as i32;
    } else {
        height1 = height2;
        width1 = (height1 as f64 * aspect_ratio).ceil() as i32;
    }

    (width1, height1)
}

fn compute_fit(mut width1: i32, mut height1: i32, width2: i32, height2: i32) -> (i32, i32) {
    let aspect_ratio = width1 as f64 / height1 as f64;

    if width2 as f64 / height2 as f64 >= aspect_ratio {
        height1 = height2;
        width1 = (height1 as f64 * aspect_ratio).ceil() as i32;
    } else {
        width1 = width2;
        height1 = (width1 as f64 / aspect_ratio).ceil() as i32;
    }

    (width1, height1)
}

#[cfg(test)]
mod tests {
    use crate::image::{compute_crop, compute_fit};

    #[test]
    fn test_compute_crop() {
        assert_eq!((1920, 1080), compute_crop(1280, 720, 1920, 1080));
        assert_eq!((1920, 3414), compute_crop(720, 1280, 1920, 1080));
        assert_eq!((1920, 1092), compute_crop(1900, 1080, 1920, 1080));
    }

    #[test]
    fn test_compute_fit() {
        assert_eq!((1920, 1080), compute_fit(1280, 720, 1920, 1080));
        assert_eq!((608, 1080), compute_fit(720, 1280, 1920, 1080));
        assert_eq!((1900, 1080), compute_fit(1900, 1080, 1920, 1080));
    }
}
