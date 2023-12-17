use std::collections::HashSet;

use select::{document::Document, predicate::Name};

fn main() {
    let seed_url: &str = "https://sudeep.live";
    let http_client: reqwest::Client = reqwest::Client::new();
    let _ = surfaceweb_crawl(http_client, seed_url);
}

#[tokio::main]
async fn surfaceweb_crawl(
    http_client: reqwest::Client,
    seed_url: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let res: reqwest::Response = http_client.get(seed_url).send().await?;
    let body: String = res.text().await?;
    let html_document: Document = Document::from(body.as_str());

    let urls: HashSet<String> = html_document
        .find(Name("a"))
        .filter_map(|node: select::node::Node<'_>| node.attr("href"))
        .map(str::to_string)
        .collect::<HashSet<String>>();

    println!("{:#?}", urls);

    Ok(())
}
