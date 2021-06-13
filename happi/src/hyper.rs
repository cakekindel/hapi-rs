//! Use `hyper` as a driver for `happi`
//!
//! Implements `happi::Client` for `hyper::Client`.

use futures::{FutureExt, TryFutureExt};
pub use hyper::{client::connect::HttpConnector, Body, Request, Response};

use crate as happi;

impl happi::Client for hyper::Client<HttpConnector, Body> {
  fn execute(&self,
             req: Request<Body>)
             -> happi::Fut<Result<Response<Body>, happi::Error>> {
    self.request(req).map_err(happi::Error::Http).boxed()
  }
}

#[cfg(test)]
mod test {
  use crate as happi;

  #[test]
  fn it_works() {
    async fn run() {
      let _mock = mockito::mock("GET", "/hello").with_status(200)
                                                .with_body("foo")
                                                .create();

      let client = hyper::Client::new();
      let req = hyper::Request::get(format!("{}/hello", mockito::server_url())).body(hyper::Body::empty()).unwrap();
      let res = happi::Client::execute(&client, req).await.unwrap();

      assert_eq!(res.status(), 200)
    }

    tokio_test::block_on(run())
  }
}
