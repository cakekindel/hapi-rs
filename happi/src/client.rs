//! A client capable of executing requests

use crate::{Error, Fut};

/// A client capable of executing requests
// TODO(orion): decouple from hyper
pub trait Client {
  /// Execute a request
  fn execute(&self,
             req: hyper::Request<hyper::Body>)
             -> Fut<Result<hyper::Response<hyper::Body>, Error>>;
}
