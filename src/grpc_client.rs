pub mod services {
    tonic::include_proto!("services");
}

use services::{payment_service_client::PaymentServiceClient, PaymentRequest}; 

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = PaymentServiceClient::connect("http://[::1]:50051").await?; 
    let request = tonic::Request::new(PaymentRequest {
        user_id: "user_123".to_string(), 
        amount:100.00,
    }); 

    let response = client.process_payment(request).await?; 
    println!("RESPNOSE={:?}", response.into_inner());

    Ok(())
}