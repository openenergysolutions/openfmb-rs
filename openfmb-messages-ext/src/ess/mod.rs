// SPDX-FileCopyrightText: 2021 Open Energy Solutions Inc
//
// SPDX-License-Identifier: Apache-2.0

use crate::OpenFMBError;

mod control;
mod event;
mod reading;
mod status;

pub use control::*;
pub use event::*;
pub use reading::*;
pub use status::*;

pub trait ESSStatusExt {
    fn soc(&self) -> Result<f64, OpenFMBError>;
}

pub trait ESSReadingExt {}
