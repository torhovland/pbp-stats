extern crate reqwest;

fn main() {
    let participant = download_participants().unwrap();
    println!("Participant: {}", participant);
}

fn download_participants() -> Result<String, reqwest::Error> {
    let body = reqwest::get("http://www.paris-brest-paris.org?lang=en&cat=presentation&page=resultats_2015")?.text();
    body
}