use pdbmol::parser::cif;
use std::{collections::HashMap, env, error::Error, fmt::Display, fs, path::Path};

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
                let res = pdbmol::ccd::Residue::try_from(&d[0].1);
                println!("{:#?}", res);
            }
            Err(e) => println!("{}", e.to_string()),
        }
    } else {
        let cif_path = Path::new("data/ccd-20240406.cif");
        let cif_file = fs::read_to_string(cif_path)?;
        let cif_parsed = cif::parse(&cif_file);

        // This fails at the first residue, 000, because it parses as an int
        // TODO: Store all values as strings and let the schema decode type
        match cif_parsed {
            Err(e) => println!("{}", e.to_string()),
            Ok(d) => {
                let map: Result<HashMap<&'_ str, pdbmol::ccd::Residue>, _> = d
                    .iter()
                    .map(|(key, value)| {
                        println!("loading residue {key}");
                        pdbmol::ccd::Residue::try_from(value).map(|v| (*key, v))
                    })
                    .collect();
                println!("{:#?}", map?)
            }
        };
    }

    Ok(())
}
