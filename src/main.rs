extern crate base64;
use std::io::*;
use std::fs::*;

mod c_code;
use c_code::{C_FOOTER, C_HEADER};

struct CmdArgs {
    comment_files: Vec<String>,
}

fn parse_args() -> CmdArgs {
    let mut comment_files = Vec::new();
    let mut state = -1;
    for a in std::env::args() {
        if a == "-c" || a == "--commentFiles" {
            state = 0;
        } else if state == 0 {
            comment_files.push(a)
        }
    }
    CmdArgs { comment_files: comment_files }
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
    stdout().write(C_FOOTER).unwrap();
}
