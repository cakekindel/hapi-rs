//! An API modeled with happi

use crate as happi;

/// An API modeled with happi
pub trait Api {
  /// The base route for the API
  fn base_url(&self) -> Option<&str>;

  /// The Client used to execute requests
  fn client(&self) -> &dyn happi::Client;
}
