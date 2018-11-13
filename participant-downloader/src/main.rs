extern crate failure;
extern crate reqwest;
extern crate scraper;

use failure::Error;
use scraper::{Html, Selector};

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
    download_participants().expect("Failed to download the participant list.");
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

        let participant = match (
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
        };

        match participant { 
            Some(p) => println!("{:?}", p), 
            _ => println!("No participant")
        };
    }

    Ok(())
}
