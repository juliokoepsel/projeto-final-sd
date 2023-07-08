#[derive(Serialize, Deserialize, Debug)]
pub struct CarroRequest {
    pub placa: String,
    pub horas: i32,
    pub preco_hora: f32,
}

pub async fn carros_list_handler(db: DB) -> WebResult<impl Reply> {
    let carros = db.fetch_carros().await.map_err(|e| reject::custom(e))?;
    Ok(json(&carros))
}

pub async fn create_carro_handler(body: CarroRequest, db: DB) -> WebResult<impl Reply> {
    db.create_carro(&body).await.map_err(|e| reject::custom(e))?;
    Ok(StatusCode::CREATED)
}

pub async fn edit_carro_handler(id: String, body: CarroRequest, db: DB) -> WebResult<impl Reply> {
    db.edit_carro(&id, &body)
        .await
        .map_err(|e| reject::custom(e))?;
    Ok(StatusCode::OK)
}

pub async fn delete_carro_handler(id: String, db: DB) -> WebResult<impl Reply> {
    db.delete_carro(&id).await.map_err(|e| reject::custom(e))?;
    Ok(StatusCode::OK)
}