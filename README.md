# rust-course-2024-07-14


1) Introduction, hello world, variables, command line tools, development tools, variable, mut, parse (string to number). types. Overflow in integers.

2) We learned about Option, Result, match and various unwrap_* methods handline Result.  Random, STDIN, We dealt a bit with ownership (String and &str) e.g. trim_end return &str and we need to call to_owner to get a String.
We read some files. Arrays. String memory allocation.
const, chars
A bit of enums.

3)
* strings
* ownership
* vectors
* enums
* hashMap

------------------------
Approximate plan


4)
* files
    * https://github.com/szabgab/wc.rs
* filesystem
* structs
* traits
* methods
* functions
* simple macro to have default parameters for functions.
* reading JSON
    * what if the json contains  a field with `null` value?
    * can serde_json handle JSON lines https://jsonlines.org/   Maybe this: https://crates.io/crates/serde-jsonlines
    * read the yaml files of the kantoniko database
    * read the Cargo.toml file

Exercise
Create a vector of 3 strings;
print the len. capacity and point {:p}   var.as_ptr  of the vector

add 5 more elements to the vector
print again the same thing

add 2 more element 
print again

get fed up with printing create a macro instead of the 2nd line above






5)
* async (http)
* modules
* command line interface
* iterators
* lifetime specifiers
* Error handling


6)
* threading
* regex
* ruff




what is the story with the percent* crates? why do they always match?

maybe:

* unsafe
* drop - how are resource freed (e.g. filehandle, etc.)

