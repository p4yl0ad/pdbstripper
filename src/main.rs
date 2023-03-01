use std::env::args;
use std::process::exit;
use regex::bytes::Regex;
use regex::bytes::Captures;

const ERR_NOT_ENOUGH_ARGS: i32 = 0x00000001;

fn overwrite_pdb_string(target: String, outfile: String){
    println!("overwrite_pdb_string {}", target);
    let mut file = std::fs::read(target.clone()); // vec of u8s
    match file {
        Ok(val) => {
            println!("[P] successfully read file {}", target);
            let re = Regex::new(r"C:\\.*\.pdb").unwrap();
            let c = &val[..];
            let mat = re.find(c);
            match mat {
                Some(matchval) => {
                    println!("START \t: {}\nEND\t: {}", matchval.start(), matchval.end());
                    println!("LENGTH\t: {}", (matchval.end() - matchval.start()));
                    let arr = vec![0u8; matchval.end() - matchval.start()];
                    let c = &arr[..];
                    println!("{:?}", c);
                    let result = re.replace_all(&val, c);
                    println!("{:?}", result);
                    let file_out = std::fs::write(outfile.clone(), result);
                    match file_out {
                        Ok(ref file_out) => {
                            println!("[P] Wrote data to : {}", outfile.clone());
                        },
                        Err(error2) => {
                            eprintln!("[F] Failed to write data to {} with the error : {}", outfile, error2);
                        }
                    }
                },
                None => {
                    eprintln!("[F] Did not find a .pdb string in target binary");
                }
            }
        },
        Err(error) => {
            eprintln!("[F] Failed to read input file \"{}\" with error : {}", target.clone(), error);
        }
    }
}

fn main() {
    let args:Vec<String> = args().collect();
    println!("Args {}", args.len());
    if args.len() < 3 {
        println!("Usage: {} target.exe target_out.exe", args[0]);
        exit(ERR_NOT_ENOUGH_ARGS);
    }
    println!("Target file to modify \"{}\"", args[1].to_string());
    println!("Output file to modify \"{}\"", args[2].to_string());
    let _ = overwrite_pdb_string(args[1].to_string(), args[2].to_string());
}

/*
USAGE:
cargo run --release target.exe target_out.exe

TODO:
[X] pdb remover - C:\Users\p4\CLionProjects\peparse\target\release\deps\peparse.pdb repalce with 0x00
[] rust ref remover - overwrite all strings which reference .rs
[] Timestomping - stamp on compile time in PE header
*/
