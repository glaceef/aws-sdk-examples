use aws_sdk_dynamodb::{
    Client,
    Error,
    model::AttributeValue,
};

type AwsResult = Result<(), Error>;

#[tokio::main]
async fn main() -> AwsResult {
    let sdk_config = aws_config::load_from_env().await;
    let client = Client::new(&sdk_config);

    // https://docs.rs/aws-sdk-dynamodb/0.12.0/aws_sdk_dynamodb/struct.Client.html

    // list_tables(&client).await?;
    // scan(&client).await?;

    Ok(())
}

async fn list_tables(client: &Client) -> AwsResult {
    // https://docs.rs/aws-sdk-dynamodb/0.12.0/aws_sdk_dynamodb/client/fluent_builders/struct.ListTables.html

    let req = client.list_tables()
        .limit(10);
    let res = req.send().await?;

    println!("{:?}", res);

    Ok(())
}

async fn scan(client: &Client) -> AwsResult {
    // https://docs.rs/aws-sdk-dynamodb/0.12.0/aws_sdk_dynamodb/client/fluent_builders/struct.Scan.html

    let req = client.scan()
        .table_name("table_name")
        .filter_expression("")
        .expression_attribute_names(":name1", "value1") // 1回の呼び出しで1組の追加を行う。複数の組を登録する場合は複数回実行する。
        .expression_attribute_names(":name2", "value2") // 上書きする場合は set_expression_attribute_names を呼び出す。
        .expression_attribute_values(
            ":zero",
            AttributeValue::N("0".into()), // https://docs.rs/aws-sdk-dynamodb/0.12.0/aws_sdk_dynamodb/model/enum.AttributeValue.html
        );
    let res = req.send().await?;

    println!("{:?}", res);

    // .send() は自身を消費するため、ループさせる場合は req.clone をしておく。

    Ok(())
}