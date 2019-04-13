use std::sync::Arc;

use chrono::naive::NaiveDate;
use failure::Error;
use reqwest::Client;
use select::document::Document;
use select::predicate::{Attr, Class, Name};
use xvii::Roman;

pub const BASE_URL: &str = "https://www.swordscomic.com";

#[derive(Debug)]
pub struct Comic {
    id: u32,
    title: String,
    date: NaiveDate,
    image: String,
    characters: Vec<String>,
    locations: Vec<String>,
    creatures: Vec<String>,
    swords: Vec<String>
}
//struct Character {}
//struct Creature {}
//struct Location {}
//struct Sword {}

pub struct SwordsComic {
    client: Arc<Client>,
}

impl Default for SwordsComic {
    fn default() -> Self {
        Self {
            client: Arc::new(Client::new()),
        }
    }
}

impl SwordsComic {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn from_client(client: Client) -> Self {
        Self {
            client: Arc::new(client),
        }
    }


    pub fn get_comic(&self, id: u32) -> Result<Comic, Error> {
        let roman_no = match Roman::from(id as i32) {
            Some(roman) => roman.to_uppercase(),
            None => String::from("cover"),
        };

        let mut resp = self.client.get(format!("{}/swords/{}", BASE_URL, roman_no).as_str()).send()?.error_for_status()?;
        let dom = Document::from(resp.text()?.as_str());

        let title = dom.find(Attr("id", "comic-title"))
            .next()
            .unwrap()
            .text();

        let date = dom.find(Attr("id", "comic-post-date"))
            .next()
            .unwrap()
            .text();
        let date = NaiveDate::parse_from_str(date.as_str(), "%B %e, %Y")?;

        let image = dom.find(Attr("id", "comic-image"))
            .next()
            .unwrap()
            .attr("src")
            .unwrap()
            .to_string();

        macro_rules! create_vec {
            ($vec_name:ident) => (
                let mut $vec_name: Vec<String> = Vec::new();
            )
        }

        create_vec!(characters);
        create_vec!(creatures);
        create_vec!(swords);
        create_vec!(locations);


        for tag_group in dom.find(Class("tag-group")) {
            macro_rules! iter_on_tag_group_and_append {
                ($var_name:ident) => (
                    for tag in tag_group.find(Class("tag")) {
                        $var_name.push(String::from(tag.text().as_str().trim()));
                    }
                )
            }

            let category = tag_group.find(Name("p")).next().unwrap().text();
            if category == "Characters:" {
                iter_on_tag_group_and_append!(characters);
            } else if category == "Locations:" {
                iter_on_tag_group_and_append!(locations);
            } else if category == "Swords:" {
                iter_on_tag_group_and_append!(swords);
            } else if category == "Creatures:" {
                iter_on_tag_group_and_append!(creatures);
            }
        }

        let comic = Comic {
            id,
            title,
            date,
            image,
            characters,
            creatures,
            swords,
            locations,
        };
        Ok(comic)
    }
}