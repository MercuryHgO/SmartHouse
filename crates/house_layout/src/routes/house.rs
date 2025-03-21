use std::{borrow::BorrowMut, ops::DerefMut};

use gauge::house_layout::{floor::{self, Floor}, room::Room};
use http::{HttpRequestBuilder, HttpResponce, HttpResponceBuilder, Status};
use json_minimal::Json;

use crate::AppState;

use super::RouteHandler;

fn app_write(args: &super::RouteArgs) -> crate::Result<&mut AppState> {
    let mut app = args.app
        .write()
        .map_err(|e|
            http::Error::InternalServerError(e.to_string().into())
        )?
    ;
    Ok(&mut app)
}

fn app_read(args: &super::RouteArgs) -> crate::Result<&AppState> {
    todo!()
}

pub(super) struct House {}

impl RouteHandler for House {
    fn handle(args: super::RouteArgs) -> crate::Result<HttpResponce> {
        match args.request.method() {
            http::Method::GET    => get(args),
            http::Method::POST   => post(args),
            http::Method::DELETE => delete(args),
            http::Method::PATCH  => patch(args),
            _ => Ok(
                HttpResponceBuilder::default()
                    .status(http::Status::MethodNotAllowed)
                    .build()
            ),
        }
    }
}

fn get(args: super::RouteArgs) -> crate::Result<HttpResponce> {
    let content: Json = args.app.read()
        .map_err(|e|
            http::Error::InternalServerError(e.to_string().into())
        )?
        .house
        .clone()
        .into()
    ;
    let content = Json::JSON(vec![content]);

    let responce = HttpResponceBuilder::default()
        .header("Content-Type".to_string(),"application/json".to_string())
        .content( &content.print() )
        .build()
    ;

    Ok(responce)
}

// POST

fn post(args: super::RouteArgs) -> crate::Result<HttpResponce> {
    let routes_str: Vec<&str> = args.route[2..]
        .into_iter()
        .map(|s| s.as_str())
        .collect()
    ;

    let responce = HttpResponceBuilder::default();

    let status: Status = match *routes_str {
        ["floor"] => {
            app_write(&args)?
                .house
                .push_floor()
            ;

            Status::OK
        },
        ["floor",floor,"room"] => {
            let floor_number: usize = 
                floor.parse()
                .map_err(|_| http::Error::MalformedRequest)?
            ;

            let floor = app_write(&args)?
                .house
                .floors()
                .get(floor_number as usize)
                .ok_or(http::Error::ResponceError(
                    HttpResponceBuilder::default()
                        .status(Status::NotFound)
                        .build()
                ))?
            ;

            let content = args.request.content();

            let room: Room = content
                .map(|r| -> crate::Result<Room> {
                    Json::parse(r.as_bytes())
                        .map_err(|_|
                            http::Error::MalformedRequest
                        )?
                    .try_into()?
                })?
                .unwrap_or_default()
            ;

            floor.add_room(room)

            Status::OK
        },
        _ => Status::NotFound
    };

    responce = responce.status(status);
    let responce = responce.build();

    Ok(responce)
}

fn add_floor(args: super::RouteArgs) -> crate::Result<HttpResponce> {
    Ok(
        HttpResponceBuilder::default()
            .content(&"Aboba")
            .build()
    )
}

fn patch(args: super::RouteArgs) -> crate::Result<HttpResponce> {
    todo!()
}

fn delete(args: super::RouteArgs) -> crate::Result<HttpResponce> {
    todo!()
}
