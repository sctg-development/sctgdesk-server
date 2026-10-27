// Copyright (c) 2024 Ronan LE MEILLAT for SCTG Development
//
// This file is part of the SCTGDesk project.
//
// SCTGDesk is free software: you can redistribute it and/or modify
// it under the terms of the Affero General Public License version 3 as
// published by the Free Software Foundation.
//
// SCTGDesk is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// Affero General Public License for more details.
//
// You should have received a copy of the Affero General Public License
// along with SCTGDesk. If not, see <https://www.gnu.org/licenses/agpl-3.0.html>.
use rocket::{http::Status, response::Responder, Request, Response};
use rocket_okapi::gen::OpenApiGenerator;
use rocket_okapi::okapi::openapi3::Responses;
use rocket_okapi::okapi::{schemars, Map};
use rocket_okapi::response::OpenApiResponderInner;
use rocket_okapi::{JsonSchema, OpenApiError};

#[derive(Debug, JsonSchema)]
pub enum ActionResponse {
    Empty,
    Error(String),
}

impl<'r> Responder<'r, 'static> for ActionResponse {
    fn respond_to(self, _: &'r Request<'_>) -> rocket::response::Result<'static> {
        match self {
            ActionResponse::Empty => Response::build().status(Status::Ok).ok(),
            ActionResponse::Error(err) => {
                let body = rocket::serde::json::json!({ "error": err }).to_string();
                Response::build()
                    .header(rocket::http::ContentType::JSON)
                    .sized_body(body.len(), std::io::Cursor::new(body))
                    .status(Status::Ok)
                    .ok()
            }
        }
    }
}

impl OpenApiResponderInner for ActionResponse {
    fn responses(_generator: &mut OpenApiGenerator) -> Result<Responses, OpenApiError> {
        use rocket_okapi::okapi::openapi3::{RefOr, Response as OpenApiReponse};

        let mut responses = Map::new();
        responses.insert(
            "422".to_string(),
            RefOr::Object(OpenApiReponse {
                description: "\
                # [422 Unprocessable Entity](https://developer.mozilla.org/en-US/docs/Web/HTTP/Status/422)\n\
                This response is given when you request body is not correctly formatted. \
                ".to_string(),
                ..Default::default()
            }),
        );
        responses.insert(            "200".to_string(),
        RefOr::Object(OpenApiReponse {
            description: r#"\
            # [200 OK](https://developer.mozilla.org/en-US/docs/Web/HTTP/Status/200)  
            This response is given when the request is successful.  
            The body is empty if there is no error,  
            The body contains a json object with the error {"error":"Error message"}   
            "#.to_string(),
            ..Default::default()
        }));
        Ok(Responses {
            responses,
            ..Default::default()
        })
    }
}