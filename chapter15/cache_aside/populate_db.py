import sys
import sqlite3
import csv
from random import randint

from faker import Faker
fake = Faker()


def setup_db():

    try:
        db = sqlite3.connect('data/quotes.sqlite3')

        # Get a cursor object
        cursor = db.cursor()
        cursor.execute('''
            CREATE TABLE quotes(id INTEGER PRIMARY KEY, text TEXT)
        ''')

        db.commit()
    except Exception as e:
        print(e)
    finally:
        db.close()


def add_quotes(quotes_list):
    quotes = []
    try:
        db = sqlite3.connect('data/quotes.sqlite3')

        cursor = db.cursor()

        quotes = []
        for quote_text in quotes_list:
            quote_id = randint(1, 100)
            quote = (quote_id, quote_text)
            
            try:
                cursor.execute('''INSERT INTO quotes(id, text) VALUES(?, ?)''', quote)
                quotes.append(quote)
            except Exception as e:
                print(f"Error with quote id {quote_id}: {e}")
                
        db.commit()
    except Exception as e:
        print(e)
    finally:
        db.close()

    return quotes


def main():
    args = sys.argv

    if args[1] == 'init':
        setup_db()

    elif args[1] == 'update_db_and_cache':
        quotes_list = [fake.sentence() for _ in range(1, 11)]
        quotes = add_quotes(quotes_list)
        print("New (fake) quotes added to the database:")
        for q in quotes:
            print(f"Added to DB: {q}")

        # Populate the cache with this content
        with open('data/quotes_cache.csv', "a", newline="") as csv_file:
            writer = csv.DictWriter(csv_file, 
                                    fieldnames=['id', 'text'], 
                                    delimiter=";")
            for q in quotes:
                print(f"Adding '{q[1]}' to cache")
                writer.writerow({'id': str(q[0]), 'text': q[1]})

    elif args[1] == 'update_db_only':
        quotes_list = [fake.sentence() for _ in range(1, 11)]
        quotes = add_quotes(quotes_list)
        print("New (fake) quotes added to the database ONLY:")
        for q in quotes:
            print(f"Added to DB: {q}")


if __name__ == "__main__":
    main()
