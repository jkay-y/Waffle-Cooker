
use regex::Regex;
use scraper::{Html, Selector};
use thirtyfour::prelude::*;
use tokio::time::Duration;

#[tokio::main]
async fn main() {

    println!("[DEBUG] Inside main()");

    match select_a_waffle(None).await {
        _ => (),
    };

}

enum WCExceptionCodes {
    WCTEMP(String),
}

async fn select_a_waffle(waffle_num: Option<u32>) -> Result<WaffleBoard, WCExceptionCodes> {

    println!("[DEBUG] Inside select_a_waffle()");
    
    let mut firefox_capabilities = DesiredCapabilities::firefox();
    match firefox_capabilities.set_headless() {
        Ok(_) => println!("[DEBUG] select_a_waffle() - FirefoxCapabilities.set_headless()"),
        Err(_) => {
            println!("[ERROR] select_a_waffle() - FirefoxCapabilities.set_headless() failed");
            return Err(WCExceptionCodes::WCTEMP("Error when doing set_headless()".to_string()));
        },
    }

    let web_driver: WebDriver = match WebDriver::new("http://localhost:4444", firefox_capabilities).await {
        Ok(web_driver) => {
            println!("[DEBUG] select_a_waffle() - Initialised WebDriver::new()");
            web_driver
        },
        Err(_) => {
            println!("[ERROR] select_a_waffle() - WebDriver::new()");
            return Err(WCExceptionCodes::WCTEMP("Error when initialising WebDriver".to_string()));
        },
    };

    let url: String;
    match waffle_num {
        Some(num) => {
            println!("[DEBUG] select_a_waffle() - Grabbing Waffle #{}", num);
            url = "https://www.wafflegame.net/archive".to_string();
        },
        None => {
            println!("[DEBUG] select_a_waffle() - Getting Daily Waffle");
            url = "https://www.wafflegame.net/daily".to_string();
        },
    }

    match web_driver.goto(&url).await {
        Ok(_) => {
            println!("[DEBUG] select_a_waffle() - WebDriver connected to {}", url);
            match waffle_num {
                Some(num) => {
                    tokio::time::sleep(Duration::from_secs(2)).await;
                    let css_string = format!("div[data-id='{}']", num);
                    let div_element = match web_driver.find(By::Css(&css_string)).await {
                        Ok(elem) => {
                            println!("[DEBUG] select_a_waffle() - Found div with data-id={}", num);
                            elem
                        },
                        Err(_) => {
                            println!("[ERROR] select_a_waffle() - Couldn't find specified Waffle");
                            return Err(WCExceptionCodes::WCTEMP("Couldn't find specified Waffle".to_string()));
                        },
                    };
                    match div_element.click().await {
                        Ok(_) => println!("[DEBUG] select_a_waffle() - Executed click()"),
                        Err(_) => {
                            println!("[ERROR] select_a_waffle() - Failed to execute click() on div");
                            return Err(WCExceptionCodes::WCTEMP("Failed to execute click() on div".to_string()));
                        },
                    }
                },
                _ => (),
            }
        },
        Err(_) => {
            println!("[ERROR] select_a_waffle() - WebDriver failed to connect to {}", url);
            match quit_driver(web_driver).await {
                _ => (),
            };
            return Err(WCExceptionCodes::WCTEMP("WebDriver failed to connect to {url}".to_string()));
        },
    };


    let page_html: String = match web_driver.source().await {
        Ok(text) => {
            match quit_driver(web_driver).await {
                _ => (),
            }
            println!("[DEBUG] select_a_waffle() - Retrieved wafflegame.net HTML from WebDriver");
            text
        }
        Err(_) => {
            match quit_driver(web_driver).await {
                _ => (),
            }
            println!("[ERROR] select_a_waffle() - Failed to retrieve wafflegame.net HTML from WebDriver");
            return Err(WCExceptionCodes::WCTEMP("Failed to retrieve wafflegame.net HTML".to_string()));
        },
    };
    
    return waffle_html_to_board(page_html);

}

