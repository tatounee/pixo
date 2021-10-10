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
    recto: String,
    #[serde(alias = "answer")]
    verso: String,
    #[serde(deserialize_with = "tip_to_Tip")]
    #[serde(default = "tip_none")]
    tip: Tip,
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

#[allow(non_snake_case)]
fn tip_to_Tip<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Tip, D::Error> {
    struct TipToTip;

    impl<'de> Visitor<'de> for TipToTip {
        type Value = Tip;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a string or a list of string")
        }

        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(Tip::One(v.to_owned()))
        }

        fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
        where
            A: serde::de::SeqAccess<'de>,
        {
            let first = seq.next_element();
            let second = seq.next_element();

            if let Ok(Some(tip_recto)) = first {
                if let Ok(Some(tip_verso)) = second {
                    Ok(Tip::RectoVerso(tip_recto, tip_verso))
                } else {
                    Ok(Tip::One(tip_recto))
                }
            } else {
                Ok(Tip::None)
            }
        }
    }

    let visitor = TipToTip;
    deserializer.deserialize_any(visitor)
}

const fn tip_none() -> Tip {
    Tip::None
}
