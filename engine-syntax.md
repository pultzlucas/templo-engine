# Engine Functions

The Templo engine provides you some native functions to manipulate the value of a placeholder.

# placeholders

````py
class {> upper_first(class_name) <}:
    def __init__(self):
        self.name = '{> class_name <}'

obj = {> upper_first(class_name) <}()

print(f'The class name is {obj.name}')
````

Each "{> arg <}" statement is considered a placeholder.

# functions

All the functions follows the below syntax.

```
{> FUNCTION_NAME(EXPRESSIONS,...) <}
```
Functions can expect more than one argument and you can call functions inside of another functions.

The syntax is simple if you already uses a language based in C/C++.

# strings

Strings is a text between quotes (').

````
{> 'Good Life!' <}

{> upper('Rust ❤') <}
````

Double quotes (") is not considered.



