# Example of a singleton design pattern
class single:
    # instance is a class variable/attribute and not
    # an instance variable/attribute
    instance = None

    def getinstance():
        if single.instance is None:
            # This creates a new object, and hence calls __init__
            # single()
            return single()

        # single.instance will refer to the object of the class
        return single.instance

    def __init__(self):
        # If the only object/instance of the class
        # already exists, then raise an exception
        if single.instance is not None:
            raise Exception("this is single")
        else:
            # The class variable/attribute will point to the
            # only object/instance of the class
            single.instance = self


s = single.__new__(single)

# single.__init__(s)  # Same as the function call below it
s.__init__()

# s = single()  # Causes an error
# print(s)

# s1 = single()  # Causes an error

s = single.getinstance()
print(s)

s = single.getinstance()
print(s)
