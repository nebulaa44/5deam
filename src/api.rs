#[derive(serde::Deserialize)]
pub struct Level
{
    pub data: String,
}

pub async fn get_level(id: i64) -> reqwest::Result<Level>
{
    let endpoint = format!("https://5beam.zelo.dev/api/level?id={id}");

    let body = reqwest::get(endpoint).await?.json().await?;
    Ok(body)
}