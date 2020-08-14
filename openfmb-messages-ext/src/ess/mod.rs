use crate::OpenFMBError;

mod control;
mod reading;
mod status;

pub use control::*;
pub use reading::*;
pub use status::*;

pub trait ESSStatusExt {
    fn soc(&self) -> Result<f32, OpenFMBError>;
}

pub trait ESSReadingExt {}
