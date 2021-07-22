use jsonwebtoken::errors::ErrorKind;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use tonic::{Request, Status};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    company: String,
    exp: usize,
}
const KEY: &[u8] = b"secret";

pub fn interceptor(req: Request<()>) -> Result<Request<()>, Status> {
    let token = match req.metadata().get("authorization") {
        Some(token) => token.to_str().unwrap(),
        None => return Err(Status::unauthenticated("Token not found")),
    };
    let validation = Validation {
        sub: Some("mrarsenal".to_string()),
        ..Validation::default()
    };
    match decode::<Claims>(&token, &DecodingKey::from_secret(KEY), &validation) {
        Ok(c) => c,
        Err(err) => match *err.kind() {
            ErrorKind::InvalidToken => panic!("Token is invalid"),
            ErrorKind::InvalidIssuer => panic!("Issuer is invalid"),
            _ => panic!("Some other errors"),
        },
    };
    Ok(req)
}

pub fn get_token(user: String) -> String {
    let my_claims = Claims {
        sub: user,
        company: "ArsenalFC".to_owned(),
        exp: 10000000000,
    };
    let token = encode(
        &Header::default(),
        &my_claims,
        &EncodingKey::from_secret(KEY),
    );
    token.unwrap()
}
