extern crate failure;
extern crate reqwest;
extern crate scraper;

use failure::Error;
use scraper::{ElementRef, Html, Selector};

mod participant_id
{
    use std::fmt;
    
    struct ParticipantIdNumber(u16);

    impl ParticipantIdNumber {
        fn new(i: u16) -> ParticipantIdNumber {
            assert!(i >= 1 && i < 1000);
            ParticipantIdNumber(i)
        }
    }

    impl fmt::Debug for ParticipantIdNumber {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { write!(f, "{}", &self.0) }
    }

    #[derive(Debug)]
    pub struct Id {
        letter: char,
        number: ParticipantIdNumber,
    }

    pub fn parse_id(s: String) -> Option<Id> {
        match s.len() {
            4 => Some(Id { 
                letter: s.chars().next().unwrap(), 
                number: ParticipantIdNumber::new(s[1..].parse().unwrap()) 
            }),
            _ => None
        }    
    }
}

#[derive(Debug)]
struct Participant {
    time: String,
    id: participant_id::Id,
    last_name: String,
    first_name: String,
    country: String,
    gender: String,
    mach: String,
    club: String,
}

fn main() {
    let list = download_participants().expect("Failed to download the participant list.");

    for participant in list {
        println!("{:?}", participant);
    }
}

fn next_cell(row_iter: &mut scraper::element_ref::Select) -> Option<String> {
    row_iter.next().map(|td| td.inner_html())
}

fn download_participants() -> Result<Vec<Participant>, Error> {
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
            match participant_id::parse_id(id_string) {
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
