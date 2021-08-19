use clap::{App, Arg, SubCommand};
use std::collections::HashMap;
use std::ptr::null;

use tonic::{transport::Server, Request, Response, Status};

use configureProvider::configure_provider_service_client::ConfigureProviderServiceClient;

use configureProvider::{ExportPolicyRequest, ExportPolicyResponse};
use configureProvider::{SetPolicyRequest, SetPolicyResponse};
use configureProvider::{SetRawPolicyRequest, SetRawPolicyResponse};

pub mod configureProvider {
    tonic::include_proto!("configureprovider");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = App::new("verdict")
        .version("0.1")
        .author("Inclavare-Containers Team")
        .arg(
            Arg::with_name("set_policy")
                .short("s")
                .long("set_policy")
                .value_name("POLICY_NAME")
                .value_name("FILE_REFERENCE")
                .help("Generate a policy file named <POLICY_NAME>, according to the contents in <FILE_REFERENCE>.")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("set_raw_policy")
                .short("r")
                .long("set_raw_policy")
                .value_name("POLICY_NAME")
                .value_name("FILE")
                .help("Write the contents of <FILE> into the policy file named <POLICY_NAME>.")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("export_policy")
                .short("e")
                .long("export_policy")
                .value_name("POLICY_NAME")
                .value_name("FILE")
                .help("Export the contents of the policy file named <POLICY_NAME> into a file named <FILE>.")
                .takes_value(true),
        )
        .get_matches();

    let mut client = ConfigureProviderServiceClient::connect("http://[::1]:60000").await?;

    //set_policy
    /*{
        let reference = json!({
            "mrEnclave": [
                "123",
                "456",
                "111",
                "222"
            ],
            "mrSigner": [
                "123",
                "456",
                "111",
                "222"
            ],
            "productId": {
                ">=": 5
            },
            "svn": {
                ">=": 1
            },
        });
        let request = SetPolicyRequest {
            policyname: "attestation.rego".as_bytes().to_vec(),
            references: reference.to_string().into_bytes(),
        };
        let response = client.set_policy(request).await?;
        let response: SetPolicyResponse = response.into_inner();
        let status = response.status;
        println!("SetPolicy status is: {:?}", String::from_utf8(status).unwrap());
    }*/

    //set_raw_policy
    /*{
        let policy = "package policy\n\n\
        mrEnclave = \"123\"\n\
        mrSigner = \"456\"\n\
        productId = 1\n\n\

        default allow = false\n\n\

        allow = true {\n\
            \tinput.mrEnclave == \"123\"\n\
            \tinput.mrSigner == \"456\"\n\
            \tinput.productId == 1\n\
        }";
        let request = SetRawPolicyRequest {
            policyname: "raw.rego".as_bytes().to_vec(),
            policycontent: policy.to_string().into_bytes(),
        };
        let response = client.set_raw_policy(request).await?;
        let response: SetRawPolicyResponse = response.into_inner();
        let status = response.status;
        println!("SetRawPolicy status is: {:?}", String::from_utf8(status).unwrap());
    }*/

    //export_policy
    /*{
        let policyname = "attestation.rego";
        let request = ExportPolicyRequest {
            policyname: policyname.as_bytes().to_vec(),
        };
        let response = client.export_policy(request).await?;
        let response: ExportPolicyResponse = response.into_inner();
        let status = &response.status;
        println!(
            "export_policy status is: {}",
            std::str::from_utf8(status).unwrap()
        );
        let policy = String::from_utf8(response.policycontent).unwrap();
        println!("policy: {} content is:\n{}", policyname, policy);
    }*/

    Ok(())
}
