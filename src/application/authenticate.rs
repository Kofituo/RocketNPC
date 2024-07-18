use crate::infrastructure::security::jwt_helper;
use crate::model::api_error;
use crate::model::api_error::APIError;
use crate::model::api_response;
use crate::model::{self, authenticate};
use chrono::Utc;
use rocket::serde::json::Json;
extern crate bcrypt;
use log::info;

const FILE: &str = "application/authenticate.rs";

pub fn authenticate(
    _login: authenticate::Login,
) -> Result<Json<api_response::ApiResponse>, APIError> {
    const METHOD: &str = "authenticate";

    if _login.userName == "admin" && _login.userPassword == "admin" {
        let authentication = model::authenticate::Authentication::new(1, 1, Utc::now());
        let token = jwt_helper::encode_token(authentication)?;
        info!(target:"app::login", "Login Sucesss for user : {}", _login.userName);
        let response: api_response::ApiResponse =
            api_response::ApiResponse::new(String::from(""), token, String::from(""), 0);
        return Ok(Json(response));
    }

    return Err(APIError::new(
        api_error::APIErrorTypes::AuthenticationError,
        format!("{}", "Invalid Password or Username"),
        FILE.to_string(),
        METHOD.to_string(),
        Utc::now(),
        format!("pasword = {}", "none".to_owned()),
        api_error::APIErrorCodes::APPAUTAUT05,
    ));
}
