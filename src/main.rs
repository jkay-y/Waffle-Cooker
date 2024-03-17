use scraper::{Html, Selector};
use thirtyfour::prelude::*;
use tokio::time::Duration;

#[tokio::main]
async fn main() {

    let mut caps = DesiredCapabilities::firefox();
    caps.set_headless().expect("Failed to set headless");

    let driver = match WebDriver::new("http://localhost:4444", caps).await {
        Ok(driver) => {
            driver
        },
        Err(_) => {
            panic!("Failed to connect to port 4444");
        },
    };

    match driver.goto("https://www.wafflegame.net/daily").await {
        Ok(_) => (),
        Err(_) => {
            panic!("Couldn't connect to wafflegame.net");
        },
    };

    tokio::time::sleep(Duration::from_secs(5)).await;

    let page_html: String;

    match driver.source().await {
        Ok(text) => {
            page_html = text;
            match driver.quit().await {
                Ok(_) => (),
                Err(_) => {
                    panic!("Had trouble quitting");
                },
            };

        },
        Err(_) => {
            panic!("Couldn't retrieve source from driver");
        },
    };

    get_waffle_into(page_html);

}

fn get_waffle_into(waffle_html: String) {
    let page_doc = Html::parse_document(&waffle_html);

    let daily_num_selector = Selector::parse("div.game-number").unwrap();

    for element in page_doc.select(&daily_num_selector) {
        if !element.inner_html().is_empty() {
            println!("{:?}", element.inner_html());
        }
    }

    for x in 0..5 {
        for y in 0..5 {
            if x % 2 != 0 && y % 2 != 0 {
                continue;
            }
            let selector = format!(r#"div[data-pos*="{{\"x\":{},\"y\":{}}}"]"#, x, y);
            let tile_selector = Selector::parse(&selector).unwrap();
            for element in page_doc.select(&tile_selector) {
                if !element.inner_html().is_empty() {
                    println!("Pos ({x}, {y}) - {:?}", element.html());
                }
            }

        }
    }

}
