use pico_args::Arguments;
use anyhow::Error;

mod config;

use config::Args;

fn main() -> Result<(), Error> {
    let mut parsed = Arguments::from_env();

    let args = Args{
        command: parsed.subcommand()?
    };

    if let Some(command) = args.command {

        match command.as_str() {
            "publish" => {}
            "subscribe" => {}
            _ => {
                println!("unrecognized command: {}", command);
                std::process::exit(1);
            }
        }

    }



    // watch <path> --exec ""

    // send <path>

    Ok(())
}



