use actix_web::{web, HttpResponse};
use diesel::prelude::*;
use serde::Deserialize;

use crate::emailService::send_invitation;
use crate::models::{Veri, Pool};

#[derive(Deserialize)]
pub struct VerificationData {
    pub email: String,
}

pub async fn post_verification(
    verification_data: web::Json<VerificationData>,
    pool: web::Data<Pool>,
) -> Result<HttpResponse, actix_web::Error> {
    // run diesel blocking code
    web::block(move || create_verification(verification_data.into_inner().email, pool)).await??;

    Ok(HttpResponse::Ok().finish())
}

fn create_verification(
    eml: String,
    pool: web::Data<Pool>,
) -> Result<(), crate::err::ServiceError> {
    let veri = dbg!(query(eml, pool)?);
    send_invitation(&veri)
}

/// Diesel query
fn query(eml: String, pool: web::Data<Pool>) -> Result<Veri, crate::err::ServiceError> {
    use crate::schema::invitationinfo::dsl::invitationinfo;

    let new_invitation: Veri = eml.into();
    let conn = &pool.get()?;

    let inserted_invitation = diesel::insert_into(invitationinfo)
        .values(&new_invitation)
        .get_result(conn)?;

    Ok(inserted_invitation)
}