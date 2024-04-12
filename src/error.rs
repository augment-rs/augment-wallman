use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Pixbuf Not Yet Implemented")]
    PixbufNotYetImplemented,
}
