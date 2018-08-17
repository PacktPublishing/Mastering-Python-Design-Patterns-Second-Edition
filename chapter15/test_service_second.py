
from nameko.testing.services import worker_factory
from nameko.standalone.rpc import ClusterRpcProxy
from service_second import PeopleDataPersistenceService

config = {
    'AMQP_URI': "pyamqp://guest:guest@127.0.0.1"
}

def test_peopledata_persist():
    with ClusterRpcProxy(config) as cluster_rpc:
        out = cluster_rpc.people_data_persistence.save.call_async('people.csv')
        print(out.result())

if __name__ == "__main__":
    test_peopledata_persist()