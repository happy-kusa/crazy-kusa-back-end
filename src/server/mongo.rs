use mongodb::{
    bson::{doc, Document},
    options::{ClientOptions, ServerApi, ServerApiVersion},
    Client,
};
use futures::stream::TryStreamExt;

const MONGO_DB_LINK: &str = "mongodb+srv://chris:chris1234@chris-db.ec2i5ii.mongodb.net/?retryWrites=true&w=majority";
const MY_DATABASE: &str = "cocktail";
const MY_COLLECTION: &str = "recipes";

pub async fn connect_to_mongodb() -> mongodb::error::Result<Client> {
    
    // 解析 MongoDB 連接字串
    println!("chris 01");
    let mut client_options = ClientOptions::parse(MONGO_DB_LINK,).await?;

    // 將 client_options 物件的 server_api 欄位設定為 Stable API 版本 1
    println!("chris 02");
    let server_api = ServerApi::builder().version(ServerApiVersion::V1).build();
    client_options.server_api = Some(server_api);

    // 取得叢集 client
    println!("chris 03");
    let client = Client::with_options(client_options)?;

    // ping 伺服器看看是否可以連接到集群
    println!("chris 04");
    client
        .database("admin")
        .run_command(doc! {"ping": 1}, None)
        .await?;

    // 加入資料(會產生一組OID)
    // println!("chris 05");
    // let groceries_database = client.database(MY_DATABASE);
    // let _recipes = groceries_database.collection::<Document>(MY_COLLECTION);

    // let insert_oid = _recipes.insert_one(doc! {
    //     "name":"chris",
    //     "job":"社畜"
    // }, None).await?;
    // println!("chris 06 = {:?} _recipes ={:?}", insert_oid, _recipes);

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
