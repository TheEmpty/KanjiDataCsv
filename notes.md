Kanji.json from https://github.com/davidluzgouveia/kanji-data. Format as follows

```
"勝": {
    "strokes": 12,
    "grade": 3,
    "freq": 185,
    "jlpt_old": 2,
    "jlpt_new": 3,
    "meanings": ["Victory","Win","Prevail","Excel"],
    "readings_on": ["しょう"],
    "readings_kun": ["か.つ","-が.ち","まさ.る","すぐ.れる","かつ"],
    "wk_level": 9,
    "wk_meanings": ["Win"],
    "wk_readings_on": ["しょう"],
    "wk_readings_kun": ["!か"],
    "wk_radicals": ["Moon","Gladiator","Power"]
}
```

Fields can be null. `!` means not accepted. `^` means non-primary reading.

`$ cargo run > output.csv` 
