use std::process::exit;

pub fn error_exit(stderr: String) -> ! {
    println!("{stderr}");
    exit(1);
}
