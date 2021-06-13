//! # an auto-impl http client that will leave you `happi` :)
//!
//! This includes a CI/CD pipeline, README templating, and cargo-make scripts.
//!
//! # Setup
//! |Type|Name|Value|How-To|
//! |--|--|--|--|
//! |Github Repo Secret|CARGO_TOKEN|Token issued for your user by crates.io|[How-To](https://doc.rust-lang.org/cargo/reference/publishing.html#before-your-first-publish)|
//! |Github Repo Secret|GH_TOKEN|A GitHub PAT|[How-To](https://docs.github.com/en/github/authenticating-to-github/creating-a-personal-access-token)|

#![doc(html_root_url = "https://docs.rs/happi-derive/0.0.6")]
#![cfg_attr(docsrs, feature(doc_cfg))]
// #![feature(doc_cfg)] // for local docs
#![warn(missing_copy_implementations)]
#![warn(missing_debug_implementations)]
#![warn(unreachable_pub)]
#![warn(unused_crate_dependencies)]
#![deny(missing_docs)]
#![forbid(unsafe_code)]

extern crate proc_macro;

use quote::ToTokens;

#[proc_macro_attribute]
pub fn happi(attr: proc_macro::TokenStream,
           item: proc_macro::TokenStream)
           -> proc_macro::TokenStream {
  let struct_ = syn::parse::<syn::ItemStruct>(item).expect("should be a struct definition");
  struct_.to_token_stream().into()
}
