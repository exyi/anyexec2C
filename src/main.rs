extern crate base64;
extern crate argparse;
extern crate askama;



use std::io::*;
use std::fs::*;
use std::process::{Command, exit};
use std::path::Path;
use std::env;

use argparse::{ArgumentParser, List, StoreOption};

mod code;

use code::CodeTemplate;

enum OutputLanguage {
    C,
    CWithChecks,
    CSharp
}


struct CmdArgs {
    comment_files: Vec<String>,
    build_file: Option<String>,
    asset_files: Vec<String>,
    exec_file: Option<String>,
    target: OutputLanguage,
}

fn parse_args() -> CmdArgs {
    let mut args = CmdArgs {
        comment_files: Vec::new(),
        build_file: None,
        asset_files: Vec::new(),
        exec_file: None,
        target: OutputLanguage::C
    };

    let mut target = "c".to_owned();
    {
        let mut ap = ArgumentParser::new();
        ap.set_description("Generate source code for staged delivery of any binary executable.");
        ap.refer(&mut args.comment_files).add_option(
            &["-c", "--comment"],
            List,
            "Original source code files attached as comments inside the generated file",
        );
        ap.refer(&mut args.build_file).add_option(
            &["-b", "--build"],
            StoreOption,
            "Compile and exec this file. It also attaches it as a comment file. Can't be used together with --exec",
        );
        ap.refer(&mut args.asset_files).add_option(
            &["-a", "--asset"],
            List,
            "File that will be just saved alongside your executable. Its name will stay the same.",
        );
        ap.refer(&mut args.exec_file).add_option(
            &["-x", "--exec"],
            StoreOption,
            "Delivers a file. Can't be used together with --build",
        );
        ap.refer(&mut target).add_option(
            &["-t", "--target"],
            argparse::Store,
            "Output Language"
        );
        ap.parse_args_or_exit();

    }
    args.target =
        match target.to_lowercase().as_str() {
            "c" => OutputLanguage::C,
            "c_with_checks" => OutputLanguage::CWithChecks,
            "csharp" | "c#" => OutputLanguage::CSharp,
            _ => {
                eprintln!("Unsupported target type '{}'.", target);
                exit(1);
            }
        };


    // check if exec and build correctly set
    if args.build_file.is_some() == args.exec_file.is_some() {
        if args.build_file.is_some() {
            println!("Can't have build and exec together.");
        } else {
            println!("Nothing to execute.");
        }
        exit(1);
    }

    // add build file to comment files
    if args.build_file.is_some() {
        let c = args.build_file.take().unwrap();
        args.comment_files.push(c.clone());
        args.build_file = Some(c);
    }

    args
}

fn generate_source<T: CodeTemplate>(binary_filename: String, asset_files: Vec<String>, comment_files: Vec<String>) -> String {
    // write the actual source code as C comments
    let mut loaded_comment_files = Vec::new();
    for filename in comment_files {
        let path = Path::new(&filename);
        if !path.exists() {
            eprintln!("{} - file does not exist!", filename);
            exit(1);
        }

        let f = BufReader::new(File::open(&filename).expect(&format!(
            "Could not find comment file - {}",
            &filename
        )));

        loaded_comment_files.push((filename, f.lines().map(|l| l.unwrap()).collect()));
    }


    let mut f = File::open(binary_filename).unwrap();
    let mut executable = Vec::new();
    f.read_to_end(&mut executable).unwrap();
    let binary_b64 = base64::encode(&executable);

    let mut assets = Vec::new();
    for asset in asset_files {
        let mut f = File::open(&asset).unwrap();
        let mut data = Vec::new();
        f.read_to_end(&mut data).unwrap();
        assets.push((asset, base64::encode(&data)));
    }

    T::render(binary_b64, assets, loaded_comment_files)
}

