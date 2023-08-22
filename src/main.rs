#[tokio::main]
async fn main() {
    let resp = reqwest::get("https://www.example.com").await.unwrap();
    assert!(resp.status().is_success());

    let body = resp.text().await.unwrap();

    let fragment = scraper::Html::parse_document(&body);
    let selector = scraper::Selector::parse("a[href]").unwrap();
    for element in fragment.select(&selector) {
        let href = element.value().attr("href").unwrap();
        println!("{:?}", href);
    }
}