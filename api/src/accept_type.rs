// Copyright (c) Aptos
// SPDX-License-Identifier: Apache-2.0

use aptos_api_types::mime_types::BCS;
use aptos_api_types::mime_types::BCS_OUTPUT_NEW;
use poem::{web::Accept, FromRequest, Request, RequestBody, Result};

#[derive(PartialEq)]
pub enum AcceptType {
    Json,
    Bcs,
}

// This impl allows us to get the data straight from the arguments to the
// endpoint handler.
#[async_trait::async_trait]
impl<'a> FromRequest<'a> for AcceptType {
    async fn from_request(request: &'a Request, _body: &mut RequestBody) -> Result<Self> {
        let accept = Accept::from_request_without_body(request).await?;
        parse_accept(&accept)
    }
}

/// Check that the accept type is one of the allowed variants. If there is no
/// overriding explicit accept type, default to JSON.
fn parse_accept(accept: &Accept) -> Result<AcceptType> {
    for mime in &accept.0 {
        if matches!(mime.as_ref(), BCS_OUTPUT_NEW | BCS) {
            return Ok(AcceptType::Bcs);
        }
    }

    // Default to returning content as JSON.
    Ok(AcceptType::Json)
}
