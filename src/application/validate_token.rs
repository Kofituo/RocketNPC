use std::env;

use chrono::Utc;
use dotenv::dotenv;

use crate::infrastructure::security::jwt_helper;
use crate::model::api_error;
use crate::model::api_error::APIError;
use crate::model::authenticate;

const FILE: &str = "application/validate_token.rs";

//pub fn validate_token(token: &String) -> Result<bool, APIError>{
pub fn is_token_valid(token: &String) -> Result<bool, APIError> {
    const METHOD: &str = "validate_token";
    dotenv().ok();

    let session_timeout: i32 = env::var("SESSION_TIMEOUT")
        .expect("Session Time Out Should be set")
        .parse::<i32>()
        .unwrap();

    let authentication: authenticate::Authentication = jwt_helper::decode_token(&token)?;

    let authentication_datetime = authentication.last_authentication;

    let datetime_now_plus_session_time_out_opt = authentication_datetime
        .checked_add_signed(chrono::Duration::minutes(session_timeout.into()));

    let mut datetime_now_plus_session_time_out = Utc::now();

    match datetime_now_plus_session_time_out_opt {
        None => {
            return Err(APIError::new(
                api_error::APIErrorTypes::GeneralException,
                "Failed to add session time_out to last authentication time".to_owned(),
                FILE.to_string(),
                METHOD.to_string(),
                Utc::now(),
                format!("token= {}", token.to_owned()),
                api_error::APIErrorCodes::APPVTOVTO01,
            ))
        }
        Some(v) => datetime_now_plus_session_time_out = v,
    }
    if datetime_now_plus_session_time_out < Utc::now() {
        return Ok(false);
    }
    Ok(true)
}

pub fn validate_token(api_params: &String, token: &String) -> Result<(), APIError> {
    const METHOD: &str = "validate_token";
    let is_token_valid_result = is_token_valid(&token);
    if let Err(e) = is_token_valid_result {
        //return Err(error_handler::handle_error_struct(e, &String::from("Failed Validating Token"), &String::from("API Execution failed.")))
        return Err(APIError::new(
            api_error::APIErrorTypes::GeneralException,
            "Failed Validating Token".to_owned(),
            FILE.to_string(),
            METHOD.to_string(),
            Utc::now(),
            format!("token= {}", token.to_owned()),
            api_error::APIErrorCodes::APPVTOVTO02,
        ));
    }
    let is_token_valid = is_token_valid_result.unwrap();
    if is_token_valid == false {
        //return Err(error_handler::handle_error(format!("{} - {}", api_error::APIErrorCodes::MAIVTO02.to_string(), String::from("Token Expired")), format!("Error: {} - File: {} - Method: {}", "Token Expired", FILE, METHOD),api_error::APIErrorTypes::GeneralException, api_params.to_string()))
        return Err(APIError::new(
            api_error::APIErrorTypes::GeneralException,
            "Token Expired".to_owned(),
            FILE.to_string(),
            METHOD.to_string(),
            Utc::now(),
            format!("token= {}", token.to_owned()),
            api_error::APIErrorCodes::APPVTOVTO03,
        ));
    }
    Ok(())
}
