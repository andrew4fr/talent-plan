use structopt::StructOpt;

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

fn main() {
    /*
    if let Some(m) = matches.subcommand_matches("get") {
        if m.is_present("key") {}
    }
    */

    let opt = Opt::from_args();
    println!("{:#?}", opt);

    match opt.cmd {
        Command::Set { key, value } => unimplemented!(),
        Command::Get { key } => unimplemented!(),
        Command::Rm { key } => unimplemented!(),
        _ => unreachable!(),
    }
}
