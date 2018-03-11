Rusty mower
===========
Kind of XKE: my first Rust mower.

## Rust discovery

Start with slides <https://xebia-france.github.io/xke-rs/#/> from [Jbbouille](https://github.com/Jbbouille) and [dimapod](https://github.com/dimapod).

## How to do this hands'on

### Tests
As usual: make the tests pass ! Run them with : 

`cargo test`

### Run
If you want to run your mower :

`cargo run instructions.txt`

### Where to start

Try to start with model; proposed order:

1. [lawn](src/lawn.rs)
1. [command](src/command.rs)
1. [position](src/position.rs)
1. [direction](src/direction.rs)
1. [mower](src/mower.rs)

[main](src/main.rs) is done.

### Online REPL

<https://play.rust-lang.org/>

### RTFM

To make all of this, I read [the book](https://doc.rust-lang.org/book/) and the [API doc](https://doc.rust-lang.org/).

Useful topics: Vec, vec!, String, Tuple, Split, Iterator...


#### Traits used in the Mower

Traits can be derived (compiler generate implementation) or implemented explicitly.

##### Debug

`Debug` is used to have a default format for structs as String. It can be compared to the generated toString() in Scala case classes.
It allows the placeholder `{:?}` in formated strings. Not to be confused with `Display`that allows `{}` placeholder and that must be explicitly implemented.

##### Clone
`Clone` allows `clone()` i.e. an explicit copy for type that cannot be explicitly copied. See <https://doc.rust-lang.org/std/clone/>.

##### PartialEq
`PartialEq` allows operator `==`. It differs from `Eq` trait that implements difference. 
`PartialEq` doesn't implement difference so "partially equal" type can have an inconsistent `!=`operator.
See <https://www.reddit.com/r/rust/comments/3f9e7q/why_does_eq_require_partialeq/>.

Used in test for equality assertions. 

##### Add<T>

`Add<P>`must be implemented for types that need to override `+` operator. Here `P` is the right operand type.
Implementor should declare the result type with Scala-like syntax `type Output=MyType` because it can be different from operand types.

Yes, this trait allow to define how `+` can return a banana when you add a dog to a screw. 

## What is strange for the Java/Scala/Js/etc. developper

### `str` and `String`

You will often need `"blabla".to_string()` or `"someString.as_str()"`.

See <https://rustbyexample.com/std/str.html> for details.  

### Rust is young

Filter an option is an "unstable" feature 😕

```
error: use of unstable library feature 'option_filter' (see issue #45860)
   --> src/command.rs:41:21
    |
 41 |   assert_eq!(actual.filter(|cmd| cmd == expected).is_some(), "'G' was not parsed as Command::G");
    |                     ^^^^^^
```

### "Functional" Rust quirks

`and_then()` instead of the usual `flatMap`(or `flat_map` _à la Rust_)

Option must be converted to iterator explicitly.

### The way rust import and modules (`use`, `extern crate`, `mod`) are working

See <https://rustbyexample.com/mod/split.html>.

## Possible improvements

What can we do better next time ?

* Main refactoring
  * Sort functions in the right file
  * Avoid functional/imperative styles mix
* Model
  * check that Mower build coords are inside lawn 😅
  * check if we can make a better usage of memory by using properly references
* Testing
  * Use a fluent test crate
  * Use generators _à la_ [ScalaCheck](https://www.scalacheck.org/)
