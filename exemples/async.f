module main

use (
    "std"
    "async"
    "net"
    "crypto/uuid"
    "json"
)

struct User {
    pub (
        Id: uuid::UUID
        Name: str
        Username: str
    )
}

impl User {
    fn New(
        name: str :? "",
        username: str :? ""
    ) -> User {
        let user: mut User = User{}

        user.Id = uuid::new()
        user.Name = name
        user.Username = username

        user
    }
}

fn getuser(url: str) -> Result<User, String> {
    let result = net::get<User>(url)::await()

    match result {
        Ok(user) => {
            Ok(user)
        }

        Err(err) => {
            std::log("Error: &{err}")
            Err(err)
        }
    }
}

fn main() -> i8 {
    let result = getuser("https://example.com/user")::await()

    match result {
        Ok(user) => {
            std::print("User: &{user.Name} (@&{user.Username})")
        }

        Err(err) => {
            std::log("Failed to get user: &{err}")
        }
    }

    0
}