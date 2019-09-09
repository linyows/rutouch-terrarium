#[macro_use]
extern crate http_guest;

use http_guest::{Request, Response, RequestExt};

pub fn user_entrypoint(_req: &Request<Vec<u8>>) -> Response<Vec<u8>> {
    let mut vec: Vec<u8> = Vec::new();
    let mut url = "https://raw.githubusercontent.com/linyows/rutouch-terrarium/master/misc/kagoshima.jpg";
    let req = Request::builder()
        .method("GET")
        .uri(url)
        .header("X-Terrarium", "test")
        .body(vec)
        .unwrap();
    RequestExt::send(req).expect("request failed")
}

guest_app!(user_entrypoint);
