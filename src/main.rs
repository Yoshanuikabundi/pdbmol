use pdbmol::data::CCD;
use pdbmol::parser::cif;
use std::{env, error::Error, fmt::Display, fs, path::Path};

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    let cif_path = Path::new(&args[1]);
    let cif_file = fs::read_to_string(cif_path)?;
    let cif_parsed = cif::parse(&cif_file);

    match cif_parsed {
        Ok(d) => println!("{:#?}", d),
        Err(e) => println!("{}", e.to_string()),
    }

    // let ccd = CCD.clone();
    // println!("{ccd:?}")

    Ok(())
}