fn main() {
    let mut args = parse_args();

    // build executable
    if let Some(build_file) = args.build_file {
        let extension = build_file.clone();
        let extension = extension.split(".").last();
        if extension.is_none() {
            eprintln!("Failed to split extension off build file name");
            exit(1);
        }
        let extension = extension.unwrap();
        args.exec_file = match extension {
            // D lang
            "d" => {
                compile(&format!("dmd -O {} -of=a.out -od={}",
                                 build_file,
                                 env::temp_dir().as_os_str().to_str().expect("TMP dir has non-unicode name!")
                ), "a.out")
            }
            "go" => {
                compile(&format!("go build -o a.out {}", build_file), "a.out")
            }
            "c" => {
                compile(&format!("gcc -o a.out {} -O3 -Wall", build_file), "a.out")
            }
            "cpp" => {
                compile(&format!("g++ -o a.out {} -O3 -std=c++17 -Wall", build_file), "a.out")
            }
            "rs" => {
                let program_name = bash_command("cat Cargo.toml | grep \"name\" | sed 's/.*\"\\(.*\\)\"/\\1/'");
                let pn = program_name.trim();
                compile(&format!("cargo build -Z unstable-options --release --target x86_64-unknown-linux-musl --out-dir ."), pn)
            }
            extension => {
                eprintln!("File extension '{}' not recognized! Can't build!", extension);
                exit(1);
            }
        };
    }

    // generate source code
    let exec_file = args.exec_file.expect("No executable file supplied!");
    println!("{}", match args.target {
        OutputLanguage::C => generate_source::<::code::c_code::CCodeTemplate>,
        OutputLanguage::CWithChecks => generate_source::<::code::c_code::CCodeWithChecksTemplate>,
        OutputLanguage::CSharp => generate_source::<::code::csharp_code::CSharpCodeTemplate>,
    }(exec_file, args.asset_files, args.comment_files));
}


/// Takes `&str`, executes it. Expected executable result is file called `a.out`
fn compile(build_command: &str, result_binary: &str) -> Option<String> {
    // assemble command
    let mut parts = build_command.split(" ");
    let mut command = Command::new(parts.next().unwrap());
    let args: Vec<&str> = parts.collect();
    let command = command.args(args);

    // execute
    let output = command.output()
        .expect(&format!("Failed to execute compiler. Command:\n{}", build_command));

    // Check output and print it to stderr...
    {
        let stderr = ::std::io::stderr();
        let mut handle = stderr.lock();
        let _result = handle.write_all(&output.stdout);
        let _result = handle.write_all(&output.stderr);
    }

    // check status
    if !output.status.success() {
        eprintln!("\nCompilation error!");
        exit(1);
    }

    // Check for compiled binary existence
    let path = Path::new(result_binary);
    if !path.exists() {
        eprintln!("Couldn't find compiled binary... Failed.");
        exit(1);
    }

    bash_command(&format!("strip {}", result_binary));      // remove debug symbols from the binary

    Some(String::from(result_binary))
}

fn bash_command(cmd: &str) -> String {
    // assemble command
    let mut command = Command::new("bash");
    let command = command.args(&["-c", cmd]);

    // execute
    let output = command.output()
        .expect(&format!("Failed to execute bash command. Command:\n{}", cmd));

    // Print stderr to stderr...
    {
        let stderr = ::std::io::stderr();
        let mut handle = stderr.lock();
        let _result = handle.write_all(&output.stderr);
    }

    // check status
    if !output.status.success() {
        eprintln!("\nBash command execution error!");
        exit(1);
    }

    String::from_utf8(output.stdout).expect("Could not parse command output - invalid UTF8")
}


#[test]
fn test_bash_command() {
    assert_eq!(bash_command("echo \"ahoj\" | cat"), "ahoj\n");
    assert_eq!(bash_command("cat Cargo.toml | grep \"name\" | sed 's/.*\"\\(.*\\)\"/\\1/'"), "anyexec2c\n");
}
