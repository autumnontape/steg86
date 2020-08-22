use anyhow::Result;
use clap::{App, AppSettings, Arg};

mod steg86;

trait AddCommonArguments {
    fn add_common_arguments(self) -> Self;
}

impl<'b> AddCommonArguments for App<'b> {
    fn add_common_arguments(self) -> Self {
        self.arg(
            Arg::with_name("raw")
                .about("treat the input as a raw binary")
                .long("raw")
                .short('r'),
        )
        .arg(
            Arg::with_name("bitness")
                .about("the bitness of the raw binary")
                .long("bitness")
                .short('b')
                .takes_value(true)
                .possible_values(&["16", "32", "64"])
                .requires("raw"),
        )
        .arg(
            Arg::with_name("safe-ranges")
                .about("path to a CSV file containing safe ranges")
                .long_about(
                    "A path to a CSV file containing safe ranges. Each row must
contain two values, the first being an offset into the text
section and the second being a length, both in bytes.
Numbers are parsed as hexadecimal if they begin with \"0x\"
and decimal otherwise. Unsorted and overlapping ranges are
permitted.",
                )
                .long("safe-ranges")
                .short('s')
                .takes_value(true),
        )
    }
}

fn main() -> Result<()> {
    env_logger::init();

    let matches = App::new(env!("CARGO_PKG_NAME"))
        .setting(AppSettings::VersionlessSubcommands)
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .version(env!("CARGO_PKG_VERSION"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .subcommand(
            App::new("profile")
                .about("profile a binary for steganographic storage capacity")
                // TODO(ww): --json flag.
                .add_common_arguments()
                .arg(
                    Arg::with_name("input")
                        .about("the binary to profile")
                        .index(1)
                        .required(true),
                ),
        )
        .subcommand(
            App::new("embed")
                .about("embed some data into a binary steganographically")
                .add_common_arguments()
                .arg(
                    Arg::with_name("input")
                        .about("the binary to embed into")
                        .index(1)
                        .required(true),
                )
                .arg(
                    Arg::with_name("output")
                        .about("the path to write the steg'd binary to")
                        .index(2)
                        .required(false),
                ),
        )
        .subcommand(
            App::new("extract")
                .about("extract the hidden data from a binary")
                .add_common_arguments()
                .arg(
                    Arg::with_name("input")
                        .about("the binary to extract from")
                        .index(1)
                        .required(true),
                ),
        )
        .get_matches();

    match matches.subcommand() {
        ("profile", Some(matches)) => steg86::command::profile(&matches),
        ("embed", Some(matches)) => steg86::command::embed(&matches),
        ("extract", Some(matches)) => steg86::command::extract(&matches),
        _ => unreachable!(),
    }
}
