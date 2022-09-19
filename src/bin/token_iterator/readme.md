# Intro
This project implements a token iterator. The aim of this project is to practice generics and traits, and their uses such as iterator, conversions etc by trying to implement some features for a struct. The project will also involve IO and use of Result type including `?` operator.

# Code structure
`token_iterator.rs` Provides some structs:
- `FilesToIterateBuilder`: This is defined to build an instance of `FilesToIterate`. No code changes will be required in this struct, but you will need to implement 1 trait for it (see problem statement below). You will also be adding trait bounds to generics in this struct.
- `FilesToIterate`: Represents file to iterate over and delimiter to use for iterating. The delimiter type is generic. We don't need to modify this struct, but we will need to implement atleast some trait for this in order to achieve the end goal. You will also be adding trait bounds to generics in this struct.
- `Token`: Represent 1 token. This will be the type returned from iterator. This provides 1 constructor function `new`, which checks that token should be ascii and returns an instance of Result type. We don't need to change code, but we will be migrating the `new` function to a trait impl for Token.
- `InvalidTokenError`: This is the error type returned from `Token::new` function. 
- `TokenIteratorError`: This is the error type we will use to return any errors from our iterator.

# Problem statement
The goal is to implement an iterator to read from a list of files, split the content using provided delimiter and return each token (word) based on the delimiter, one at a time. In other words, we want to be able to run some code like this:
```rust
let filesToIterate = FilesToIterateBuilder.new(',')
    .add('/path/to/file1')
    .add('/path/to/file2')
    .build()

for token_result in filesToIterate {
    match token_result {
        Ok(token) => {
            // do something
        },
        Err(_) => {
            // som error logic
        }
    }
}
```

## Requirements
- `Token` struct provides `new` method to convert from string to a token (with possible error). This is also the signature of standard trait [`TryFrom`](https://doc.rust-lang.org/std/convert/trait.TryFrom.html). Although the use of this trait depends on situation, but since this assignments is about looking at traits, we will be implementing TryFrom for `Token` struct. Move the code from `new` to `TryFrom` implementation. Note that `TryFrom` also implies [`TryInto`](https://doc.rust-lang.org/std/convert/trait.TryInto.html). After the change, we can write code like this:
    ```rust
        let t: Token = "some token".try_into()?;
        // this is equivalent to 
        // let t: Token = Token::try_from("some token")?;
    ```
- Provide a default trait implementation for `FilesToIterateBuilder`, which will split the content on [any whitespace](https://doc.rust-lang.org/std/primitive.char.html#method.is_whitespace). See [Default trait](https://doc.rust-lang.org/std/default/trait.Default.html) for trait definition to implement. After this, we would be able to get a default builder by running:
  ```rust
    let builder: FilesToIterateBuilder<_> = Default::default()
  ```
- `FilesToIterate` (and `FilesToIterateBuilder`) takes delimiter as a generic type `D`. This was kept generic to allow multiple ways to specify a delimiter. A dlimiter can be a `char`, `array or slice of char` to specify a list of chars to split on, or `a function (closure)` which takes a char and returns a bool to tell the iterator whether to split on this or not. Figure out a way to restrict the generic parameter to these types, and also think about how you will handle all these cases using 1 general code for iterator. Hint: you don't need to specify the bound or handle these cases directly, think about adding abstractions and other standard traits that you can use.
- Write an iterator to iterate over tokens, where tokens content of files split using the delimiter. The iterator will return `Result<Token, TokenIteratorError>` on each call to next. 


## Constraints:
- You are NOT allowed to modify any existing code, except for changes explicitly listed below. You will be adding new trait implementations or functions, but will not modify existing structs and functions.
- Do NOT read contents of file ahead of time, this defeats the purpose of iterators which allow doing operations lazily (on call to next). By not reading all contents in advance, you will also be reducing memory requirements of your code.
- Do NOT write explicit return statements for errors, use [`?` operator](https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html#a-shortcut-for-propagating-errors-the--operator) instead. This keeps the code readable by not bloating it with error handling. Notice that there can be multiple types of error during iteration (io error on reading file, and `InvalidTokenError` while creating the token using non-ascii chars). But iterator can return only error type `TokenIteratorError` (since the return type has to be `Result<Token, TokenIteratorError>`). If you use `?` operator on these error types directly, the code will not compile as the return error type is different. One way is to [map the result to required error type](https://doc.rust-lang.org/std/result/enum.Result.html#method.map_err). But since we want the core logic to be readable and minimize error handling logic, figure out a way to work with `?` operator directly. It is possible to directly use `?` operator on different error types if you utilize the correct standard traits.
- Do NOT use existing types or libs to solve the problem, as it defeats the purpose of figuring out the solution by yourself for learning. For example, there are existing functions and traits to split text using different types of delimiters in standard lib itself. Don't use that or copy code from there directly. Focus on learning.

## Testing
To check your implementation, run the tests using `cargo test --bin token_iterator`