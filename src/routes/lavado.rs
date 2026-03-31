use rocket::form::Form;
use rocket::futures::SinkExt;
use rocket::serde::json::{Json, Value, json};
use rocket::tokio::time::{Duration, sleep};
use rocket::{FromForm, get, post};
use rocket_ws::{Channel, Message, WebSocket};
use uuid::Uuid;

#[derive(FromForm)]
pub struct AnalysisRequest<'r> {
    pub initial_wallet: String,
    pub final_wallet: String,
    pub use_url: bool,
    pub csv_url: Option<String>,
    pub csv_file: Option<String>,
    pub file: Option<rocket::fs::TempFile<'r>>,
}

#[post("/lavado", data = "<form_data>")]
pub async fn start_analysis(mut form_data: Form<AnalysisRequest<'_>>) -> Json<Value> {
    let job_id = Uuid::new_v4().to_string();

    if form_data.use_url {
        if let Some(url) = &form_data.csv_url {}
    } else if let Some(mut file) = form_data.file.take() {
        let file_path = format!("/tmp/analysis_{}.csv", job_id);

        match file.persist_to(&file_path).await {
            Ok(_) => todo!(),
            Err(e) => eprintln!("Error al guardar archivo del trabajo {}: {}", job_id, e),
        }
    }

    Json(json!({  "job_id": job_id, "status": "queued" }))
}

#[get("/analysis/<job_id>")]
pub fn analysis_ws(ws: WebSocket, job_id: String) -> Channel<'static> {
    ws.channel(move |mut stream| {
        Box::pin(async move {
            for i in 1..=5 {
                sleep(Duration::from_secs(1)).await;
                let msg = format!("Trabajo {}: Analizando profundidad {}", job_id, i);

                if stream.send(Message::Text(msg)).await.is_err() {
                    break;
                }
            }

            let _ = stream
                .send(Message::Text("Análisis finalizado".into()))
                .await;
            Ok(())
        })
    })
}
