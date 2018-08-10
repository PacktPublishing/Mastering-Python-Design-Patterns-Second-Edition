import os  
verbose = True  

class CreateFile:

    def __init__(self, path, txt='hello world\n'):  
        self.path = path 
        self.txt = txt 

    def execute(self):  
        if verbose:  
            print(f"[creating file '{self.path}']")  
        with open(self.path, mode='w', encoding='utf-8') as out_file:  
            out_file.write(self.txt)
            
    def undo(self):  
        try:
            delete_file(self.path)
        except:
            print('delete action not successful...')
            print('... file was probably already deleted.')

def delete_file(path):
    if verbose:
        print(f"deleting file {path}...")
    os.remove(path)
     
def main():

    orig_name = 'file1'  
    df=delete_file  

    commands = [CreateFile(orig_name),] 
    commands.append(df)  
  
    for c in commands:  
        try:  
            c.execute()  
        except AttributeError as e:  
            df(orig_name)  
  
    for c in reversed(commands):  
        try:  
            c.undo()  
        except AttributeError as e:  
            pass
            
if __name__ == "__main__":  
    main()
    