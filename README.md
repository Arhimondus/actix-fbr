## Installation:
`cargo add Arhimondus/actix-fbr`

## Example:

### main.rs
```rust
use actix_web::{Responder, HttpServer, App, web, HttpResponse};
use actix_fbr::{routes, services};

routes!("src/routes");

#[actix_web::main]
async fn main() -> std::io::Result<()> {
	HttpServer::new(|| {
		App::new()
		 	.route("/", web::to(|| async { HttpResponse::Ok().body("index") }))
			.service(services!("src/routes"))
	})
	.bind(("127.0.0.1", 8080))?
	.run()
	.await
}
```

### src/routest/one1.rs
```rust
use actix_web::{Responder};

pub async fn get() -> impl Responder {
	"[get] one1"
}
```

### src/routest/one2.rs
```rust
use actix_web::{Responder};

pub async fn get() -> impl Responder {
	"[get] one2"
}
```

## Currently limitation
Only first level paths support.