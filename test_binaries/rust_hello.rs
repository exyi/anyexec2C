#![feature(start, std_misc)]

#[start]
fn start(argc: isize, argv: *const *const u8) -> isize {
    // hopefully we dont need command line arguments
    //unsafe { ::std::rt::args::init(argc, argv); }
    println!("Hello World from Rust!");                                                                                        
    return 123;
}

fn main() {
    ::std::process::exit(124);
}