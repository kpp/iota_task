use iota_task::*;

fn main() {
    let filepath = std::env::args().skip(1).next()
        .expect("The first arg should be a path to the database");

    process_file(&filepath);
}
