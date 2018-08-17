
from nameko.testing.services import worker_factory
from service_first import PeopleListService

def test_people():
    service_worker = worker_factory(PeopleListService)
    result = service_worker.populate()
    for name in result:
        print(name)

if __name__ == "__main__":
    test_people()