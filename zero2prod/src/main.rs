use actix_web:: { web, App, HttpRequest, HttpServer, Responder };


//  helper function
async fn greet(req: HttpRequest) -> impl Responder{
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    //need a server start one.. with options first defined??
    HttpServer::new( || {
        //start app
        App::new()
        //chain routes and call helper
        .route("/", web::get().to(greet))
        .route("/{name}", web::get().to(greet))        
    })
    .bind("127.0.0.1:8000")? //error handling shorthand for the Result<()> enum
    .run() // run the server 
    .await // interesting placement
}
