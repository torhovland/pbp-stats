use failure::Error;
use reqwest;
use scraper::{element_ref, ElementRef, Html, Selector};
use participant_id;

#[derive(Debug, Serialize, Deserialize)]
pub struct Participant {
    time: String,
    id: participant_id::Id,
    last_name: String,
    first_name: String,
    country: String,
    gender: String,
    mach: String,
    club: String,
}

pub fn download_participants() -> Result<Vec<Participant>, Error> {
    let html = reqwest::get("http://www.paris-brest-paris.org?lang=en&cat=presentation&page=resultats_2015")?
        .text()?;

    let document = Html::parse_document(&html);
    let tr_selector = Selector::parse("tr").unwrap();

    let list = document
        .select(&tr_selector)
        .filter_map(|tr| parse_participant_row(tr))
        .collect();

    Ok(list)
}

fn next_cell(row_iter: &mut element_ref::Select) -> Option<String> {
    row_iter.next().map(|td| td.inner_html())
}

fn parse_participant_row(tr: ElementRef<'_>) -> Option<Participant> {
    let td_selector = Selector::parse("td").unwrap();
    let mut iter = tr.select(&td_selector);

    match (
        next_cell(&mut iter), 
        next_cell(&mut iter), 
        next_cell(&mut iter), 
        next_cell(&mut iter), 
        next_cell(&mut iter), 
        next_cell(&mut iter), 
        next_cell(&mut iter), 
        next_cell(&mut iter)) {
        (
            Some(time), 
            Some(id_string),
            Some(last_name), 
            Some(first_name), 
            Some(country), 
            Some(gender), 
            Some(mach), 
            Some(club)
        ) => 
            match participant_id::parse_id(&id_string) {
                Some(id) => 
                    Some(Participant {
                        time,
                        id,
                        last_name,
                        first_name,
                        country,
                        gender,
                        mach,
                        club
                    }),
                _ => None
            },
        _ => None
    }
}
