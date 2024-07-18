use crate::infrastructure::security::jwt_helper;
use crate::model::api_error;
use crate::model::api_error::APIError;
use crate::model::api_response;
use crate::model::{self, authenticate};
use capnp::capability::{Promise, Response};
use capnp::Error;
use capnp_rpc::rpc_twoparty_capnp::Side;
use capnp_rpc::RpcSystem;
use chrono::Utc;
use rocket::serde::json::Json;
extern crate bcrypt;
use crate::infrastructure::capnp_rpc::client::{new_capnp_client, run_client, RpcResponse};
use crate::model::api_response::ApiResponse;
use crate::ocs365_capnp;
use crate::ocs365_capnp::authenticate::authenticate_results;
use log::info;

const FILE: &str = "application/authenticate.rs";

pub async fn authenticate(
    login: authenticate::Login,
) -> Result<Json<api_response::ApiResponse>, APIError> {
    const METHOD: &str = "authenticate";

    if login.userName == "admin" && login.userPassword == "admin" {
        let authentication = model::authenticate::Authentication::new(1, 1, Utc::now());
        let token = jwt_helper::encode_token(authentication)?;
        info!(target:"app::login", "Login Sucesss for user : {}", login.userName);
        let rpc_response = run_client::<ocs365_capnp::authenticate::Client>(login).await;
        println!("response {:?}", rpc_response);
        let response: api_response::ApiResponse = api_response::ApiResponse::new(
            rpc_response.unwrap_or_default(),
            token,
            String::from(""),
            0,
        );
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

impl RpcResponse for ocs365_capnp::authenticate::Client {
    type InputData = authenticate::Login;
    type CapNpResult = authenticate_results::Owned;

    fn new(rpc_system: RpcSystem<Side>) -> Self {
        new_capnp_client(rpc_system)
    }

    fn get_promise(
        self,
        data: Self::InputData,
    ) -> Promise<Response<Self::CapNpResult>, Error> {
        let mut request = self.authenticate_request();
        let mut builder = request.get().init_auth();
        builder.set_user_name(&data.userName);
        builder.set_user_password(&data.userPassword);
        request.send().promise
    }
    fn extract_response(response: Response<Self::CapNpResult>) -> capnp::Result<String> {
        response
            .get()?
            .get_result()?
            .get_description()?
            .to_string()
            .map_err(|e| capnp::Error::failed(e.to_string()))
    }
}
