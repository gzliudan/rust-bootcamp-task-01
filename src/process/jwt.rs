pub fn sign_jwt(sub: &str, aud: &str, exp: &str) {
    println!("sign jwt: sub={}, aud={}, exp={}", sub, aud, exp);
}

pub fn verify_jwt(token: &str) {
    println!("verify jwt: token={}", token)
}
