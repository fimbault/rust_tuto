# rust_tuto
A serie of tutorials on rust, for teaching purposes

- basic_syntax : implement 4 functions to protect you from covid (wear_mask, keep_distance, wash_hands, get_tested)
- mut : examples related to mutability
- ownership-pluralize : a step-by-step approach to ownership

Those examples correspond to what is most surprising to newcomers to rust. 
The main idea is that one needs to find ways around patterns such as:

```
let a = assign variable
function(a)
reassign a or reuse a
```

## how to learn
To really learn rust, I suggest:
* [rustlings](https://github.com/rust-lang/rustlings) : the exercices are a great intro to the syntax. Don't hesitate to look at the rust book as soon as you need, but it's good to learn by trying. To use rustlings, clone the repository, run `rustling watch` in a command window, and start working on the first exercice with your favorite IDE (the name of the exercice is provided in the window)
* [adventofcode](https://adventofcode.com/) is a good way to get into real problem solving.  
