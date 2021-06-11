//! # an auto-impl http client that will leave you `happi` :)
//!
//! This includes a CI/CD pipeline, README templating, and cargo-make scripts.
//!
//! # Setup
//! |Type|Name|Value|How-To|
//! |--|--|--|--|
//! |Github Repo Secret|CARGO_TOKEN|Token issued for your user by crates.io|[How-To](https://doc.rust-lang.org/cargo/reference/publishing.html#before-your-first-publish)|
//! |Github Repo Secret|GH_TOKEN|A GitHub PAT|[How-To](https://docs.github.com/en/github/authenticating-to-github/creating-a-personal-access-token)|

#![doc(html_root_url = "https://docs.rs/happi/0.0.1")]
#![cfg_attr(docsrs, feature(doc_cfg))]
// #![feature(doc_cfg)] // for local docs
#![warn(missing_copy_implementations)]
#![warn(missing_debug_implementations)]
#![warn(unreachable_pub)]
#![warn(unused_crate_dependencies)]
#![deny(missing_docs)]
#![forbid(unsafe_code)]

/// TODO
#[derive(Clone, Copy, Debug)]
pub struct Todo;
