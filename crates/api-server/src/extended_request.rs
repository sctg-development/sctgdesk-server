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
use rocket::request::{FromRequest, Outcome, Request};
use std::{collections::HashMap, convert::Infallible};
use rocket_okapi::request::OpenApiFromRequest;
use rocket_okapi::gen::OpenApiGenerator;
use rocket_okapi::request::RequestHeaderInput;
use rocket_okapi::OpenApiError;

/// # ExtendedRequest
/// 
/// This struct is used to extend the Request struct from Rocket adding a headers field.
/// this is needed for analysis of the headers in the request. Particularly useful for
/// retrieving the host with utils::get_host::get_host()
#[derive(Debug)]
pub struct ExtendedRequest{
    pub headers: HashMap<String, String>,
}

/// # FromRequest for ExtendedRequest
/// 
/// This implementation of FromRequest is used to create an ExtendedRequest from a Rocket Request.
#[rocket::async_trait]
impl<'r> FromRequest<'r> for ExtendedRequest {
    type Error = Infallible;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let headers_src = request.headers().clone();
        let mut headers:HashMap<String, String> = HashMap::new();
        headers_src.iter().for_each(|k|{
            headers.insert(k.name.to_string().to_lowercase(),k.value.to_string().to_lowercase());
        });
        Outcome::Success(Self { headers })
    }
}

/// # OpenApiFromRequest for ExtendedRequest
/// 
/// This implementation of OpenApiFromRequest is used to create an OpenApi RequestHeaderInput from a Rocket Request.
/// This is used to generate the OpenAPI documentation for the request.
/// 
impl OpenApiFromRequest<'_> for ExtendedRequest {
    fn from_request_input(_: &mut OpenApiGenerator, _: std::string::String, _: bool) -> Result<RequestHeaderInput, OpenApiError> { 
        Ok(RequestHeaderInput::None)
    }
}