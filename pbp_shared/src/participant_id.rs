#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_yields_none() {
        assert!(parse_id("").is_none());
    }

    #[test]
    fn wrong_length_yields_none() {
        assert!(parse_id("N12345678").is_none());
    }

    #[test]
    fn no_char_yields_none() {
        assert!(parse_id("1234").is_none());
    }

    #[test]
    fn lowercase_char_yields_none() {
        assert!(parse_id("n234").is_none());
    }

    #[test]
    fn invalid_char_yields_none() {
        assert!(parse_id("Å234").is_none());
    }

    #[test]
    fn invalid_number_yields_none() {
        assert!(parse_id("N23x").is_none());
    }
}

/// ```
/// let id = pbp_shared::participant_id::parse_id("N057").unwrap();
/// assert_eq!(id.letter(), 'N');
/// assert_eq!(id.number(), 57);
/// ```
pub fn parse_id(s: &str) -> Option<Id> {
    match s.len() {
        4 => {
            let read_letter = s.chars().next();
            let read_number = s[1..].parse();

            match (read_letter, read_number) {
                (Some(letter), Ok(number)) =>
                    ParticipantIdLetter::new(letter).map(|letter| 
                        Id { 
                            letter, 
                            number: ParticipantIdNumber::new(number)
                        }),
                _ => None
            }
        },
        _ => None
    }    
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct Id {
    letter: ParticipantIdLetter,
    number: ParticipantIdNumber,
}

impl Id {
    pub fn letter(self) -> char { self.letter.0 }
    pub fn number(self) -> u16 { self.number.0 }
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
struct ParticipantIdLetter(char);

impl ParticipantIdLetter {
    fn new(c: char) -> Option<ParticipantIdLetter> {
        match c {
            'A' ... 'Z' => Some(ParticipantIdLetter(c)),
            _ => None
        }
    }
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
struct ParticipantIdNumber(u16);

impl ParticipantIdNumber {
    fn new(i: u16) -> ParticipantIdNumber {
        assert!(i >= 1 && i < 1000);
        ParticipantIdNumber(i)
    }
}
