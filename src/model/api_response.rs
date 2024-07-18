use chrono::DateTime;
use chrono::Utc;
use rocket::serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug)]
pub struct ApiResponse {
    return_description: String,
    return_result: String,
    return_message: String,
    return_code: i32,
}

impl ApiResponse {
    pub fn new(
        return_description: String,
        return_result: String,
        return_message: String,
        return_code: i32,
    ) -> ApiResponse {
        ApiResponse {
            return_description,
            return_result,
            return_message,
            return_code,
        }
    }
}

//  impl fmt::Display for ApiResponse {
//    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//        write!(f, "return_description = {} -  return_result = {}", self.transaction_id, self.user_id)
//    }
// }

#[derive(Serialize, Deserialize, Debug)]
pub struct ApiResponseError {
    pub return_description: String,
    pub return_timestamp: DateTime<Utc>,
    pub return_message: String,
    pub return_code: i32,
}

impl ApiResponseError {
    pub fn new(
        return_description: String,
        return_timestamp: DateTime<Utc>,
        return_message: String,
        return_code: i32,
    ) -> ApiResponseError {
        ApiResponseError {
            return_description,
            return_timestamp,
            return_message,
            return_code,
        }
    }
}
