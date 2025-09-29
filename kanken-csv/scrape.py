# ChatGPT code with a few touches
# to use the right selector and use files
# Originally wanted ChatGPT to create the
# final CSV product but it freaked out and
# offered to create a scrapper instead of told
# me to manually combine the Kanken and Kanji JSON
# file it found on github lol.

from bs4 import BeautifulSoup
import os
import csv
import time

BASE_URL = "https://kanjikana.com/en/kanji/kanken/"

# Levels to scrape
levels = ["10", "9", "8", "7", "6", "5", "4", "3", "2.5", "2"]

kanji_data = []

for level in levels:
    page = 1
    while True:
        file_path = f"html/{level}/page{page}.html"

        if not os.path.exists(file_path):
            print(f"There is no {file_path}.")
            break

        print(f"Scraping Level {level}, Page {page}: {file_path}")
        with open(file_path, 'r', encoding='utf-8') as file:
            file_content = file.read()
        soup = BeautifulSoup(file_content, "html.parser")
        
        # Find all kanji on the page
        kanji_elements = soup.select(".square h3")
        print(f"Found TODO kanji elements")
        if not kanji_elements:
            break  # No more kanji on this page
        
        for k in kanji_elements:
            kanji_data.append((k.text.strip(), level))
        
        page += 1
        time.sleep(0.5)  # Be polite to the server

# Write to CSV
epoch = int(time.time())
csv_file = f"kanken_levels_f{epoch}.csv"
with open(csv_file, mode="w", encoding="utf-8", newline="") as file:
    writer = csv.writer(file)
    writer.writerow(["kanji", "level"])
    writer.writerows(kanji_data)

print(f"Scraping complete! CSV saved as {csv_file}")
