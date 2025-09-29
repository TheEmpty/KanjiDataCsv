use serde::Deserialize;
use std::collections::HashMap;
use std::fs::{self, File};

#[derive(Deserialize, Debug)]
struct KanjiInfo {
    #[serde(rename(deserialize = "jlpt_new"))]
    jlpt: Option<u8>,
    #[serde(rename(deserialize = "wk_level"))]
    wanikani_level: Option<u8>,
    grade: Option<u8>,
    freq: Option<u16>,

    #[serde(rename(deserialize = "readings_kun"))]
    kunyomi: Option<Vec<String>>,
    #[serde(rename(deserialize = "readings_on"))]
    onyomi: Option<Vec<String>>,
    meanings: Option<Vec<String>>,
}

fn opt_to_str<T: std::fmt::Display>(val: &Option<T>, prefix: &str) -> String {
    match val {
        Some(x) => format!("{prefix}{x}"),
        None => String::new(),
    }
}

fn opt_vec_to_str(val: &Option<Vec<String>>) -> String {
    val.as_ref().map(|x| x.join(",")).unwrap_or_default()
}

impl KanjiInfo {
    fn to_csv(&self, kanji: &str, kanken: u8) -> String {
        let jlpt = opt_to_str(&self.jlpt, "N");
        let wanikani = opt_to_str(&self.wanikani_level, "WK");
        let grade = opt_to_str(&self.grade, "G");
        let frequency = opt_to_str(&self.freq, "FQ");
        let kunyomi = opt_vec_to_str(&self.kunyomi);
        let onyomi = opt_vec_to_str(&self.onyomi);
        let meanings = opt_vec_to_str(&self.meanings);

        format!(
            "{kanji};{kanken};{jlpt};{wanikani};{grade};{frequency};{kunyomi};{onyomi};{meanings}"
        )
    }
}

#[derive(serde::Deserialize)]
struct KankenInfo {
    kanji: String,
    level: u8,
}

fn main() {
    let file_path = "kanji.json";
    let file_contents = fs::read_to_string(file_path).expect("Unable to read JSON file");

    let general_kanji_json: HashMap<String, KanjiInfo> =
        serde_json::from_str(&file_contents).expect("Unable to parse JSON");

    println!("kanji;kanken;jlpt;wanikani;grade;frequency;kunyomi;onyomi;meanings;");

    let csv_file = File::open("kanken.csv").expect("Unable to find CSV file");
    let mut reader = csv::Reader::from_reader(csv_file);
    for row in reader.deserialize() {
        let kanken_info: KankenInfo = row.expect("Failed to parse row");
        let kanji = kanken_info.kanji;
        let kanji_info = general_kanji_json
            .get(&kanji)
            .unwrap_or_else(|| panic!("Do not have kanji info for {kanji}"));
        println!("{}", kanji_info.to_csv(&kanji, kanken_info.level));
    }
}
