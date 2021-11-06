# templo-engine
Template engine of Templo tool for insert and modify variables inside of text files.

 # example
 
 The input text can have some placeholders represented by "{> arg <}". These placeholders will be used to insert 
 the variables passed to compile function. The engine provides some native functions
 to manipulate the variable value. 
 
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
 
 // The variables
 let variables: Vec<templo_engine::Input> = vec![templo_engine::Input {
     key: "class_name".to_string(),
     value: "dog".to_string(),
     value_type: templo_engine::InputValueType::String,
 }];
 
 // Compiling the text
 let text = templo_engine::insert(input_text, &variables);
 
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
