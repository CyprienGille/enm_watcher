use reqwest::Client;
use scraper::{Html, Selector};
use std::{thread, time};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::builder().build()?;

    let mut n = 10; // number of elements in the list
    let mut d = 0; // running time

    // Before the results got out, there were 10 bullet points in the concerned list
    while n == 10 {
        // get the page in plain text form
        let response = client
            .get("https://www.enm.justice.fr/espace-candidat/concours-d-acces-2022")
            .send()
            .await?
            .text()
            .await?;

        // parse the HTML
        let document = Html::parse_document(&response);

        // create a selector that will select the list elements we are interested in
        let elems_selector = Selector::parse("ul~h3~ul.ul-triangle>li").unwrap();

        // Select all elements in the results list
        let elems = document.select(&elems_selector);

        n = elems.count(); // update count

        println!("Running depuis {} minutes : RAS.", d);
        d += 5;

        // Sleep for five minutes before re-checking
        // Note: sleeping in an async function should be avoided when there is actual concurrency happening
        let five_minutes = time::Duration::from_secs(300);
        thread::sleep(five_minutes);
    }

    println!("Les résultats sont sortis!");
    Ok(())
}
