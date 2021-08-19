use configureProvider::configure_provider_service_server::{ConfigureProviderService, ConfigureProviderServiceServer};

use configureProvider::{SetPolicyRequest, SetPolicyResponse};
use configureProvider::{SetRawPolicyRequest, SetRawPolicyResponse};
use configureProvider::{ExportPolicyRequest, ExportPolicyResponse};

use tonic::{transport::Server, Request, Response, Status};

pub mod configureProvider {
    tonic::include_proto!("configureprovider");
}

#[derive(Debug, Default)]
pub struct configProviderService {}

#[tonic::async_trait]
impl ConfigureProviderService for configProviderService {
    async fn set_policy(
        &self,
        request: Request<SetPolicyRequest>,
    ) -> Result<Response<SetPolicyResponse>, Status> {
        let empty = "".to_string();
        let request: SetPolicyRequest = request.into_inner();
        let policyname = std::str::from_utf8(&request.policyname)
            .unwrap_or_else(|_| {
                println!("parse policyname failed");
                &empty
            });
        let references = std::str::from_utf8(&request.references)
            .unwrap_or_else(|_| {
                println!("parse references failed");
                &empty
            });

        println!("{:?}", policyname);
        println!("{:?}", references);

        let res = SetPolicyResponse {
            status: "OK".as_bytes().to_vec(),
        };

        Ok(Response::new(res))
    }   

    async fn set_raw_policy(
        &self,
        request: Request<SetRawPolicyRequest>,
    ) -> Result<Response<SetRawPolicyResponse>, Status> {
        let empty = "".to_string();
        let request: SetRawPolicyRequest = request.into_inner();
        let policyname = std::str::from_utf8(&request.policyname)
            .unwrap_or_else(|_| {
                println!("parse policyname failed");
                &empty
            });
        let policycontent = std::str::from_utf8(&request.policycontent)
            .unwrap_or_else(|_| {
                println!("parse policycontent failed");
                &empty
            });
        
        println!("{:?}", policycontent);
        println!("{:?}", policyname);

        let res = SetRawPolicyResponse {
            status: "OK".as_bytes().to_vec(),
        };

        Ok(Response::new(res))
    }     

    async fn export_policy(
        &self,
        request: Request<ExportPolicyRequest>,
    ) -> Result<Response<ExportPolicyResponse>, Status> {
        let policyname = String::from_utf8(request.into_inner().policyname)
            .unwrap_or_else(|_| {
                println!("parse policyname failed");
                "".to_string()
            });

        println!("{:?}", policyname);

        let res = ExportPolicyResponse {
            status: "OK".as_bytes().to_vec(),
            policycontent: "hhhhhhhhhhhhhh".to_string().into_bytes(),
        };

        Ok(Response::new(res))
    }       
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let service = configProviderService::default();

    Server::builder()
        .add_service(ConfigureProviderServiceServer::new(service))
        .serve(addr)
        .await?;

    Ok(())
}