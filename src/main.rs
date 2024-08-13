use std::{env, process::exit};

// use pdbmol::ccd::CCD;

// fn main() {
//     let args: Vec<String> = env::args().collect();

//     for resname in &args[1..] {
//         println!("{:#?}", CCD[resname.as_str()]);
//     }
// }

use pdbmol::parser::pdb;

fn main() {
    for filename in env::args().skip(1) {
        let records = pdb::load(&filename);
        match &records {
            Err(e) => {
                eprintln!("Errors encountered loading {filename}: {e}");
                exit(1)
            }
            Ok(records) => {
                println!("{filename}:");
                let mut errors = false;
                for record in records {
                    if record.is_err() {
                        errors = true
                    }
                    println!("    {record:?}");
                }
                if errors {
                    eprintln!("Errors encountered in {filename}");
                    exit(2)
                }

                let pdb_str = std::fs::read_to_string(&filename).unwrap();
                let mut lines_iter = pdb_str.lines();
                for written_line in records
                    .iter()
                    .map(|r| r.as_ref().unwrap())
                    .map(|record| format!("{record}"))
                    .flat_map(|s| s.lines().map(ToOwned::to_owned).collect::<Vec<_>>())
                {
                    println!("         {written_line:?}");
                    loop {
                        match lines_iter.next() {
                            None => {
                                eprintln!("Written line {written_line:?} does not match any line in {filename}");
                                exit(3);
                            }
                            Some(read_line) if read_line.trim_end() == written_line.trim_end() => {
                                break
                            }
                            Some(read_line) => {
                                println!("Skipping {read_line:?}");
                            }
                        }
                    }
                }
                for read_line in lines_iter {
                    println!("Skipping {read_line:?}");
                }
            }
        };
    }
}
