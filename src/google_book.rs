use crate::book::Book;
use anyhow::Result;
use serde::Deserialize;
#[derive(Debug, Clone, Deserialize)]
struct SearchResponse {
    items: Vec<BookItem>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
struct BookItem {
    self_link: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
struct VolumeResponse {
    volume_info: VolumeInfo,
}
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
struct VolumeInfo {
    title: String,
    authors: Vec<String>,
    industry_identifiers: Vec<IndustryIdentifier>
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
struct IndustryIdentifier {
    #[serde(rename = "type")]
    isbn_type: ISBNType,
    identifier: String,
}

#[derive(Debug, Clone, Deserialize,PartialEq, Eq)]
enum ISBNType {
    #[serde(rename = "ISBN_13")]
    Isbn13,
    #[serde(rename = "ISBN_10")]
    Isbn10,
}


pub async fn get_book(book: Book) -> Result<Book> {
    let mut book = book;
    let mut url = String::new();
    url.push_str("https://www.googleapis.com/books/v1/volumes?q=");
    for auth in &book.authors {
        url.push_str("+inauthor:");
        url.push_str(auth.as_str());
    }
    url.push_str("+intitle:");
    url.push_str(&book.title.as_str());
    url.push_str("&fields=items/selfLink");

    if let Ok(resp) = reqwest::get(url).await?.json::<SearchResponse>().await {
        let mut url = String::new();
        url.push_str(&resp.items[0].self_link);
        url.push_str("?fields=volumeInfo(title,authors,industryIdentifiers,imageLinks)");
        if let Ok(resp) = reqwest::get(url).await?.json::<VolumeResponse>().await {
            let v = resp.volume_info;
            dbg!(&v);
            book.authors = v.authors;
            book.title = v.title;
            book.isbn13 = v.industry_identifiers.iter().filter(|i| i.isbn_type == ISBNType::Isbn13).map(|i|i.identifier.clone()).next().unwrap_or_default();
        }
    };
    Ok(book)
}
