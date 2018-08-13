from rx import Observable

def firstnames_from_db(file_name):
    file = open(file_name)

    # collect and push stored people firstnames
    return Observable.from_(file) \
        .flat_map(lambda content: content.split(', ')) \
        .filter(lambda name: name!='') \
        .map(lambda name: name.split()[0]) \
        .group_by(lambda firstname: firstname) \
        .flat_map(lambda grp: grp.count().map(lambda ct: (grp.key, ct)))

db_file = "people.txt"

# Emit data every 5 seconds
Observable.interval(5000) \
    .flat_map(lambda i: firstnames_from_db(db_file)) \
    .subscribe(lambda value: print(str(value)))

# Keep alive until user presses any key
input("Starting... Press any key to quit\n")
