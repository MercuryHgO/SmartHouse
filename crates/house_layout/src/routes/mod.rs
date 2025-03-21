use std::fmt::Debug;

use house::House;
use http::{HttpRequest, HttpResponce, HttpResponceBuilder, Status};

use crate::{App, Result};

mod house;

struct RouteArgs {
    request: HttpRequest,
    app: App,
    route: Vec<String>
}

trait RouteHandler {
    fn handle(args: RouteArgs) -> Result<HttpResponce>;
}

pub(crate) fn route(request: HttpRequest, app: App) -> Result<HttpResponce> {
    let route = request
        .path()
        .split("/")
        .map(|s| s.to_owned())
        .collect()
    ;

    let args = RouteArgs {
        request,
        app,
        route
    };

    let responce = match args.route[1].as_str() {
        "" =>
            HttpResponceBuilder::default().build(),
        "house" =>
            House::handle(args)?,
        _ =>
            HttpResponceBuilder::default()
                .status(Status::NotFound)
                .build(),
    };

    Ok(responce)
}


pub(crate) fn error_handler(e: http::Error) -> HttpResponce {
    let status: Status = match e {
        http::Error::MalformedStatusLine => Status::BadRequest,
        http::Error::MalformedRequest => Status::BadRequest,
        http::Error::UnknownRequestMethod => Status::BadRequest,
        http::Error::UnsopportedProtocol => Status::BadRequest,
        http::Error::MalformedHeader => Status::BadRequest,
        http::Error::ResponceError(e) => return e,
        _ => {
            eprint!("{}", e);
            Status::InternalServerError
        }
    };
    
    HttpResponceBuilder::default()
        .status(status)
        .build()
}
