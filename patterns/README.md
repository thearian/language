## Language Patterns
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
