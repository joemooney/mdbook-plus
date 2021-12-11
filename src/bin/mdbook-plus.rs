extern crate clap;
extern crate mdbook;
extern crate mdbook_plus;
extern crate serde_json;

use clap::{crate_version, App, Arg, ArgMatches, SubCommand};
use mdbook::errors::Error;
use mdbook::preprocess::{CmdPreprocessor, Preprocessor};
use mdbook_plus::MDBookPlus;

use std::io;
use std::process;

pub fn make_app() -> App<'static, 'static> {
    App::new("mdbook-plus")
        .version(crate_version!())
        .about("mdbook preprocessor to add Table of Contents")
        .subcommand(
            SubCommand::with_name("supports")
                .arg(Arg::with_name("renderer").required(true))
                .about("Check whether a renderer is supported by this preprocessor"),
        )
}

fn main() {
    let matches = make_app().get_matches();

    if let Some(sub_args) = matches.subcommand_matches("supports") {
        handle_supports(sub_args);
    } else {
        if let Err(e) = handle_preprocessing() {
            eprintln!("{}", e);
            process::exit(1);
        }
    }
}

fn handle_preprocessing() -> Result<(), Error> {
    let (ctx, book) = CmdPreprocessor::parse_input(io::stdin())?;

    if ctx.mdbook_version != mdbook::MDBOOK_VERSION {
        eprintln!(
            "Warning: The mdbook-plus preprocessor was built against version \
             {} of mdbook, but we're being called from version {}",
            mdbook::MDBOOK_VERSION,
            ctx.mdbook_version
        );
    }

    eprintln!("Warning: The mdbook-plus preprocessor not running...");
    let processed_book = MDBookPlus.run(&ctx, book)?;
    serde_json::to_writer(io::stdout(), &processed_book)?;

    Ok(())
}

fn handle_supports(sub_args: &ArgMatches) -> ! {
    let renderer = sub_args.value_of("renderer").expect("Required argument");
    let supported = MDBookPlus.supports_renderer(&renderer);

    // Signal whether the renderer is supported by exiting with 1 or 0.
    if supported {
        process::exit(0);
    } else {
        process::exit(1);
    }
}
