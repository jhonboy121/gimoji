import requests
import json
import logging
import os

# Configure logging
logging.basicConfig(level=logging.INFO, format='%(asctime)s - %(levelname)s - %(message)s')

# URL of the OpenMoji API endpoint
url = "https://openmoji.org/data/openmoji.json.gz"

def fetch_openmoji_data(url):
    response = requests.get(url)
    if response.status_code == 200:
        return response.json()
    else:
        logging.error(f"Failed to fetch data from {url}")
        return None

def map_to_schema(emojis_data):
    mapped_emojis = []
    for emoji_data in emojis_data:
        mapped_emojis.append({
            "emoji": emoji_data["emoji"],
            "entity": f"&#x{emoji_data['hexcode']};",
            "code": f":{emoji_data['annotation'].replace(' ', '_').lower()}:",
            "description": emoji_data["annotation"],
            "name": emoji_data["annotation"].replace(' ', '_').lower(),
            "semver": None
        })
    return mapped_emojis

def save_to_json(mapped_emojis, output_file):
    with open(output_file, 'w', encoding='utf-8') as f:
        json.dump({
            "$schema": "https://gitmoji.dev/api/gitmojis/schema",
            "gitmojis": mapped_emojis
        }, f, ensure_ascii=False, indent=4)
    logging.info(f"Emojis saved to {output_file}")

if __name__ == "__main__":
    logging.info("Fetching OpenMoji data...")
    emojis_data = fetch_openmoji_data(url)
    
    if emojis_data:
        logging.info("Mapping emojis to the required schema...")
        mapped_emojis = map_to_schema(emojis_data)
        
        output_file = "emojis.json"
        logging.info(f"Saving mapped emojis to {output_file}...")
        save_to_json(mapped_emojis, output_file)
        logging.info("Emojis fetched, mapped, and saved successfully.")
