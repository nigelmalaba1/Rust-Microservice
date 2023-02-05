
//Add a new route

//use actix_web::{web, App, HttpResponse, HttpServer};
use actix_web::{web, App, HttpResponse, HttpServer};
async fn index() -> HttpResponse {
    HttpResponse::Ok().body("Hello, World!")
}

//async fn index() -> HttpResponse means that the function is asynchronous and returns a HttpResponse.

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    //The #[actix_web::main] attribute is a macro that creates an asynchronous main function.
    //The main function is the entry point of the application.
    //The main function returns a std::io::Result.
    //The main function calls the HttpServer::new function to create a new HTTP server.
    //The HttpServer::new function takes a closure that returns an App instance.
    //The App instance is configured with a route that matches all GET requests to the root path (/).
    //The route is configured to call the index function when a request is received.
    //The HttpServer::new function is called with the bind method to bind the server to the IP address
    
    
    HttpServer::new(|| App::new().route("/", web::get().to(index)))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
