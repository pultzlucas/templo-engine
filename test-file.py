class {> upper_first(class_name) <}:
    def __init__(self):
        self.name = '{> class_name <}'

obj = {> upper_first(class_name) <}()

print(f'The class name is {obj.name}')