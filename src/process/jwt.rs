use anyhow::Result;
use fancy_duration::ParseFancyDuration;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::time::{Duration, SystemTime};

const SECRET: &str = "secret";

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    aud: String,
    sub: String,
    exp: usize,
}

pub fn sign_jwt(aud: String, sub: String, exp: String) -> Result<String> {
    let duration = Duration::parse_fancy_duration(exp)?;
    if duration.is_zero() {
        return Err(anyhow::anyhow!("expiration time is zero"));
    }
    let exp_time = SystemTime::now() + duration;
    let exp = exp_time.duration_since(SystemTime::UNIX_EPOCH)?;
    let exp = exp.as_secs() as usize;
    let my_claims = Claims { aud, sub, exp };
    let token = encode(
        &Header::default(),
        &my_claims,
        &EncodingKey::from_secret(SECRET.as_ref()),
    )?;
    Ok(token)
}

pub fn verify_jwt(token: String) -> Result<bool> {
    let mut validation = Validation::new(Algorithm::HS256);
    validation.set_audience(&["aud"]);
    validation.set_required_spec_claims(&["exp", "sub", "aud"]);

    let decoded = decode::<Claims>(
        &token,
        &DecodingKey::from_secret(SECRET.as_ref()),
        &validation,
    );

    match decoded {
        Ok(decoded_token) => {
            println!("decoded token: {:?}", decoded_token);
            Ok(true)
        }
        Err(err) => {
            println!("decoding error: {:?}", err);
            Ok(false)
        }
    }
}
