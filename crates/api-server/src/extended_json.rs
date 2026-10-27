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
use std::collections::HashMap;

use rocket::data::{Data, FromData, Outcome, Limits};
use rocket::request::{Request,local_cache};
use rocket::http::Status;
use rocket_okapi::gen::OpenApiGenerator;
use rocket_okapi::okapi::openapi3::RequestBody;
use rocket_okapi::request::OpenApiFromData;
use rocket_okapi::{JsonSchema,Result as OkapiResult};
use serde::Deserialize;

/// A wrapper around a JSON value that includes headers.
#[derive(Debug)]
pub struct ExtendedJson<T> {
    pub data: T,
    pub headers: HashMap<String, String>,
}

#[derive(Debug)]
pub enum Error<'a> {
    Io(std::io::Error),
    Parse(&'a str, serde_json::error::Error),
}

impl<T> ExtendedJson<T> {
    #[inline(always)]
    pub fn into_inner(self) -> T {
        self.data
    }
    /// Returns a reference to the headers of the request.
    #[inline(always)]
    pub fn headers(&self) -> HashMap<String, String> {
        self.headers.clone()
    }
}

impl<'r, T: Deserialize<'r>> ExtendedJson<T> {
    fn from_str(s: &'r str, h: HashMap<String, String>) -> Result<Self, Error<'r>> {
        let data = serde_json::from_str(s).map_err(|e| Error::Parse(s, e))?;
        let headers = h;
        Ok(Self { data, headers })
    }

    async fn from_data(req: &'r Request<'_>, data: Data<'r>) -> Result<Self, Error<'r>> {
        let headers_src = req.headers().clone();
        let mut headers:HashMap<String, String> = HashMap::new();
        headers_src.iter().for_each(|k|{
            headers.insert(k.name.to_string().to_lowercase(),k.value.to_string().to_lowercase());
        });
        let limit = req.limits().get("json").unwrap_or(Limits::JSON);
        let string = match data.open(limit).into_string().await {
            Ok(s) if s.is_complete() => s.into_inner(),
            Ok(_) => {
                let eof = std::io::ErrorKind::UnexpectedEof;
                return Err(Error::Io(std::io::Error::new(eof, "data limit exceeded")));
            },
            Err(e) => return Err(Error::Io(e)),
        };
        Self::from_str(local_cache!(req, string), headers)
    }
}

#[rocket::async_trait]
impl<'r, T: Deserialize<'r>> FromData<'r> for ExtendedJson<T> {
    type Error = Error<'r>;

    async fn from_data(req: &'r Request<'_>, data: Data<'r>) -> Outcome<'r, Self> {
        match Self::from_data(req, data).await {
            Ok(value) => Outcome::Success(value),
            Err(Error::Io(e)) if e.kind() == std::io::ErrorKind::UnexpectedEof => {
                Outcome::Error((Status::PayloadTooLarge, Error::Io(e)))
            },
            Err(Error::Parse(s, e)) if e.classify() == serde_json::error::Category::Data => {
                Outcome::Error((Status::UnprocessableEntity, Error::Parse(s, e)))
            },
            Err(e) => Outcome::Error((Status::BadRequest, e)),

        }
    }
}

macro_rules! fn_request_body {
    ($gen:ident, $ty:path, $mime_type:expr) => {{
        use rocket_okapi::okapi::{
            openapi3::{MediaType, RequestBody},
            Map,
        };
        let schema = $gen.json_schema::<$ty>();
        Ok(RequestBody {
            content: {
                let mut map = Map::new();
                map.insert(
                    $mime_type.to_owned(),
                    MediaType {
                        schema: Some(schema),
                        ..MediaType::default()
                    },
                );
                map
            },
            required: true,
            ..rocket_okapi::okapi::openapi3::RequestBody::default()
        })
    }};
}

type OkapiResult_ = OkapiResult<RequestBody>;
impl<'r, T: JsonSchema + Deserialize<'r>> OpenApiFromData<'r> for ExtendedJson<T> {
    fn request_body(gen: &mut OpenApiGenerator) -> OkapiResult_ {
        //Ok(gen.json_body::<T>("application/json"))
        fn_request_body!(gen, T, "application/json")
    }
}