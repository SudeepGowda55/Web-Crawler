use std::collections::HashSet;

use select::{document::Document, node::Node, predicate::Name};

fn main() {
    let seed_url: &str = "https://matcrypt.com";
    let http_client: reqwest::Client = reqwest::Client::new();
    let _ = surfaceweb_crawl(http_client, seed_url);
}

#[tokio::main]
async fn surfaceweb_crawl(
    http_client: reqwest::Client,
    current_seed_url: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let res: reqwest::Response = http_client.get(current_seed_url).send().await?;
    let body: String = res.text().await?;
    let html_document: Document = Document::from(body.as_str());

    scrape_website(&html_document, current_seed_url);
    get_images(&html_document, current_seed_url);
    get_links(&html_document, current_seed_url);

    Ok(())
}

fn get_links(html_document: &Document, _url: &str) {
    let urls: HashSet<String> = html_document
        .find(Name("a"))
        .filter_map(|node: select::node::Node<'_>| node.attr("href"))
        .map(str::to_string)
        .collect::<HashSet<String>>();

    println!("{:#?}", urls);
}

fn get_images(html_document: &Document, url: &str) {
    for node in html_document.find(Name("img")) {
        if let Some(src) = node.attr("src") {
            println!("Site Url: {}", url);
            println!("Last Time Website Crawled: {}", chrono::Utc::now());
            println!("Image Url: {}", src);
            if let Some(alt_tag) = node.attr("alt") {
                println!("Image Alt: {}", alt_tag);
            }
            println!();
        }
    }
}

fn scrape_website(html_document: &Document, url: &str) {
    println!("Website URL : {}", url);
    let title = html_document
        .find(Name("title"))
        .next()
        .map(|title_node| title_node.text());

    if let Some(title_text) = title {
        println!("Title: {}", title_text);
    }

    // let description_selector = "meta[name=description]";
    // let description = html_document
    //     .find(Name("meta"))
    //     .next()
    //     .and_then(|node| node.attr("content"));

    // match description {
    //     Some(description_text) => println!("Description: {}", description_text),
    //     None => println!("Description not found"),
    // }
}
