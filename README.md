# Templo Engine

Template engine for insert and modify variables inside of text
files.

# example

The input text can have some placeholders represented by "{> arg <}". These placeholders will be
used to insert the arguments passed to the engine. The engine provides some native functions
to manipulate the argument value as well.

input.py
```py
class {> upper_first(class_name) <}:
    def __init__(self):
    self.name = '{> class_name <}'

obj = {> upper_first(class_name) <}()

print(f'The class name is {obj.name}')

```

## execution

```rust
use templo_engine::*;

// Getting the input text
let input_text = std::fs::read_to_string("./input.py").unwrap();

// The arguments
let arguments = vec![
    EngineArg {
        key: String::from("class_name"),
        value: String::from("dog"),
        value_type: EngineArgType::String,
    }
];

// Inserting the arguments on text
let engine = Engine::new(arguments);
let text = engine.compile(input_text);

// writing the output file
std::fs::write("./output.py", text.unwrap()).unwrap();
```

output.py
```py
class Dog:
    def __init__(self):
    self.name = 'dog'

obj = Dog()

print(f'The class name is {obj.name}')
```
