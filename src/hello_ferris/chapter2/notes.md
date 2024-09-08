# Chapter 2 Notes

`use std::io;` Imports the Rust's standar library package for I/O.


`let mut guess = String::new();` This creates a mutable variable, using `let apples` will create an inmutable variable as Rust uses inmutable as default, this means that the value of the variables cannot be changed. `String::new()` creates an String which is from the standard library, `::` which indicates that `new` is an associated function of String. In summary, it creates a new String.

`//This is a comment` With `//` we can create comments.

```rust
io::stdin()
    .read_line(&mut guess)
    .expect("Failed to read line"); 
```

If we have not imported the `std::io` at the beggining we could call the `stdin()` function like `std::io::Stdin()`. Then, `read_line(&mut guess)` is a calling to the method to read the line of the user's input and we stored in guess passing `&mut guess`. The `&` means that the value is a reference so the data has not to be copied multiple times on the memory. References are inmutable by default that is why to make in this case mutable we need to write `&mut guess` instead of `&guess`. The `expect` is there to handle a possible error and if you don't write it you would get a WARNING at compilation telling that you have a possible error not handled. This code could be all written in a single line instead of three:

```rust
io::stdin().read_line(&mut guess).expect("Failed to read line"); 
```

Here `println!("You guessed: {}", guess);` the `{}` is used to set the variable in the place.
