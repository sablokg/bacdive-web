use reqwest::blocking::get;
use scraper::{Html, Selector};
use std::error::Error;

/*
Gaurav Sablok
codeprog@icloud.com
*/

pub fn webminer(id: &str) -> Result<String, Box<dyn Error>> {
    let idnumber = id.parse::<usize>().unwrap();
    let formatstring_download = format!("{}/{}", "https://bacdive.dsmz.de/strain", idnumber);
    let infobox = get(&formatstring_download).expect("string not found");
    let document = Html::parse_document(&infobox.text().expect("message not present"));
    let snpselect = Selector::parse(".infobox_key").expect("method failed");
    for element in document.select(&snpselect) {
        let vector_1 = element.text().collect::<Vec<_>>().join("-");
        println!("{}", vector_1);
    }
    Ok("The webmine results are as follows".to_string())
}

/// Same scrape as `webminer`, but returns the results instead of printing
/// them to stdout, so callers such as the web server can render them.
pub fn webminer_capture(id: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let idnumber = id.parse::<usize>()?;
    let formatstring_download = format!("{}/{}", "https://bacdive.dsmz.de/strain", idnumber);
    let infobox = get(&formatstring_download)?;
    let document = Html::parse_document(&infobox.text()?);
    let snpselect = Selector::parse(".infobox_key").expect("method failed");
    let mut results: Vec<String> = Vec::new();
    for element in document.select(&snpselect) {
        let vector_1 = element.text().collect::<Vec<_>>().join("-");
        results.push(vector_1);
    }
    Ok(results)
}
