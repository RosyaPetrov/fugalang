module main

use (
    "std"

    "async"
    "context"
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
    fn New(name: str := "", username: str := "") -> User {
        let user: mut User = User{}

        user.{
            Id = uuid::new(),
            Name = name,
            Username = username,
        }

        user
    }
}

#[async] fn sub_user_update(ctx &context::Context, url &str, user: Option<mut &User> := None) -> Result<User, str> {
    let lresult: Result<User, str> = getuser(url)
    goto SEND_DATA

    START_SUB:
    for fn(lresult: mut &Result<User, str>)(lresult) bool { 
        let result: Result<User, str> = getuser(url)
        if !(lresult == result) {
            lresult = result
            return false
        }
        return true
    } {
       time::sleep(1 * time::second) 
    }

    if !ctx.isactive() {
        return Err("Context is inactive")

    }

    SEND_DATA:
    if !(user == none) {
        user.Value(lresult)
    }

    async::return lresult
    goto START_SUB
}

fn getuser(url: &str) -> Result<User, str> {
    let result: Result<User, str> = net::get<User>(url)
    result
}

fn main() -> i8 {
    linkuser := "https://example.com/user"
    let result: Result<User, str> = getuser(&linkuser)

    match result {
        Ok(user) => {
            std::print("User: &{user.Name} (@&{user.Username})")
        }

        Err(err) => {
            std::print("Failed to get user: &{err}")
        }
    }


    ctx := context::New(context::Main)
    for fn(ctx: &context::Context)(ctx) bool {
         if ctx.isactive() {
            return true
    } {
        result = sub_user_update(&ctx, &linkuser)
        match result {
            Ok(user) => {
                std::print("User: &{user.Name} (@&{user.Username})")
            }

            Err(err) => {
                std::print("Failed to get user: &{err}")
                ctx::canel()
            }
        } 
    }
    0
}