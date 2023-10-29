use mongodb::bson::{doc, document::Document, oid::ObjectId, Bson};
use mongodb::{options::ClientOptions, Client, Collection};

const MONGO_DB_LINK: &str = "mongodb+srv://chris:chris1234@chris-db.ec2i5ii.mongodb.net";

const DB_NAME: &str = "booky";
const COLL: &str = "books";

const ID: &str = "_id";
const NAME: &str = "name";
const AUTHOR: &str = "author";
const NUM_PAGES: &str = "num_pages";
const ADDED_AT: &str = "added_at";
const TAGS: &str = "tags";

#[derive(Clone, Debug)]
pub struct DB {
    pub client: Client,
}

impl DB {
    #[derive(Serialize, Deserialize, Debug)]
    pub struct Book {
        pub id: String,
        pub name: String,
        pub author: String,
        pub num_pages: usize,
        pub added_at: DateTime<Utc>,
        pub tags: Vec<String>,
    }

    pub async fn init() -> Result<Self> {
        let mut client_options = ClientOptions::parse(MONGO_DB_LINK).await?;
        client_options.app_name = Some("booky".to_string());
        Ok(Self {
            client: Client::with_options(client_options)?,
        })
    }

    pub async fn fetch_books(&self) -> Result<Vec<Book>> {
        let mut cursor = self
            .get_collection()
            .find(None, None)
            .await
            .map_err(MongoQueryError)?;

        let mut result: Vec<Book> = Vec::new();
        while let Some(doc) = cursor.next().await {
            result.push(self.doc_to_book(&doc?)?);
        }
        Ok(result)
    }

    fn get_collection(&self) -> Collection {
        self.client.database(DB_NAME).collection(COLL)
    }

    fn doc_to_book(&self, doc: &Document) -> Result<Book> {
        let id = doc.get_object_id(ID)?;
        let name = doc.get_str(NAME)?;
        let author = doc.get_str(AUTHOR)?;
        let num_pages = doc.get_i32(NUM_PAGES)?;
        let added_at = doc.get_datetime(ADDED_AT)?;
        let tags = doc.get_array(TAGS)?;

        let book = Book {
            id: id.to_hex(),
            name: name.to_owned(),
            author: author.to_owned(),
            num_pages: num_pages as usize,
            added_at: *added_at,
            tags: tags
                .iter()
                .filter_map(|entry| match entry {
                    Bson::String(v) => Some(v.to_owned()),
                    _ => None,
                })
                .collect(),
        };
        Ok(book)
    }
}
    