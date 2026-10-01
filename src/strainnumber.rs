use std::collections::HashSet;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

/*
Gaurav Sablok
gsablok@proton.me
*/

pub fn strainnumber(path: &str) -> Result<HashSet<String>, Box<dyn Error>> {
    let fileopen = File::open(path).expect("file not found");
    let fileread = BufReader::new(fileopen);
    let mut strain: HashSet<String> = HashSet::new();
    for i in fileread.lines() {
        let line = i.expect("line not found");
        if line.starts_with("\"") || line.starts_with("ID") || line.is_empty() {
            continue;
        } else if !line.starts_with("\"") || !line.starts_with("ID") || !line.is_empty() {
            let lineselect = &line
                .split(",")
                .map(|x| x.replace("\"", ""))
                .collect::<Vec<_>>()[3];
            strain.insert(lineselect.to_string());
            let lineselectadd = &line
                .split(",")
                .map(|x| x.replace("\"", ""))
                .collect::<Vec<_>>()[4];
            if lineselectadd != "0" {
                strain.insert(lineselect.to_string());
            }
        }
    }

    Ok(strain)
}
