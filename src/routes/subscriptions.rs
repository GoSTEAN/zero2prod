use actix_web::{web, HttpResponse};
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
    pool: web::Data<PgPool>,  // Changed to PgPool
) -> HttpResponse {
    // Log the incoming request data
    tracing::info!(
        "Received subscription request - email: {}, name: {}",
        form.email,
        form.name
    );

    // Log the database connection info
    tracing::info!(
        "Attempting database operation with pool: {:?}",
        pool.as_ref()
    );

    match insert_subscriber(&pool, &form).await
    {
        Ok(_) => {
            tracing::info!("New subscriber details have been saved");
            HttpResponse::Ok().finish()
        }
        Err(e) => {
            tracing::error!(
                "Failed to add subscriber: {:#?}. email={}, name={}", 
                e, 
                form.email, 
                form.name
            );
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[tracing::instrument(
    name = "Saving new subscriber details in the database", 
    skip(form, pool),
    fields(
        request_id = %Uuid::new_v4(),
        subscriber_email = %form.email,
        subscriber_name = %form.name,
    )
)]
pub async fn insert_subscriber(
        pool: &PgPool,
        form: &FormData
    ) -> Result<(), sqlx::Error> {
        let id = Uuid::new_v4();
        let now = Utc::now();
        
        tracing::info!(
            "Executing INSERT query with values - id: {}, email: {}, name: {}, time: {}",
            id,
            form.email,
            form.name,
            now
        );

        sqlx::query!(
            r#"
            INSERT INTO subscriptions (id, email, name, subscribed_at)
            VALUES ($1, $2, $3, $4)
            "#,
            id,
            form.email,
            form.name,
            now
        )
        .execute(pool)
        .await
        .map_err(|e| {
            tracing::error!("Database error details: {:#?}", e);
            e
        })?;
        Ok(())
}