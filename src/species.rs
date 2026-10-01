use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

/*
Gaurav Sablok
gsablok@proton.me
*/

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct BacdiveSearchSpecies {
    pub id: String,
    pub species: String,
    pub speciesinformation: String,
}

pub fn bacdivespeciessearch(
    bacdive_analyzer: &str,
    species: Option<String>,
) -> Result<Vec<BacdiveSearchSpecies>, Box<dyn Error>> {
    let mut bachold: Vec<String> = Vec::new();
    let bacdiveanalyzer = File::open(bacdive_analyzer).expect("file not found");
    let bacdivereader = BufReader::new(bacdiveanalyzer);
    for i in bacdivereader.lines() {
        let line = i.expect("line not found");
        if line.starts_with("\"") || line.starts_with("ID") || line.is_empty() {
            continue;
        } else if !line.starts_with("\"") || !line.starts_with("ID") {
            bachold.push(line);
        }
    }
    let mut speciessearch: Vec<BacdiveSearchSpecies> = Vec::new();
    if species.is_some() {
        for i in bachold.iter() {
            let linevec: Vec<_> = i
                .split(",")
                .map(|x| x.replace("\"", ""))
                .collect::<Vec<_>>();
            if linevec[1]
                .split(" ")
                .collect::<Vec<_>>()
                .join(" ")
                .contains(&species.clone().expect("id not found"))
            {
                let idinsert = linevec[0].to_string();
                let speciesinsert = linevec[1].to_string();
                let information = linevec[2..].join("").to_string();
                speciessearch.push(BacdiveSearchSpecies {
                    id: idinsert.clone(),
                    species: speciesinsert.clone(),
                    speciesinformation: information.clone(),
                });
            }
        }
    }
    Ok::<Vec<BacdiveSearchSpecies>, Box<dyn Error>>(speciessearch)
}
