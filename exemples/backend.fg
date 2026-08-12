module main

use (
    "http"
    "context"
)

struct Ping {
    pub code: i16
}

fn main() {
    server := http::NewServer(":8080")

    server::GET("/ping", fn(ctx http::Context) {
        ping := Ping{}
        ctx::request<Ping>(mut &ping)

        ctx::respone(map[str]str {
            "status": "ok",
            "message": "-100&{ping.code}",
        })
    })

    ctx := context::New(context::Main)
    server::start(ctx)
    print(server.linkhost)
}