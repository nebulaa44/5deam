mod api;
mod levels_txt;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args
{
    //? These should probably be subcommands

    /// Creates a new levels.txt at the given path
    #[arg(short, long)]
    generate_levels: bool,

    /// Automatically add level to levels.txt
    #[arg(short, long)]
    add: bool,

    /// Level ID to download
    #[arg(short, long, default_value_t = 0)]
    id: i64,
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>>
{
    let args = Args::parse();

    if args.generate_levels
    {
        levels_txt::init_levels().unwrap();
    }

    if args.id != 0
    {
        let resp = api::get_level(args.id).await?.data + "\r\n\r\n";
        if !args.add
        {
            println!("{resp}");
        }

        levels_txt::add_level(resp)?;
    }
    Ok(())
}
