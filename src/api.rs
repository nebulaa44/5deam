pub async fn get_level(id: i64) -> reqwest::Result<String>
{
    let endpoint = format!("https://5beam.zelo.dev/api/level?id={id}");

    let body = reqwest::get(endpoint).await?.text().await?;
    Ok(body)
}