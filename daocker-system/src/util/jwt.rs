use salvo::{jwt_auth::{ConstDecoder, CookieFinder, JwtAuth}, Router};
use serde::{Serialize, Deserialize};

use crate::setting::SETTINGS;

#[derive(Serialize, Deserialize)]
pub struct JwtClaims {
    pub username: String,
    pub exp: i64
}

#[allow(dead_code)]
pub fn auth_handler() -> JwtAuth<JwtClaims, ConstDecoder> {
    JwtAuth::new(ConstDecoder::from_secret(SETTINGS.get_token().get_secret_key().as_bytes()))
    .finders(vec![Box::new(CookieFinder::new("token"))])
    .force_passed(true)
}

#[allow(dead_code)]
pub fn jwt_router() -> Router {
    Router::with_hoop(auth_handler())
}

