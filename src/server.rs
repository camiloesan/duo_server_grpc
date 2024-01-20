mod dal;

use crate::duo::user_manager_server::UserManagerServer;
use duo::user_manager_server::UserManager;
use duo::{Credentials, SuccessStatusResponse, UserDetails, UserInfoRequest, BooleanResponse, UsernameRequest, EmailRequest};
use tonic::{transport::Server, Request, Response, Status};

pub mod duo {
    tonic::include_proto!("duo");
}

#[derive(Debug, Default)]
pub struct DuoService {}

#[tonic::async_trait]
impl UserManager for DuoService {
    async fn add_user(
        &self,
        request: Request<UserInfoRequest>,
    ) -> Result<Response<SuccessStatusResponse>, Status> {
        let request_data = request.into_parts();
        let response = SuccessStatusResponse {
            success: dal::users::add_user(
                request_data.2.username,
                request_data.2.email,
                request_data.2.password,
            ),
        };
        Ok(Response::new(response))
    }

    async fn get_user(
        &self,
        request: Request<Credentials>,
    ) -> Result<Response<UserDetails>, Status> {
        let request_data = request.into_parts();
        let response =
            dal::users::get_user_by_credentials(request_data.2.username, request_data.2.password);
        Ok(Response::new(response))
    }

    async fn is_username_available(
        &self,
        request: Request<UsernameRequest>,
    ) -> Result<Response<BooleanResponse>, Status> {
        println!("username request");
        let request_data = request.into_parts();
        let response = BooleanResponse {
            response: dal::users::is_username_available(request_data.2.username),
        };
        Ok(Response::new(response))
    }

    async fn is_email_available(
        &self,
        request: Request<EmailRequest>,
    ) -> Result<Response<BooleanResponse>, Status> {
        println!("username request");
        let _request_data = request.into_parts();
        let response = BooleanResponse {
            response: dal::users::is_email_available(_request_data.2.email),
        };
        Ok(Response::new(response))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let address = "0.0.0.0:50051".parse()?;
    let service = DuoService::default();

    Server::builder()
        .add_service(UserManagerServer::new(service))
        .serve(address)
        .await?;
    Ok(())
}
