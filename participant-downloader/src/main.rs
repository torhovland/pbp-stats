extern crate failure;
extern crate reqwest;
extern crate scraper;

use failure::Error;
use scraper::{Html, Selector};

fn main() {
    download_participants().expect("Failed to download the participant list.");
    println!("Participant");
}

fn next_cell(row_iter: &mut scraper::element_ref::Select) -> Option<String> {
    row_iter.next().map(|td| td.inner_html())
}

fn download_participants() -> Result<(), Error> {
    let html = reqwest::get("http://www.paris-brest-paris.org?lang=en&cat=presentation&page=resultats_2015")?
        .text()?;

    let document = Html::parse_document(&html);
    let tr_selector = Selector::parse("tr").unwrap();
    let td_selector = Selector::parse("td").unwrap();

    for tr in document.select(&tr_selector) {
        let mut iter = tr.select(&td_selector);
        let option_time = next_cell(&mut iter);
        let option_id = next_cell(&mut iter);
        let option_last_name = next_cell(&mut iter);
        let option_first_name = next_cell(&mut iter);
        let option_country = next_cell(&mut iter);
        let option_gender = next_cell(&mut iter);
        let option_mach = next_cell(&mut iter);
        let option_club = next_cell(&mut iter);

        match (option_time, option_id, option_last_name, option_first_name, option_country, option_gender, option_mach, option_club) {
            (Some(time), Some(id), Some(last_name), Some(first_name), Some(country), Some(gender), Some(mach), Some(club)) => 
                println!("{} - {} - {} - {} - {} - {} - {} - {}", time, id, last_name, first_name, country, gender, mach, club),
            _ => println!("Skipping row")
        };        
    }

    Ok(())
}
