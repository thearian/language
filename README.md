# Language: a programming language
> this repository is in development and is not recommended to be used.

Language was made to make programming a step easier to learn. It is not made for production purposes and is not recommended to be used in any enviroment.\
`Language` will be transformed to a `python` file. So you can write python in language with no problem.

## Usage
Create a `.lang` file like `my_lang_file.lang` and write your code in it. run the command below to compile.
```
language my_lang_file result_python_file
```
Now you can run your created python file (`result_python_file.py`)./
To run the script right after compile, add `-r` or `--run` flag after the destination file
```
language my_lang_file result_python_file --run
```

## Development
### Run and test
Install `Rust` and `Cargo`.\
To run the app and test, use `cargo run`
```
cargo run
```
### Patterns
The patterns of the language and what it will be compiled to are written with *Regular Expression* in `/patterns` directory.\
Each subject has its own rust file (like `functions.rs`). In the file there is a public `statements()` function which will return a Vector of arrays.\
In the vectors you can write arrays of pattern. The first index is the regular expression of a pattern. And the second index is the defination of that pattern.\
**Easy Example:**
```rust
["then give ", "return "]
```
This code above will translate the patterns with `then give` keyword, to a `return` keyword
Language file:
```
...
then give myname
```
Python file:
```python
return myname
```
**Real Example:**
```rust
[r"untill (?P<t>.+)\r\n", "while not $t:\n\t"]
```
This code above will translate the patterns with `untill` keyword, to a `while` loop
Language file:
```
untill x>0
    x+=1
```
Python file:
```python
while not x>0:
    x+=1
```