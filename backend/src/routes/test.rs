use axum::{Extension, Json};
use sqlx::PgPool;
use mongodb::Database;
use serde::Serialize;
use utoipa::ToSchema;
#[derive(Serialize, ToSchema)]
pub struct HealthStatus {
    postgres: bool,
    mongodb: bool,
}


#[utoipa::path(
    get,
    path = "/test",
    tag = "talky",
    responses(
        (status = 200, description =  "Health check status", body = HealthStatus)
    )
)]
pub async fn health_check(                                                                                                                                                               
      Extension(pg): Extension<PgPool>,                                                                                                                                                
      Extension(mongo): Extension<Database>,                                                                                                                                           
  ) -> Json<HealthStatus> {                                                                                                                                                            
      // Test PostgreSQL avec une requête simple                                                                                                                                       
      let pg_ok = sqlx::query("SELECT 1")                                                                                                                                              
          .fetch_one(&pg)                                                                                                                                                              
          .await                                                                                                                                                                       
          .is_ok();                                                                                                                                                                    
                                                                                                                                                                                       
      // Test MongoDB avec un ping                                                                                                                                                     
      let mongo_ok = mongo                                                                                                                                                             
          .run_command(mongodb::bson::doc! { "ping": 1 })                                                                                                                              
          .await                                                                                                                                                                       
          .is_ok();                                                                                                                                                                    
                                                                                                                                                                                       
      Json(HealthStatus {                                                                                                                                                              
          postgres: pg_ok,                                                                                                                                                             
          mongodb: mongo_ok,                                                                                                                                                           
      })                                                                                                                                                                               
  }                                                                                                                                                                                 