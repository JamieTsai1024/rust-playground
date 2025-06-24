mod chrono_bson_datetime;
mod date_time;
mod mid_implementation;
mod new_implementation;
mod old_implementation;

fn main() {
    // Loop imdexing
    // for ix in [0, 1] {
    //     println!("Hello, world: {}", ix);
    // }

    // RUST-1748
    // old_implementation::run_example();
    // mid_implementation::run_example();
    new_implementation::run_example();
    date_time::run_example();
    chrono_bson_datetime::run_example();
}
