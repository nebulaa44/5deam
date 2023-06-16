mod api;
mod init_levels;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args
{
    //? These should probably be subcommands

    /// Creates a new levels.txt at the given path
    #[arg(short, long, default_value_t = format!(""))]
    generate_levels: String,

    /// Level ID to download
    #[arg(short, long, default_value_t = 0)]
    id: i64,
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>>
{
    let args = Args::parse();

    if args.generate_levels != "" 
    {
        init_levels::init_levels();
    }

    if args.id != 0
    {
        let resp = api::get_level(args.id).await?.data;
        println!("{resp}");
    }
    Ok(())
}
