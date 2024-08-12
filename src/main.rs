use pdbmol_ccd::CCD;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    for resname in &args[1..] {
        println!("{:#?}", CCD[resname.as_str()]);
    }
}
