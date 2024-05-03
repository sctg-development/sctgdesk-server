mod tokens;
mod bearer;
pub mod address_book;

pub mod types;
pub mod get_host;

pub use tokens::Token;
pub use bearer::{BearerAuthToken, CookieAuthToken, MixedAuthToken, IntoToken};
// pub use address_book::AddressBook;
pub use types::*;

#[macro_export]
macro_rules! unwrap_or_return {
    ($optval:expr) => {
        match $optval {
            Ok(val) => val,
            Err(err) => {
                log::debug!("ERR in unwrap_or_return: {:?}", &err);
                return err;
            },
        }
    };
}

#[macro_export]
macro_rules! include_svg_as_base64 {
    ($path:expr) => {
        {
            use ::base64::prelude::{Engine as _, BASE64_STANDARD};
            let svg_data = include_bytes!($path);
            let base64 = BASE64_STANDARD.encode(svg_data);
            format!("data:image/svg+xml;base64,{}", base64)
        }
    };
}

#[macro_export]
macro_rules! include_png_as_base64 {
    ($path:expr) => {
        {
            use ::base64::prelude::{Engine as _, BASE64_STANDARD};
            let svg_data = include_bytes!($path);
            let base64 = BASE64_STANDARD.encode(svg_data);
            format!("data:image/png;base64,{}", base64)
        }
    };
}
