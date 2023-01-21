use actix_identity::Identity;
use actix_web::{dev::Payload, web, Error, FromRequest, HttpRequest, HttpResponse};
use diesel::prelude::*;
use futures::future::{err, ok, Ready};
use serde::Deserialize;

use crate::err::ServiceError;