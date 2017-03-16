extern crate iron;
extern crate router;
#[macro_use]
extern crate chan;
extern crate chan_signal;

use iron::{Iron, Request, Response, IronResult};
use iron::status;
use router::Router;
use std::sync::{Arc, Mutex};
use chan_signal::Signal;

mod rover;
use rover::Rover;

fn main() {
    let rover1 = Arc::new(Mutex::new(Rover::new().unwrap()));
    let rover2 = rover1.clone();
    let rover3 = rover1.clone();
    let rover4 = rover1.clone();

    let signal = chan_signal::notify(&[Signal::INT, Signal::TERM]);

    let mut router = Router::new();
    router.put("/api/stop",
               move |r: &mut Request| stop(r, &rover1.lock().unwrap()),
               "stop");
    router.put("/api/speed",
               move |r: &mut Request| set_speed(r, &rover2.lock().unwrap()),
               "set_speed");
    router.get("/api/speed",
               move |r: &mut Request| set_speed(r, &rover3.lock().unwrap()),
               "get_speed");

    let mut serv = Iron::new(router).http("0.0.0.0:3000").unwrap();

    println!("now listening");

    chan_select! {
        signal.recv() -> signal => {
            println!("received signal: {:?}", signal);
            serv.close().unwrap();
        },
    }

    println!("exiting");
    let r = &rover4.lock().unwrap();
    r.stop().unwrap();
    // r.unexport().unwrap();

    fn stop(_: &mut Request, rover: &Rover) -> IronResult<Response> {
        rover.stop().unwrap();
        println!("stop");
        Ok(Response::with((status::Ok, "stop")))
    }

    fn get_speed(_: &mut Request, rover: &Rover) -> IronResult<Response> {
        let (left, right) = rover.get_speed().unwrap();
        println!("speed {} {}", left, right);
        Ok(Response::with((status::Ok, format!("speed {} {}", left, right))))
    }

    fn set_speed(_: &mut Request, rover: &Rover) -> IronResult<Response> {
        rover.set_speed(100, 100).unwrap();
        Ok(Response::with((status::Ok, "speed")))
    }
}
