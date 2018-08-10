from pyparsing import Word, OneOrMore, Optional, Group, Suppress, alphanums  
        
class Boiler:  
    def __init__(self):  
        self.temperature = 83# in celsius 
 
    def __str__(self):  
        return f'boiler temperature: {self.temperature}'  
 
    def increase_temperature(self, amount):  
        print(f"increasing the boiler's temperature by {amount} degrees")  
        self.temperature += amount  
 
    def decrease_temperature(self, amount):  
        print(f"decreasing the boiler's temperature by {amount} degrees")  
        self.temperature -= amount
         
word = Word(alphanums)  
command = Group(OneOrMore(word))  
token = Suppress("->")  
device = Group(OneOrMore(word))  
argument = Group(OneOrMore(word))  
event = command + token + device + Optional(token + argument)  

boiler = Boiler()  
print(boiler)  

cmd, dev, arg = event.parseString('increase -> boiler temperature -> 3 degrees')
cmd_str = ' '.join(cmd)
dev_str = ' '.join(dev)

if 'increase' in cmd_str and 'boiler' in dev_str:
    boiler.increase_temperature(int(arg[0]))  
    
print(boiler)
