use aws_sdk_cloudfront::{
    Client,
    Error,
    model::{
        EventType,
        LambdaFunctionAssociation,
        LambdaFunctionAssociations,
    },
    output::GetDistributionConfigOutput,
};

type AwsResult = Result<(), Error>;

#[tokio::main]
async fn main() -> AwsResult {
    let sdk_config = aws_config::load_from_env().await;
    let client = Client::new(&sdk_config);

    // get_distribution_config(&client).await?;
    // set_basic_auth(&client).await?;

    Ok(())
}

async fn get_distribution_config(client: &Client) -> AwsResult {
    let req = client.get_distribution_config()
        .id("");
    let res = req.send().await?;

    if let Some(distribution_config) = res.distribution_config {
        // if let Some(default_cache_behavior) = distribution_config.default_cache_behavior {
        //     println!("{:?}", default_cache_behavior.lambda_function_associations);
        // }

        println!("{:#?}", distribution_config);
    }

    Ok(())
}

async fn set_basic_auth(client: &Client) -> AwsResult {
    let cf_id = "";

    let req = client.get_distribution_config()
        .id(cf_id);
    let res = req.send().await?;

    // if let Some(mut distribution_config) = res.distribution_config {
    if let GetDistributionConfigOutput {
        distribution_config: Some(mut distribution_config),
        e_tag: Some(e_tag),
        ..
    } = res {
        if let Some(ref mut default_cache_behavior) = distribution_config.default_cache_behavior {
            println!("{:?}", default_cache_behavior.lambda_function_associations);

            let lambda_function_association = LambdaFunctionAssociation::builder()
                .event_type(EventType::ViewerRequest)
                .lambda_function_arn("")
                .build();
            let lambda_function_associations = LambdaFunctionAssociations::builder()
                .quantity(1)
                .items(lambda_function_association)
                .build();

            default_cache_behavior.lambda_function_associations = Some(lambda_function_associations);
        }

        println!("{:#?}", distribution_config);

        let req = client.update_distribution()
            .id(cf_id)
            .distribution_config(distribution_config)
            .if_match(e_tag);
        let res = req.send().await?;

        println!("{:#?}", res);
    }

    Ok(())
}