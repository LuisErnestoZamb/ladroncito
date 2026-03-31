use rocket::form::Form;
use rocket::futures::SinkExt;
use rocket::serde::json::{Json, Value, json};
use rocket::{FromForm, get, post};
use rocket_ws::{Channel, Message, WebSocket};
use uuid::Uuid;

use crate::services::pull_file::pull_file;
use crate::services::web_search::sort_search;

#[derive(FromForm)]
pub struct AnalysisRequest<'r> {
    pub initial_wallet: String,
    pub final_wallet: String,
    pub use_url: bool,
    pub csv_url: Option<String>,
    pub csv_file: Option<String>,
    pub file: Option<rocket::fs::TempFile<'r>>,
}

#[derive(Debug, FromForm)]
struct WalletParams {
    initial_wallet: String,
    final_wallet: String,
}

#[post("/lavado", data = "<form_data>")]
pub async fn start_analysis(mut form_data: Form<AnalysisRequest<'_>>) -> Json<Value> {
    let job_id = Uuid::new_v4().to_string();

    if form_data.use_url {
        if let Some(url) = &form_data.csv_url {
            let _ = pull_file(url, job_id.clone()).await;
        }
    } else if let Some(mut file) = form_data.file.take() {
        let file_path = format!("/tmp/analysis_{}.csv", job_id);

        match file.persist_to(&file_path).await {
            Ok(_) => println!(
                "Trabajo {}: Archivo guardado exitosamente en {}",
                job_id, file_path
            ),
            Err(e) => eprintln!("Error al guardar archivo del trabajo {}: {}", job_id, e),
        }
    }

    Json(json!({  "job_id": job_id, "status": "queued" }))
}

#[get("/analysis/<job_id>?<params..>")]
pub fn analysis_ws(ws: WebSocket, job_id: String, params: WalletParams) -> Channel<'static> {
    ws.channel(move |mut stream| {
        Box::pin(async move {
            let msg = format!(
                "Trabajo {}: Analizando desde {} hasta {}",
                job_id, params.initial_wallet, params.final_wallet
            );
            let _ = stream.send(Message::Text(msg)).await;

            match sort_search(&params.initial_wallet, &params.final_wallet, &job_id).await {
                Ok(path) => {
                    let msg = json!({
                        "status": "success",
                        "path": path,
                        "hops": path.len() - 1
                    })
                    .to_string();
                    let _ = stream.send(Message::Text(msg)).await;
                }
                Err(e) => {
                    let msg = json!({ "status": "error", "message": e }).to_string();
                    let _ = stream.send(Message::Text(msg)).await;
                }
            }

            let _ = stream
                .send(Message::Text("Análisis finalizado".into()))
                .await;
            Ok(())
        })
    })
}
