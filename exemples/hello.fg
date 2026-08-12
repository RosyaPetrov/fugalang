module main

use (
    "std"
)

struct People<T> {
    pub Name: mut T
}

fn getname() -> People<str> {
    let name: mut str = "Name"
    let people: mut People {
        Name: name,
    }

    people
}

fn main() -> i8 {
    std::print("Hello, &{getname().Name}!")
    return 0
}
