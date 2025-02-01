mod book;
mod google_book;

use crate::book::parse_books;
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let input_books = parse_books("database/books.txt")?;
let mut books = Vec::new();
    for ibook in input_books.into_iter().take(2) {
        if let Ok(book) = google_book::get_book(ibook).await {
            books.push(book);
        }
        }
        println!("{books:?}");
    Ok(())
}
