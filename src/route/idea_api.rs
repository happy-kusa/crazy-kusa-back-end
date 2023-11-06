use crate::{database::mongodb::MongoRepo, models::idea_model::Idea};
use actix_web::{
    delete, get, post, put,
    web::{Data, Json, Path},
    HttpResponse,
};
use mongodb::bson::oid::ObjectId;

#[post("/idea")]
pub async fn create_idea(db: Data<MongoRepo>, new_idea: Json<Idea>) -> HttpResponse {
    let data = Idea {
        id: None,
        title: new_idea.title.to_owned(),
        author: new_idea.author.to_owned(),
        content: new_idea.content.to_owned(),
    };

    let idea_detail = db.create_idea(data).await;

    match idea_detail {
        Ok(idea) => HttpResponse::Ok().json(idea),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[get("/idea/{id}")]
pub async fn get_idea(db: Data<MongoRepo>, path: Path<String>) -> HttpResponse {
    let id = path.into_inner();
    if id.is_empty() {
        return HttpResponse::BadRequest().body("invalid ID");
    }
    let idea_detail = db.get_idea(&id).await;

    match idea_detail {
        Ok(idea) => HttpResponse::Ok().json(idea),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[put("/idea/{id}")]
pub async fn update_idea(
    db: Data<MongoRepo>,
    path: Path<String>,
    new_idea: Json<Idea>,
) -> HttpResponse {
    let id = path.into_inner();
    if id.is_empty() {
        return HttpResponse::BadRequest().body("invalid ID");
    };
    let data = Idea {
        id: Some(ObjectId::parse_str(&id).unwrap()),
        title: new_idea.title.to_owned(),
        author: new_idea.author.to_owned(),
        content: new_idea.content.to_owned(),
    };

    let update_result = db.update_idea(&id, data).await;

    match update_result {
        Ok(update) => {
            if update.matched_count == 1 {
                let updated_idea_info = db.get_idea(&id).await;

                return match updated_idea_info {
                    Ok(idea) => HttpResponse::Ok().json(idea),
                    Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
                };
            } else {
                return HttpResponse::NotFound().body("No idea found with specified ID");
            }
        }
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[delete("/idea/{id}")]
pub async fn delete_idea(db: Data<MongoRepo>, path: Path<String>) -> HttpResponse {
    let id = path.into_inner();
    if id.is_empty() {
        return HttpResponse::BadRequest().body("invalid ID");
    };
    let result = db.delete_idea(&id).await;

    match result {
        Ok(res) => {
            if res.deleted_count == 1 {
                return HttpResponse::Ok().json("Successfully deleted!");
            } else {
                return HttpResponse::NotFound().json("Specified ID not found!");
            }
        }
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[get("/ideas")]
pub async fn get_all_ideas(db: Data<MongoRepo>) -> HttpResponse {
    let ideas = db.get_all_ideas().await;

    match ideas {
        Ok(ideas) => HttpResponse::Ok().json(ideas),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
