use std::string;

use scraper::{Html, Selector};
use thirtyfour::prelude::*;
use regex::Regex;
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

struct WaffleBoard {
    number: u32,
    tiles: [[WaffleTile; 5]; 5],
}

impl WaffleBoard {
    fn print_board(&self) {
        for y in 0..5 {
            for x in 0..5 {
                match self.tiles[y][x].color {
                    WaffleTileColor::Green => print!("G"),
                    WaffleTileColor::Orange => print!("O"),
                    WaffleTileColor::White => print!("W"),
                    WaffleTileColor::None => print!("N"),
                }
                print!("{},", self.tiles[y][x].letter);
            }
            print!("\n");
        }
    }
}

fn new_board() -> WaffleBoard {
    let temp_tile = WaffleTile {
        letter: 'a',
        color: WaffleTileColor::None,
    };
    let board: WaffleBoard = WaffleBoard {
        number: 0,
        tiles: [
            [temp_tile.clone(), temp_tile.clone(), temp_tile.clone(), temp_tile.clone(), temp_tile.clone()],
            [temp_tile.clone(), temp_tile.clone(), temp_tile.clone(), temp_tile.clone(), temp_tile.clone()],
            [temp_tile.clone(), temp_tile.clone(), temp_tile.clone(), temp_tile.clone(), temp_tile.clone()],
            [temp_tile.clone(), temp_tile.clone(), temp_tile.clone(), temp_tile.clone(), temp_tile.clone()],
            [temp_tile.clone(), temp_tile.clone(), temp_tile.clone(), temp_tile.clone(), temp_tile.clone()]
        ],
    };
    return board;
}

#[derive(Copy, Clone)]
struct WaffleTile {
    letter: char,
    color: WaffleTileColor,
}

impl WaffleTile {

    fn set_color(&mut self, choice: WaffleTileColor) {
        match choice {
            WaffleTileColor::Green => {
                if self.color != WaffleTileColor::Green {
                    self.color = WaffleTileColor::Green;
                }
            },
            WaffleTileColor::Orange => {
                if self.color != WaffleTileColor::Orange {
                    self.color = WaffleTileColor::Orange;
                }
            },
            WaffleTileColor::White => {
                if self.color != WaffleTileColor::White {
                    self.color = WaffleTileColor::White;
                }
            },
            _ => {
                println!("ERROR - set_color() choice was not Green | Orange | White");
            },
        }
    }

    fn set_letter(&mut self, letter_choice: char) {
        if !(self.letter == letter_choice) && letter_choice.is_ascii_uppercase() {
            self.letter = letter_choice;
        }
    }

}

#[derive(Copy, Clone, PartialEq)]
enum WaffleTileColor {
    Green,
    Orange,
    White,
    None
}


fn get_waffle_into(waffle_html: String) {

    let page_doc = Html::parse_document(&waffle_html);

    let mut board = new_board();

    let daily_num_selector = Selector::parse("div.game-number").unwrap();
    for element in page_doc.select(&daily_num_selector) {
        if !element.inner_html().is_empty() {
            let html_text = element.inner_html();
            let num_re = Regex::new(r#"\d+"#).unwrap();
            let num_captures = num_re.captures(&html_text).expect("No match found");
            board.number = num_captures[0].parse().expect("Failed to parse Daily Waffle #");
            println!("{:?}", board.number);
        }
    }

    for y in 0..5 {
        for x in 0..5 {
            if x % 2 != 0 && y % 2 != 0 {
                continue;
            }
            let selector = format!(r#"div[data-pos*="{{\"x\":{},\"y\":{}}}"]"#, x, y);
            let tile_selector = Selector::parse(&selector).unwrap();
            for element in page_doc.select(&tile_selector) {
                let outer_html = element.html();
                if !outer_html.is_empty() {
                    let class_re = Regex::new(r#"class="tile draggable tile--[a-z]( green| yellow)?"#).unwrap();
                    if class_re.is_match(&outer_html) {
                        let class_captures = class_re.captures(&outer_html).expect("No match found");
                        let mut class_name_attribute = class_captures[0].to_string();
                        class_name_attribute = class_name_attribute[28..].to_string();
                        board.tiles[y][x].set_letter(class_name_attribute.chars().nth(0).unwrap().to_ascii_uppercase());
                        match class_name_attribute.len() {
                            1 => {
                                board.tiles[y][x].set_color(WaffleTileColor::White);
                            },
                            _ => {
                                let check_for = class_name_attribute[2..].to_string().to_lowercase();
                                if check_for == "green".to_string() {
                                    board.tiles[y][x].set_color(WaffleTileColor::Green);
                                } else if check_for == "yellow".to_string() {
                                    board.tiles[y][x].set_color(WaffleTileColor::Orange);
                                }
                            },
                        }
                    }
                }
            }
        }
    }

    board.print_board();

}
