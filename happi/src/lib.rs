//! # an auto-impl http client that will leave you `happi` :)
//!
//! This includes a CI/CD pipeline, README templating, and cargo-make scripts.
//!
//! # Example
//! Using an example of <https://reqres.in/>, a `happi` implementation of this would look **something** like:
//!
//! ```ignore
//! pub fn main() -> Result<(), dyn std::error::Error> {
//!   let reqres = ReqResApi(reqwest::blocking::Client::new());
//!
//!   let user_page = reqres.get_all_users(None)?;
//!
//!   println!("{:#?}", user_page);
//! }
//!
//! // This is the *real* client that hits the API
//! #[happi(base_url = "https://reqres.in/api", blocking)]
//! pub struct ReqResApi(#[client] hyper::Client);
//!
//! // This is a trait for the `user` resource that `happi`
//! // will implement for `ReqResApi`.
//! //
//! // When you want to use this resource, your function can
//! // accept an `impl reqres::UserResource`, accepting the real
//! // deal or a mock when you write tests.
//! #[happi(
//!   api(ReqResApi),
//!   resource("/users"),
//!   responds(200, json),
//! )]
//! pub trait UserResource {
//!   #[get]
//!   pub fn get_all_users(&self, #[query] page: Option<u32>) -> Result<UserPage, happi::Error>;
//!
//!   #[get(
//!     "/{id}",
//!     responds(404, unit),
//!     when(status == 404, invoke = |_resp| Ok(None)),
//!   )]
//!   pub fn get_user(&self, id: u32) -> Result<Option<User>, happi::Error>;
//! }
//!
//! #[derive(Debug, Serialize, Deserialize)]
//! pub struct User {
//!   id: u32,
//!   first_name: String,
//!   last_name: String,
//!   avatar: String,
//! }
//!
//! #[derive(Debug, Serialize, Deserialize)]
//! pub struct UserPage {
//!   page: u32,
//!   per_page: u32,
//!   total: u32,
//!   total_pages: u32,
//!   data: Vec<User>
//! }
//! ```

#![doc(html_root_url = "https://docs.rs/happi/0.0.4")]
#![cfg_attr(docsrs, feature(doc_cfg))]
// #![feature(doc_cfg)] // for local docs
#![warn(missing_copy_implementations)]
#![warn(missing_debug_implementations)]
#![warn(unreachable_pub)]
#![warn(unused_crate_dependencies)]
#![deny(missing_docs)]
#![forbid(unsafe_code)]

use std::{future::Future, pin::Pin};

use happi_derive::foo;

pub mod client;
pub mod hyper;

#[doc(inline)]
pub use client::Client;

/// happi error
#[derive(Debug)]
pub enum Error {
  /// There was an error serializing your request body
  RequestSerialize(serde_json::Error),
  /// There was an error that prevented the HTTP request from succeeding
  Http(::hyper::Error),
}

/// A `dyn Future` returned from a trait function.
pub type Fut<T> = Pin<Box<dyn Future<Output = T>>>;
