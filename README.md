# rust_tuto
A serie of tutorials on rust, for teaching purposes

- basic_syntax : implement 4 functions to protect you from covid (wear_mask, keep_distance, wash_hands, get_tested)
- mut : examples related to mutability
- ownership-pluralize : a step-by-step approach to ownership

Those examples correspond to what is most surprising to newcomers to rust. 
The main idea is that one needs to find ways around patterns such as:

```
let a = assign 
function(a)
reassign a or reuse a
```

## how to start
To really learn rust, I suggest:
* [rustlings](https://github.com/rust-lang/rustlings) : the exercices are a great intro to the syntax. Don't hesitate to look at the rust book as soon as you need, but it's good to learn by trying. To use rustlings, clone the repository, run `rustling watch` in a command window, and start working on the first exercice with your favorite IDE (the name of the exercice is provided in the window)
* [adventofcode](https://adventofcode.com/) is a good way to get into real problem solving (and can work for any language). See [adventofcode-rs](https://github.com/fimbault/adventofcode-rs) for my solutions in rust. The challenge happens in december (one challenge per say), but you can reuse past editions to start anytime.

In case you want to debug from vscode, please see this [gist](https://gist.github.com/fimbault/cc973b1d33b004c30fdfbeb36ff6ff34).

## and then what?
Focus on what's specific to rust, or on frameworks or environments (ex: no_std) you'll use.
* [lifetimes](https://www.youtube.com/watch?v=1QoT9fmPYr8): this video is a great introduction
* [combining errors](https://richard.dallaway.com/2020/01/20/rust-error-chaining.html): often an issue when you're just trying to make things work
* serde: for serialization/deserialization
* async
* web
* wasm
