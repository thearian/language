# Contributing

## To the patterns
Patterns are **how any language syntax can be simplified**.  
This era is mainly where you would want to contribute.


Contributing to the `patterns` of language is simple.
**it is *not at* all required to know Rust to contribute.**
All you need is to know a *programming language* and have an idea of *how people may want to write it*  

### To add your translation of the language:
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

## To the translator
We have tried to make contributer as far as we can from the `rust` files
and more into the `json` files for the language translation.  

> Note that translator is only a parser for the `patterns` json files

But if there is any suggestions or bug fix for the main translator
we are happy to view it.

### To commit changes to the translator:
1. Clone the project
2. Create a new branch - *It is recommended to name it with the [commit lint principals](https://github.com/conventional-changelog/commitlint/tree/master/@commitlint/config-conventional#type-enum)*
3. Apply your changes in the `src/main.rs` or any file you desire
4. Add and commit your changes to the git history
5. Push your branch to the `origin` remote
6. Make a pull-request in the github and assign @thearian to review your changes
7. Wait for our response and thank you for considering!
