use mongodb::{
    bson::{doc, Document},
    options::{ClientOptions, ServerApi, ServerApiVersion},
    Client, results::InsertOneResult,
};
use futures::stream::TryStreamExt;

const MONGO_DB_LINK: &str = "mongodb+srv://chris:chris1234@chris-db.ec2i5ii.mongodb.net";
const MY_DATABASE: &str = "cocktail";
const MY_COLLECTION: &str = "recipes";

pub async fn connect() -> mongodb::error::Result<Client> {
    
    // 解析 MongoDB 連接字串
    let mut client_options = ClientOptions::parse(MONGO_DB_LINK,).await?;

    // 將 client_options 物件的 server_api 欄位設定為 Stable API 版本 1
    let server_api = ServerApi::builder().version(ServerApiVersion::V1).build();
    client_options.server_api = Some(server_api);

    // 取得叢集 client
    let client = Client::with_options(client_options)?;

    // ping 伺服器看看是否可以連接到集群
    client
        .database("admin")
        .run_command(doc! {"ping": 1}, None)
        .await?;

    // let document = doc! {
    //     "name": "Peter",
    //     "job": "學長"
    // };
    // match create(&client, MY_DATABASE, MY_COLLECTION, document).await {
    //     Ok(insert_result) => {
    //         println!("Successfully insert:/n{:?}", insert_result);
    //     }
    //     Err(err) => {
    //         eprintln!("Error inserting document: {:?}", err);
    //     }
    // }

    // 查找資料(指定 key 需要 _recipes)
    // let groceries_database = client.database(MY_DATABASE);
    // let _recipes = groceries_database.collection::<Document>(MY_COLLECTION);
    // let query = doc! {"name":"fishcan"};
    // if let Some(recipe_doc) = _recipes.find_one(query.clone(), None).await? {
    //     println!("chris 07-1 = {:?}", recipe_doc.get_str("job"));
    // } else {
    //     println!("chris 07-2");
    // }

    // 查找資料(All doc)
    let groceries_database = client.database(MY_DATABASE);
    let _recipes = groceries_database.collection::<Document>(MY_COLLECTION);
    let mut cursor = _recipes.find(
        None,
        None,
    ).await?;
    while let Some(_recipes) = cursor.try_next().await? {
        println!("chris 08 = {:?}", _recipes.get_str("job"));
    }

    Ok(client)
}

// 加入資料
pub async fn create(client: &Client, database_name: &str, collection_name: &str, document: Document,) -> mongodb::error::Result<InsertOneResult> {

    let groceries_database = client.database(database_name);
    let _recipes = groceries_database.collection::<Document>(collection_name);
    let oid = _recipes.insert_one(document, None).await?;

    Ok(oid)
}
