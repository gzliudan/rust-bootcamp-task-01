use anyhow::Result;
use chrono::{Duration, Utc};
use fancy_duration::ParseFancyDuration;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

const SECRET_KEY: &str = "secret";

#[derive(Debug, Serialize, Deserialize)]
struct PrivateClaim {
    aud: String,
    sub: String,
    exp: u64,
}

pub fn sign_jwt(aud: String, sub: String, exp: String) -> Result<String> {
    let duration = Duration::parse_fancy_duration(exp)?;
    if duration.is_zero() {
        return Err(anyhow::anyhow!("expiration time is zero"));
    }
    let exp = (Utc::now() + duration).timestamp() as u64;
    let private_claim = PrivateClaim { aud, sub, exp };
    let token = encode(
        &Header::default(),
        &private_claim,
        &EncodingKey::from_secret(SECRET_KEY.as_ref()),
    )?;
    Ok(token)
}

pub fn verify_jwt(token: String) -> Result<bool> {
    let mut validation = Validation::default();
    validation.set_audience(&["aud"]);

    let decoded = decode::<PrivateClaim>(
        &token,
        &DecodingKey::from_secret(SECRET_KEY.as_ref()),
        &validation,
    )
    .map(|data| data.claims);

    match decoded {
        Ok(claim) => {
            println!("{:?}", claim);
            Ok(true)
        }
        Err(err) => {
            println!("decoding error: {:?}", err);
            Ok(false)
        }
    }
}
