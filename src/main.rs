extern crate base64;
use std::io::*;
use std::fs::*;

mod c_code;
use c_code::*;

struct CmdArgs {
    comment_files: Vec<String>,
    with_error_checks: bool,
}

fn parse_args() -> CmdArgs {
    let mut comment_files = Vec::new();
    let mut state = -1;
    let mut with_error_checks = false;
    for a in std::env::args() {
        if a == "-c" || a == "--commentFiles" {
            state = 0;
        } else if a == "--with-error-checks" || a == "-e" {
            with_error_checks = true;
            state = -1;
        } else if state == 0 {
            comment_files.push(a)
        }
    }
    CmdArgs { comment_files: comment_files, with_error_checks: with_error_checks }
}

fn main() {
    let args = parse_args();
    let mut buffer = Vec::new();
    stdin().read_to_end(&mut buffer).expect(
        "Could not read stdin",
    );

    // write the actual source code as C comments
    for f in args.comment_files {
        println!("// ==============================", );
        println!("// {}", f);
        println!("// ==============================");
        let f = BufReader::new(File::open(&f).expect(
            &format!("Could not find comment file - {}", &f)
        ));
        for l in f.lines() {
            println!("// {}", l.unwrap());
        }
        println!();
    }

    // Create the actually working part of the C code
    stdout().write(C_HEADER).unwrap();
    print!("{}", base64::encode(&buffer));
    stdout().write(C_LIB_FUNCTIONS).unwrap();

    stdout().write(if args.with_error_checks { C_MAIN_WITH_CHECKS } else { C_MAIN_SIMPLE }).unwrap();
}
