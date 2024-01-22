import requests
from pymongo import MongoClient
import json

uri = "mongodb://root:example@172.20.0.4:27017"

client = MongoClient(uri)

db = client['beer_database']

collection = db['beers']

json_url = "https://raw.githubusercontent.com/Stefen-Taime/open-source-data/main/beer/json/json_bank_20240116_1.json"

response = requests.get(json_url)
data = response.json()

collection.insert_many(data)

print("Data inserted successfully!")

client.close()
