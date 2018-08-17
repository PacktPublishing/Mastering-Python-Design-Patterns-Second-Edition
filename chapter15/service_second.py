
from nameko.rpc import rpc, RpcProxy
from faker import Faker
import csv

fake = Faker()

class PeopleListService:

    name = 'peoplelist'

    @rpc
    def populate(self, number=20):

        persons = []
        for _ in range(0, number):
            p = {'firstname': fake.first_name(), 
                 'lastname': fake.last_name(), 
                 'address': fake.address()}
            persons.append(p)

        return persons


class PeopleDataPersistenceService:

    name = 'people_data_persistence'    
    peoplelist_rpc = RpcProxy('peoplelist')

    @rpc
    def save(self, filename):
        persons = self.peoplelist_rpc.populate(number=25)

        with open(filename, "a", newline="") as csv_file:
            fieldnames = ["firstname", "lastname", "address"]
            writer = csv.DictWriter(csv_file, 
                                    fieldnames=fieldnames, 
                                    delimiter=";")

            for p in persons:
                writer.writerow(p)

        return f"Saved data for {len(persons)} new people"