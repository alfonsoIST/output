use std::{thread, time};
use std::fs;

use output::OUTPUT;


mod output;
fn main() {
    OUTPUT::replace("logfile","hello");
    OUTPUT::display("logfile");
    output::warn("Hello Warn");
    output::error("Hello Error");
    output::debug("Hello Debug");
    output::info("Hello Info");
    output::ok("Hello Ok");
    let mut rotor = output::start("Hello Start ending in OK ... ");
    thread::sleep(time::Duration::new(1,0));
    rotor.ok("Hello task was completed Ok");
    let mut rotor = output::start("Hello Start ending in ERROR ... ");
    thread::sleep(time::Duration::new(1,0));
    rotor.error("Hello task was completed with Error");
    output::info( &output::green("Print in green"));
    output::info( &output::red("Print in red"));
    output::info( &output::cyan("Print in cyan"));
    output::info( &output::b_cyan("Print in b_cyan"));
    output::info( &output::grey("Print in grey"));
    output::info( &output::magenta("Print in magenta"));
    // Example of error control and display
    let myfile = "/non-writable-file";
    match fs::write(&myfile, "some-data") {
        Ok(_data) => (),
        Err(error) => output::display_error("Cannot write to file", &myfile, error),
    }
}
