use std::env;
extern crate dotenv;

use dotenv::dotenv;

use futures::stream::TryStreamExt;
use mongodb::{
    bson::{doc, extjson::de::Error, oid::ObjectId},
    results::{DeleteResult, InsertOneResult, UpdateResult},
    Client, Collection,
};

use crate::models::idea_model::Idea;

pub struct MongoRepo {
    col: Collection<Idea>,
}

impl MongoRepo {
    pub async fn init() -> Self {
        dotenv().ok();
        let url = match env::var("MONGOURI") {
            Ok(v) => v.to_string(),
            Err(_) => format!("Error loading env variable"),
        };
        let client = Client::with_uri_str(url)
            .await
            .expect("error connecting to database");
        let db = client.database("crazyDB");
        let col: Collection<Idea> = db.collection("myIdea");
        MongoRepo { col }
    }

    pub async fn create_idea(&self, new_idea: Idea) -> Result<InsertOneResult, Error> {
        let new_doc = Idea {
            id: None,
            title: new_idea.title,
            author: new_idea.author,
            content: new_idea.content,
        };
        let idea = self
            .col
            .insert_one(new_doc, None)
            .await
            .ok()
            .expect("Error creating idea");

        Ok(idea)
    }

    pub async fn get_idea(&self, id: &String) -> Result<Idea, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let idea_detail = self
            .col
            .find_one(filter, None)
            .await
            .ok()
            .expect("Error getting idea's detail");

        Ok(idea_detail.unwrap())
    }

    pub async fn update_idea(&self, id: &String, new_idea: Idea) -> Result<UpdateResult, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let new_doc = doc! {
            "$set":
                {
                    "title": new_idea.title,
                    "author": new_idea.author,
                    "content": new_idea.content
                },
        };
        let updated_doc = self
            .col
            .update_one(filter, new_doc, None)
            .await
            .ok()
            .expect("Error updating idea");
        Ok(updated_doc)
    }

    pub async fn delete_idea(&self, id: &String) -> Result<DeleteResult, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let idea_detail = self
            .col
            .delete_one(filter, None)
            .await
            .ok()
            .expect("Error deleting idea");

        Ok(idea_detail)
    }

    pub async fn get_all_ideas(&self) -> Result<Vec<Idea>, Error> {
        let mut cursors = self
            .col
            .find(None, None)
            .await
            .ok()
            .expect("Error getting list of ideas");
        let mut ideas: Vec<Idea> = Vec::new();
        while let Some(idea) = cursors
            .try_next()
            .await
            .ok()
            .expect("Error mapping through cursor")
        {
            ideas.push(idea)
        }
        Ok(ideas)
    }
}
