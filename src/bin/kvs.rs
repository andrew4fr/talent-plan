use clap::{App, Arg, SubCommand};

fn main() {
    let matches = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .subcommand(
            SubCommand::with_name("get")
                .about("Get value of a given key")
                .arg(Arg::with_name("KEY").help("string key").required(true)),
        )
        .subcommand(
            SubCommand::with_name("set")
                .about("Set value of a key")
                .arg(Arg::with_name("KEY").help("string key").required(true))
                .arg(Arg::with_name("VALUE").help("string value").required(true)),
        )
        .subcommand(
            SubCommand::with_name("rm")
                .about("Remove given key")
                .arg(Arg::with_name("KEY").help("string key").required(true)),
        )
        .get_matches();

    if matches.args.is_empty() && matches.subcommand.is_none() {
        std::process::exit(-1)
    }

    // println!("{:?}", matches);

    if let Some(m) = matches.subcommand_matches("get") {
        if m.is_present("key") {}
    }
}
