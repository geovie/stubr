use jsonwebtoken::Header;
use serde_json::Value;
use wiremock::Request;

use super::{AUTHORIZATION_HEADER, BEARER_PREFIX};

pub trait RequestAuthExtension {

    fn authorization_header(&self) -> Option<&str>;

    fn jwt(&self) -> Option<&str> {
        self.authorization_header()
            .filter(|h| h.contains(BEARER_PREFIX))
            .map(|h| &h[BEARER_PREFIX.len() + 1..])
    }

    fn jwt_header(&self) -> Option<Header> {
        self.jwt()
            .and_then(|jwt| jsonwebtoken::decode_header(jwt).ok())
    }

    fn jwt_payload(&self) -> Option<Value> {
        self.jwt()
            .and_then(|jwt| {
                jwt.split('.')
                    .take(2).last()
                    .and_then(|payload| base64::decode(payload).ok())
                    .and_then(|payload| serde_json::from_slice(payload.as_slice()).ok())
            })
    }
}

impl RequestAuthExtension for Request {
    fn authorization_header(&self) -> Option<&str> {
        self.headers.get(&AUTHORIZATION_HEADER)
            .map(|v| v.as_str())
    }
}