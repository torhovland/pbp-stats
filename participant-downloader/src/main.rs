extern crate failure;
extern crate reqwest;
extern crate scraper;

use failure::Error;
use scraper::{Html, Selector};

fn main() {
    download_participants().expect("Failed to download the participant list.");
    println!("Participant");
}

fn download_participants() -> Result<(), Error> {
    let html = reqwest::get("http://www.paris-brest-paris.org?lang=en&cat=presentation&page=resultats_2015")?
        .text()?;

    let document = Html::parse_document(&html);
    let tr_selector = Selector::parse("tr").unwrap();
    let td_selector = Selector::parse("td").unwrap();

    for tr in document.select(&tr_selector) {
        println!("tr");

        for td in tr.select(&td_selector) {
            println!("{}", td.inner_html());        
        }
    }

    Ok(())
}
