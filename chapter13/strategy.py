import time

def pairs(seq): 
    n = len(seq) 
    for i in range(n): 
        yield seq[i], seq[(i + 1) % n] 

SLOW = 3                        # in seconds 
LIMIT = 5                       # in characters 
WARNING = 'too bad, you picked the slow algorithm :(' 
 
def allUniqueSort(s): 
   if len(s) > LIMIT: 
       print(WARNING) 
       time.sleep(SLOW) 
   srtStr = sorted(s) 
   for (c1, c2) in pairs(srtStr): 
       if c1 == c2: 
           return False 
   return True 

def allUniqueSet(s): 
    if len(s) < LIMIT: 
        print(WARNING) 
        time.sleep(SLOW) 
     
    return True if len(set(s)) == len(s) else False
    
def allUnique(word, strategy):    
    return strategy(word) 

def main(): 

    WORD_IN_DESC = 'Insert word (type quit to exit)> '
    STRAT_IN_DESC = 'Choose strategy: [1] Use a set, [2] Sort and pair> '

    while True: 
        word = None 
        while not word: 
            word = input(WORD_IN_DESC) 
 
            if word == 'quit': 
                print('bye') 
                return 
 
            strategy_picked = None 
            strategies = { '1': allUniqueSet, '2': allUniqueSort } 
            while strategy_picked not in strategies.keys(): 
                strategy_picked = input(STRAT_IN_DESC)
 
                try: 
                    strategy = strategies[strategy_picked] 
                    result = allUnique(word, strategy)
                    print(f'allUnique({word}): {result}') 
                except KeyError as err: 
                    print(f'Incorrect option: {strategy_picked}') 

if __name__ == "__main__":
    main()
