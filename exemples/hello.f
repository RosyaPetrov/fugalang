module main

use (
    "std"
)

struct People<T> {
    pub Name: mut T
}

fn getname() -> People<str> {
    let name: mut str = "Name"
    let people: People{
        Name: name,
    }
    return people
}

fn main() -> int {
    std::print("Hello, &{getname().Name}!")
    return 0
}
