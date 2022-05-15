use log_example_rs::example;
use log_example_rs::log;

fn main() {
    log!("start");

    example::example1();
    example::example2();
    example::example3();

    log!("done")
}
