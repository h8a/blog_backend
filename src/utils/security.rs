use chrono::{Duration, Utc};
use jsonwebtoken::{
    decode_header,
    decode,
    encode,
    Algorithm,
    DecodingKey,
    EncodingKey,
    Header,
    Validation
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub aud: Option<String>,         // Optional. Audience
    pub exp: usize,                    // Required (validate_exp defaults to true in validation). Expiration time (as UTC timestamp)
    pub iat: Option<usize>,          // Optional. Issued at (as UTC timestamp)
    pub iss: Option<String>,         // Optional. Issuer
    pub nbf: Option<usize>,          // Optional. Not Before (as UTC timestamp)
    pub sub: Option<String>,         // Optional. Subject (whom token refers to)
    pub id: String,
}

// let token = encode(&Header::default(), &my_claims, &EncodingKey::from_secret("secret".as_ref())).unwrap();
pub fn jwt_encode(user_id: &i32) -> String {

    let exp_seconds: i64 = dotenv::var("JWT_EXP").unwrap().parse::<i64>().unwrap();

    let expiration = Utc::now() + Duration::seconds(exp_seconds);

    let claims = Claims {
        exp: expiration.timestamp() as usize,
        aud: None,
        iat: None,
        iss: None,
        nbf: None,
        sub: None,
        id: user_id.to_string(),
    };

    encode(
        &Header::new(Algorithm::HS512), 
        &claims, 
        &EncodingKey::from_secret(dotenv::var("JWT_SECRET")
            .unwrap()
            .as_ref())
        )
        .unwrap()
}

// let token = decode::<Claims>(&token, &DecodingKey::from_secret("secret".as_ref()), &Validation::default())?;
pub fn jwt_decode(token: &String) {
    println!("{:?}", decode_header(&token));
    println!("{:?}", decode::<Claims>(&token, &DecodingKey::from_secret(dotenv::var("JWT_SECRET").unwrap().as_ref()), &Validation::new(Algorithm::HS512)).unwrap());
}