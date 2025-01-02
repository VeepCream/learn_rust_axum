use std::sync::Arc;

use axum::{extract::State, response::IntoResponse, routing::post, Json, Router};

use crate::{application::usecases::adventurers::AdventurersUseCase, domain::{repositories::adventurers::AdventurersRepository, value_objects::adventurer_model::RegisterAdventurerModel}, infrastructure::postgres::{postgres_connector::PgPoolSquad, repositories::adventurers::AdventurersPostgres}};

pub fn routes (db_pool: Arc<PgPoolSquad>) -> Router {

    let adventurers_repository = AdventurersPostgres::new(db_pool);
    let adventures_use_case = AdventurersUseCase::new(Arc::new(adventurers_repository));

    Router::new()
        .route("/", post(register))
        .with_state(Arc::new(adventures_use_case))

}   

pub async fn register<T>(
    State(adventurer_suse_case): State<Arc<AdventurersUseCase<T>>>,
    Json(register_adventurer_model): Json<RegisterAdventurerModel>,
)-> impl IntoResponse 
where 
    T: AdventurersRepository + Send + Sync,
{
    unimplemented!()
}