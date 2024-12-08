use anyhow::{anyhow, Result};
use chrono::{Locale, Month, NaiveDate, NaiveDateTime, Utc};
use std::{fmt::Display, fs::read_to_string, str::FromStr};
use voca_rs::Voca;

fn main() -> Result<()> {
    let books = parse_books("database/books.txt")?;
    Ok(())
}

pub fn parse_books(filename: &str) -> Result<Vec<Book>> {
    let mut books = Vec::new();
    let raw_list = read_to_string(filename)?;
    let mut datetime = Utc::now().naive_utc();
    for (n, line) in raw_list.lines().enumerate() {
        let parsed_line: Result<ParsedLine> = line.parse();
        match parsed_line {
            Ok(parsed_line) => match parsed_line {
                ParsedLine::DateTime(naive_date) => datetime = naive_date,
                ParsedLine::Blank => {}
                ParsedLine::Book(mut book) => {
                    book.datetime = Some(datetime.clone());
                    books.push(book);
                }
            },
            Err(e) => {
                println!("Error on line {}: {e}", n + 1)
            }
        }
    }
    Ok(books)
}

#[derive(Debug, Default, Hash, Clone)]
pub enum ParsedLine {
    DateTime(NaiveDateTime),
    Book(Book),
    #[default]
    Blank,
}

impl FromStr for ParsedLine {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let s = s._trim("");
        if s.is_empty() {
            return Ok(Self::Blank);
        } else if s.contains(" : ") {
            let mut auths_title = s.clone();
            let mut note = 0;
            if s._ends_with(")") {
                auths_title = s._before_last("(")._trim("");
                let note_s = s._after_last("(")._before_last(")");
                note = match note_s.as_str() {
                    "+" => 1,
                    "++" => 2,
                    "+++" => 3,
                    "💙" => 4,
                    "❤️" => 5,
                    _ => 0,
                };
            }
            let auths = auths_title._before(" : ");
            let title = auths_title._after(" : ");
            let authors: Vec<String> = auths._split("&").iter().map(|s| s._trim("")).collect();
            let isbn13 = "".into();
            return Ok(Self::Book(Book {
                authors,
                title,
                isbn13,
                note,
                datetime: None,
            }));
        } else {
            let month_fr = s._before(" ")._trim("")._lower_case()._slugify();
            let year_s = s._after(" ")._trim("");
            let year = year_s.parse();
            let year = match year {
                Ok(i) => i,
                Err(e) => {
                    println!("{e}\n{year_s}");
                    2020
                }
            };
            let month = match month_fr.as_str() {
                "janvier" => Month::January,
                "fevrier" => Month::February,
                "mars" => Month::March,
                "avril" => Month::April,
                "mai" => Month::May,
                "juin" => Month::June,
                "juillet" => Month::July,
                "aout" => Month::August,
                "septembre" => Month::September,
                "octobre" => Month::October,
                "novembre" => Month::November,
                "decembre" => Month::December,
                _ => Month::January,
            };
            let date = NaiveDate::from_ymd_opt(year, month.number_from_month(), 1)
                .ok_or(anyhow!("Error while parsing date"))?;
            let datetime = date
                .and_hms_opt(0, 0, 0)
                .ok_or(anyhow!("Error while parsing midnight"))?;
            return Ok(Self::DateTime(datetime));
        }
    }
}

impl Display for ParsedLine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParsedLine::DateTime(datetime) => {
                let formatted_datetime = format!(
                    "{}",
                    datetime
                        .and_utc()
                        .format_localized("%B %Y", Locale::fr_BE_euro)
                )
                ._capitalize(true);
                write!(f, "{formatted_datetime}")
            }
            ParsedLine::Book(book) => {
                write!(f, "{} : {}", book.authors.join(" & "), book.title)?;
                if book.note > 0 {
                    let note = match book.note {
                        1 => "+",
                        2 => "++",
                        3 => "+++",
                        4 => "💙",
                        5 => "❤️",
                        _ => "",
                    };
                    write!(f, " ({note})")?;
                }
                write!(f, "")
            }
            ParsedLine::Blank => {
                write!(f, "")
            }
        }
    }
}

#[derive(Debug, Default, Clone, Hash)]
pub struct Book {
    pub authors: Vec<String>,
    pub title: String,
    pub isbn13: String,
    pub note: u8,
    pub datetime: Option<NaiveDateTime>,
}
