# Templo Engine

Template engine of Templo tool for insert and modify variables inside of text
files.

# example

The input text can have some placeholders represented by "{> arg <}". These
placeholders will be used to insert the arguments passed to the engine. The
engine provides some native functions to manipulate the argument value.

input.py

```py
class {> upper_first(class_name) <}:
    def __init__(self):
    self.name = '{> class_name <}'

obj = {> upper_first(class_name) <}()

print(f'The class name is {obj.name}')
```

## execution

``` rust
// Getting the input text
let input_text = std::fs::read_to_string("./input.py").unwrap();

// The arguments
let arguments: Vec<templo_engine::EngineArg> = vec![
    templo_engine::EngineArg {
        key: "class_name".to_string(),
        value: "dog".to_string(),
        value_type: templo_engine::EngineArgType::String,
    }
];

// Inserting the arguments on text
let engine = templo_engine::Engine::new(arguments);
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
