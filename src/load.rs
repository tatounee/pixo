use std::{io::BufReader, path::Path};

use std::fs::File;
use std::io;

use crate::card::{Card, Tip};
use crate::deck::Deck;

use serde::{de::Visitor, Deserialize, Deserializer};

pub fn load_data_file(path: &Path) -> Result<DataFile, io::Error> {
    let file = File::open(path)?;
    let buf_reader = BufReader::new(file);

    let data_file: DataFile = serde_json::from_reader(buf_reader)?;

    Ok(data_file)
}

#[derive(Deserialize)]
pub struct CardJson {
    #[serde(alias = "qst")]
    #[serde(deserialize_with = "single_or_list")]
    recto: Vec<String>,
    #[serde(alias = "answer")]
    #[serde(deserialize_with = "single_or_list")]
    verso: Vec<String>,
    #[serde(alias = "tips")]
    #[serde(deserialize_with = "tip_to_Tip")]
    #[serde(default = "tip_none")]
    tip: Tip,
    #[serde(default = "vec_empty")]
    tags: Vec<Tag>,
}

#[derive(Deserialize)]
pub struct DataFile {
    questions: Vec<CardJson>,
}

impl From<CardJson> for Card {
    fn from(card_json: CardJson) -> Self {
        Self::new(
            card_json.recto,
            card_json.verso,
            card_json.tip,
            card_json.tags.contains(&Tag::OnlyRecto),
        )
    }
}

impl From<DataFile> for Deck {
    fn from(data_file: DataFile) -> Self {
        Self::new(
            data_file
                .questions
                .into_iter()
                .map(Card::from)
                .collect::<Vec<Card>>(),
        )
    }
}

#[derive(Deserialize, PartialEq)]
#[serde(from = "String")]
pub enum Tag {
    OnlyRecto,
    Unknow(String),
}

impl From<String> for Tag {
    fn from(string: String) -> Self {
        match string.as_str() {
            "only_recto" => Self::OnlyRecto,
            _ => Self::Unknow(string),
        }
    }
}

fn single_or_list<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Vec<String>, D::Error> {
    struct SingleOrlIst;

    impl<'de> Visitor<'de> for SingleOrlIst {
        type Value = Vec<String>;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a string or a list of string")
        }

        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(vec![v.to_owned()])
        }

        fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
        where
            A: serde::de::SeqAccess<'de>,
        {
            let mut list = Vec::new();
            while let Ok(Some(string)) = seq.next_element() {
                list.push(string);
            }

            Ok(list)
        }
    }

    let visitor = SingleOrlIst;
    deserializer.deserialize_any(visitor)
}

#[allow(non_snake_case)]
fn tip_to_Tip<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Tip, D::Error> {
    let mut tips = single_or_list(deserializer)?;

    Ok(match tips.len() {
        0 => Tip::None,
        1 => Tip::One(tips.pop().unwrap()),
        _ => {
            let verso = tips.pop().unwrap();
            let recto = tips.pop().unwrap();
            Tip::RectoVerso(recto, verso)
        }
    })
}

const fn tip_none() -> Tip {
    Tip::None
}

const fn vec_empty<T>() -> Vec<T> {
    Vec::new()
}
