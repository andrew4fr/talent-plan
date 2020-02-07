use structopt::StructOpt;
use kvs::{KvStore, KvsError, Result};
use std::process::exit;
use std::env::current_dir;

#[derive(StructOpt, Debug)]
#[structopt(
    name = env!("CARGO_PKG_NAME"), 
    version = env!("CARGO_PKG_VERSION"),
    author = env!("CARGO_PKG_AUTHORS"),
    about = env!("CARGO_PKG_DESCRIPTION")
)]
struct Opt {
    #[structopt(subcommand)]
    cmd: Command,
}

#[derive(StructOpt, Debug)]
enum Command {
    #[structopt(about = "Get value of a key", help = "string key")]
    Get { key: String },
    #[structopt(about = "Set value of a key", help = "string key")]
    Set { key: String, value: String },
    #[structopt(about = "Remove a key from memory", help = "string key")]
    Rm { key: String },
}

fn main() -> Result<()> {
    let opt = Opt::from_args();

    let store = KvStore::open(current_dir()?)?;

    match opt.cmd {
        Command::Set { key, value } => {
            store.set(key, value)?;
        },
        Command::Get { key } => {
            if let Some(val) = store.get(key)? {
                println!("{}", val);
            } else {
                println!("Key not found");
            }
        },
        Command::Rm { key } => {
            match store.remove(key) {
                Ok(()) => {},
                Err(KvsError::KeyNotFound) => {
                    println!("Key not found");
                    exit(1);
                },
                Err(e) => return Err(e),
            }
        }
    }

    Ok(())
}