async fn quit_driver(driver: WebDriver) {
    match driver.quit().await {
        Ok(_) => {
            println!("[DEBUG] quit_driver() - Successfully closed driver");
        },
        Err(_) => {
            println!("[ERROR] Failed to quit driver");
        },
    };
}

struct WaffleBoard {
    number: u32,
    tiles: [[WaffleTile; 5]; 5],
}

impl WaffleBoard {
    fn print_board(&self) {
        println!("[DEBUG] print_board()");
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

fn new_board() -> WaffleBoard {
    println!("[DEBUG] Entering new_board()");
    let temp_tile :WaffleTile = WaffleTile {
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

fn waffle_html_to_board(waffle_html: String) -> Result<WaffleBoard, WCExceptionCodes> {

    println!("[DEBUG] Inside waffle_html_to_board()");

    let page_doc = Html::parse_document(&waffle_html);

    let mut board = new_board();

    let daily_num_selector = Selector::parse("div.game-number").unwrap();
    for element in page_doc.select(&daily_num_selector) {
        if !element.inner_html().is_empty() {
            let html_text = element.inner_html();
            let num_re = match Regex::new(r#"\d+"#) {
                Ok(re_entity) => {
                    println!("[DEBUG] waffle_html_to_board() - Regex::new() completed successfully");
                    re_entity
                },
                Err(_) => {
                    println!("[ERROR] waffle_html_to_board() - Regex::new() initialiser failed");
                    return Err(WCExceptionCodes::WCTEMP("Regex::new() initialiser failed".to_string()));
                },
            };
            match num_re.captures(&html_text) {
                Some(captures) => {
                    board.number = match captures[0].parse() {
                        Ok(num) => {
                            println!("[DEBUG] waffle_html_to_board() - Daily Waffle #{}", num);
                            num
                        },
                        Err(_) => {
                            println!("[ERROR] waffle_html_to_board() - Failed to parse Daily Waffle #");
                            return Err(WCExceptionCodes::WCTEMP("parse to u8 failed on Daily Waffle #".to_string()));
                        },
                    };
                },
                None => {
                    println!("[ERROR] waffle_html_to_board() - No match for div.game-number found");
                    return Err(WCExceptionCodes::WCTEMP("div.game-number wasn't found".to_string()));
                }
            }
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
                    let class_re = match Regex::new(r#"class="tile draggable tile--[a-z]( green| yellow)?"#) {
                        Ok(re_entity) => {
                            println!("[DEBUG] waffle_html_to_board() - Regex::new() initialised for class=\"tile draggable tile\"");
                            re_entity
                        },
                        Err(_) => {
                            println!("[ERROR] waffle_html_to_board() - Regex::new() failed for class=\"tile draggable tile\"");
                            return Err(WCExceptionCodes::WCTEMP("Regex::new() failed for class=\"tile draggable tile\"".to_string()));
                        },
                    };
                    if class_re.is_match(&outer_html) {
                        let class_captures = match class_re.captures(&outer_html) {
                            Some(captures) => {
                                println!("[DEBUG] waffle_html_to_board() - Matches found for class=\"tile draggable tile\"");
                                captures
                            },
                            None => {
                                println!("[ERROR] waffle_html_to_board() - No match found for class=\"tile draggable tile\"");
                                return Err(WCExceptionCodes::WCTEMP("No match found for class=\"tile draggable tile\"".to_string()));
                            },
                        };
                        let mut class_name_attribute = class_captures[0].to_string();
                        class_name_attribute = class_name_attribute[28..].to_string();
                        match class_name_attribute.chars().nth(0) {
                            Some(result_char) => {
                                println!("[DEBUG] waffle_html_to_board() - Tile at ({}, {}) has letter {}", x, y, result_char);
                                board.tiles[y][x].set_letter(result_char.to_ascii_uppercase());
                            },
                            None => {
                                println!("[ERROR] waffle_html_to_board() - Failed to get the first index of capture");
                                return Err(WCExceptionCodes::WCTEMP("Failed to get the first index of capture".to_string()));
                            },
                        }
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

    return Ok(board);

}
