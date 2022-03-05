# Language: a programming language
> this repository is in development and is not recommended to be used.

Langauge is a programming language simplifier. It uses an open source one-to-one regex language map  
It was made to make programming a step easier to learn. And it's major milestone is to make a universal
programming language that doesn't need learning.

## Usage
Current version's help:
```
language 0.0.1
Arian Mirahmadi thearian@github mirarianmir@gmail.com
Programming language simplifier

USAGE:
    language.exe [OPTIONS] <FILE>

ARGS:
    <FILE>    Input file with .lang extention

OPTIONS:
    -d, --destination <DESTINATION>    Output file name [default: app]
    -h, --help                         Print help information
    -l, --language <LANGUAGE>          Programming language [default: python]
    -V, --version                      Print version information
```

Create a `.lang` file like `my_lang_file.lang` and write your code in it.  
**For example:**
```python
mx = 5
untill mx > 4
	print(x)
```
> The code above is a python simplifier
> Take note that you can code in the language you will be compiling to

Then compile it using language binary:
```command
$ ./language my_lang_file.lang
```
Now you can run your created python file (`app.py`) which you can run using the python.
```command
$ python app.py
```
You can set destination to rename the result file

## Development
### Language Patterns
The patterns of the language and what it will be compiled to are written with *Regular Expression* in `patterns/` directory.  
Each language has its own json file (like `patterns/python.json`).  
The key string is the regular expression of a pattern. And the value of it is the defination of that pattern.

**Simple Example:**
```json
{"then give ": "return "}
```
This code above will translate the patterns with `then give` keyword, to a `return` keyword  
So with the language file of below:
```
def getmyname():
    myname = input()
    then give myname
```
We will have a python file like so:
```python
def getmyname():
    myname = input()
    return myname
```
**Real Example:**
```json
{"untill (?P<t>.+)\r\n": "while not $t:\n\t"}
```
This code above will translate the patterns with `untill` keyword, to a `while` loop  
So with the language file of below:
```python
untill x>0
    x+=1
```

We will have a python file like so:
```python
while not x>0:
    x+=1
```

#### Contributing
Contributing to the `patterns` of language is simple.
**it is *not at* all required to know Rust to contribute.**
All you need is to know a *programming language* and have an idea of *how people may want to write it*  
To add your translation of the language:
1. Clone the project
2. Create a new branch - *It is recommended to name it as **myGitHubId-MyLanguage***
3. Go to `patterns/` directory
4. Check if your desired language is there. If it was there open the file and **if not**:  
    4.1. Create a `json` file in the `patterns/` directory with the language name
**Without spaces or dashes**. Like `patterns/lua.json`  
    4.2 Open it and type `{}` in the file to make a valid json file
5. Add a value to the `json` object like below:
```json
"SIMPLIFIED PATTERN": "THE LANGUAGE SYNTAX"
```
- Not that both values will be parsed as a `regex` pattern
6. Add and commit your changes to the git history
7. Push your branch to the `origin` remote
8. Make a pull-request in the github and assign @thearian to review your changes
9. Wait for our response and thank you for considering!

### Translator Code
We have tried to make contributer as far as we can from the `rust` files
and more into the `json` files for the language translation.  
But if there is any suggestions or bug fix for the main translator
we are happy to view it.

1. Clone the project
2. Create a new branch - *It is recommended to name it as **myGitHubId-MyLanguage***
3. Apply your changes in the `src/main.rs` or any file you desire
4. Add and commit your changes to the git history
5. Push your branch to the `origin` remote
6. Make a pull-request in the github and assign @thearian to review your changes
7. Wait for our response and thank you for considering!
