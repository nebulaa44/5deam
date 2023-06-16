mod api;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<() ,Box<dyn std::error::Error>>
{
    let resp = api::get_level(89).await?;
    println!("{resp}");

    Ok(())
}
