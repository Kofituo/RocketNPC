use std::env;

use chrono::Utc;
use dotenv::dotenv;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};

use crate::model::api_error;
use crate::model::api_error::*;
use crate::model::authenticate::Authentication;

const FILE: &str = "infrastructure/security/jwt_helper.rs";

pub fn encode_token(authentication: Authentication) -> Result<String, APIError> {
    const METHOD: &str = "encode_token";
    let token_secret = get_secret_env_key()?;
    let token_result = encode(
        &Header::default(),
        &authentication,
        &EncodingKey::from_secret(token_secret.as_ref()),
    );
    if let Err(e) = token_result {
        return Err(APIError::new(
            api_error::APIErrorTypes::AuthenticationError,
            format!("{}-{}", e.to_string(), "Failed to Encode Token"),
            FILE.to_string(),
            METHOD.to_string(),
            Utc::now(),
            authentication.to_string(),
            api_error::APIErrorCodes::INFSECJWTETO02,
        ));
    }
    Ok(token_result.unwrap())
}

pub fn decode_token(token: &String) -> Result<Authentication, APIError> {
    const METHOD: &str = "decode_token";
    let token_secret = get_secret_env_key()?;
    let mut validation = Validation::new(Algorithm::HS256);
    validation.validate_exp = false;
    //let validation = Validation { validate_exp: false, ..Validation::default() };
    let token_decode_res = decode::<Authentication>(
        token,
        &DecodingKey::from_secret(token_secret.as_ref()),
        &validation,
    );
    if let Err(e) = token_decode_res {
        println!("error");
        return Err(APIError::new(
            api_error::APIErrorTypes::AuthenticationError,
            format!("{}-{}", e.to_string(), "SECRET Env Key Required"),
            FILE.to_string(),
            METHOD.to_string(),
            Utc::now(),
            String::from(""),
            api_error::APIErrorCodes::INFSECJWTETO03,
        ));
    };
    let authentication = token_decode_res.unwrap().claims;
    Ok(authentication)
}

fn get_secret_env_key() -> Result<String, APIError> {
    dotenv().ok();
    const METHOD: &str = "get_secret_env_key";
    let token_secret_res = env::var("SECRET");
    if let Err(e) = token_secret_res {
        return Err(APIError::new(
            api_error::APIErrorTypes::AuthenticationError,
            format!("{}-{}", e.to_string(), "SECRET Env Key Required"),
            FILE.to_string(),
            METHOD.to_string(),
            Utc::now(),
            String::from(""),
            api_error::APIErrorCodes::INFSECJWTETO01,
        ));
    };
    Ok(token_secret_res.unwrap())
}
