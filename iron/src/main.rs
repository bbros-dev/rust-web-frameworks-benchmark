// Reference:
// -
extern crate iron;

use iron::prelude::*;
use iron::status;


fn hw(_: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::Ok, "Hello World!")))
}

fn main() {
    Iron::new(hw).http("0.0.0.0:3000").unwrap();
}
