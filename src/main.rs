use pdbmol_ccd::data::Residue;
use pdbmol_cif as cif;
use std::{collections::HashMap, env, error::Error, fs, path::Path};

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() == 2 {
        println!("{:?}", args);
        let cif_path = Path::new(&args[1]);
        let cif_file = fs::read_to_string(cif_path)?;
        let cif_parsed = cif::parse(&cif_file);

        match cif_parsed {
            Ok(d) => {
                println!("{:#?}", d);
                let res = Residue::try_from(&d.into_iter().next().ok_or("no datablock")?.1);
                println!("{:#?}", res);
            }
            Err(e) => println!("{}", e.to_string()),
        }
    } else {
        let cif_path = Path::new("data/ccd-20240406.cif");
        let cif_file = fs::read_to_string(cif_path)?;
        let cif_parsed = cif::parse(&cif_file);

        match cif_parsed {
            Err(e) => println!("{}", e.to_string()),
            Ok(d) => {
                let map: Result<HashMap<&'_ str, Residue>, _> = d
                    .iter()
                    .filter(|(key, _)| !["UNL"].contains(key)) // Filter out special residues
                    .map(|(key, value)| {
                        println!("loading residue {key}");
                        Residue::try_from(value).map(|v| (*key, v))
                    })
                    .collect();
                println!("{:#?}", map?)
            }
        };
    }

    Ok(())
}
