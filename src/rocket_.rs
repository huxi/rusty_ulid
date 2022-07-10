//! [ULID](https://github.com/ulid/spec) path/query parameter and form value parsing support.
//!
//! # Enabling
//!
//! This module is only available when the `rocket` feature is enabled. Enable it
//! in `Cargo.toml` as follows:
//!
//! ```toml
//! [dependencies.rusty_ulid]
//! version = "1"
//! features = ["rocket"]
//! ```
//!
//! # Usage
//!
//! `Ulid` implements [`FromParam`] and [`FromFormField`] (i.e,
//! [`FromForm`](rocket::form::FromForm)), allowing ULID values to be accepted
//! directly in paths, queries, and forms. You can use the `Ulid` type directly
//! as a target of a dynamic parameter:
//!
//! ```rust
//! # #[macro_use] extern crate rocket;
//! use rusty_ulid::Ulid;
//!
//! #[get("/users/<id>")]
//! fn user(id: Ulid) -> String {
//!     format!("We found: {}", id)
//! }
//! ```
//!
//! You can also use the `Ulid` as a form value, including in query strings:
//!
//! ```rust
//! # #[macro_use] extern crate rocket;
//! use rusty_ulid::Ulid;
//!
//! #[get("/user?<id>")]
//! fn user(id: Ulid) -> String {
//!     format!("User ID: {}", id)
//! }
//! ```
//!
//! Additionally, `Ulid` implements `UriDisplay<P>` for all `P`. As such, route
//! URIs including `Ulid`s can be generated in a type-safe manner:
//!
//! ```rust
//! # #[macro_use] extern crate rocket;
//! use rusty_ulid::Ulid;
//! use rocket::response::Redirect;
//!
//! #[get("/user/<id>")]
//! fn user(id: Ulid) -> String {
//!     format!("User ID: {}", id)
//! }
//!
//! #[get("/user?<id>")]
//! fn old_user_path(id: Ulid) -> Redirect {
//!     # let _ = Redirect::to(uri!(user(&id)));
//!     # let _ = Redirect::to(uri!(old_user_path(id)));
//!     # let _ = Redirect::to(uri!(old_user_path(&id)));
//!     Redirect::to(uri!(user(id)))
//! }
//! ```
//!

use rocket::form::{self, FromFormField, ValueField};
use rocket::http::impl_from_uri_param_identity;
use rocket::http::uri::fmt::{Formatter, Part, UriDisplay};
use rocket::request::FromParam;

/// Error returned on [`FromParam`] or [`FromFormField`] failure.
///
use crate::DecodingError;

use crate::Ulid;

impl<'a> FromParam<'a> for Ulid {
    type Error = DecodingError;

    /// A value is successfully parsed if `param` is a properly formatted Ulid.
    /// Otherwise, an error is returned.
    #[inline(always)]
    fn from_param(param: &'a str) -> Result<Ulid, Self::Error> {
        param.parse()
    }
}

impl<'v> FromFormField<'v> for Ulid {
    #[inline]
    fn from_value(field: ValueField<'v>) -> form::Result<'v, Self> {
        Ok(field.value.parse().map_err(form::error::Error::custom)?)
    }
}

/// This implementation is identical to the `Display` implementation.
impl<P: Part> UriDisplay<P> for Ulid {
    #[inline(always)]
    fn fmt(&self, f: &mut Formatter<'_, P>) -> std::fmt::Result {
        use std::fmt::Write;
        write!(f, "{}", self)
    }
}

impl_from_uri_param_identity!(Ulid);
