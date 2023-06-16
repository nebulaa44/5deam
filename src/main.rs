mod api;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args
{
    #[arg(short, long)]
    id: i64,
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<() ,Box<dyn std::error::Error>>
{
    let args = Args::parse();

    let resp = api::get_level(args.id).await?;
    println!("{resp}");

    Ok(())
}
