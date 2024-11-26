use actix_web::{web, HttpResponse, Responder};
use sqlx::PgPool;  // Changed from PgConnection to PgPool
use chrono::Utc;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct FormData {
    pub email: String,
    pub name: String
}

#[tracing::instrument(
    name = "Adding a new subscriber",
    skip(form, pool),
    fields(
        request_id = %Uuid::new_v4(),
        subscriber_email = %form.email,
        subscriber_name = %form.name
    )
)]

pub async fn subscribe(
    form: web::Form<FormData>,
    pool: web::Data<PgPool>,
) -> impl Responder {
    let result = sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now()
    )
    .execute(pool.get_ref())
    .await;

    match result {
        Ok(_) => {
            tracing::info!("New subscriber details have been saved");
            HttpResponse::Ok().finish()
        }
        Err(e) => {
            // Check if this is a unique violation error
            if let Some(db_error) = e.as_database_error() {
                if db_error.code().as_deref() == Some("23505") {  // Postgres unique violation code
                    tracing::warn!(
                        "Attempt to subscribe with existing email: {}",
                        form.email
                    );
                    return HttpResponse::BadRequest()
                        .content_type("text/plain")                        
                        .body("Email already exists");

                }
            }
            
            // For other errors, log and return 500
            tracing::error!("Failed to execute query: {:?}", e);
            HttpResponse::InternalServerError()
                .content_type("text/plain")
                .body("Internal server error")
        }
    }
}

#[tracing::instrument(
    name = "Saving new subscriber details in the database",
    skip(form, pool)
)]
pub async fn insert_subscriber(
    pool: &PgPool,
    form: &FormData,
) -> Result<(), sqlx::Error> {
    let query = sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now()
    );

    // Log the query details
    tracing::info!("Executing database query");
    
    match query.execute(pool).await {
        Ok(result) => {
            tracing::info!("Query successful, rows affected: {}", result.rows_affected());
            Ok(())
        }
        Err(e) => {
            tracing::error!("Query failed: {:?}", e);
            Err(e)
        }
    }
}