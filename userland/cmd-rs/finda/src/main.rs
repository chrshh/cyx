use crate::{
    err::GrError,
    input::{RawArgs, get_args},
    search::search,
};

mod err;
mod input;
mod search;
mod writer;

fn main() {
    let raw_args = get_args();
    /* args[1] = FLAGS */
    /* args[2] = QUERY */
    /* args[3] = FILEPATH */

    if raw_args.len() < 2 {
        GrError::<&str>::PatternMissing.exit();
    }

    let args = RawArgs::parse_args(raw_args);

    println!("{}", args.flags);
    println!("{}", args.query);
    println!("{}", args.path);

    match search(args) {
        Ok(0) => (),
        _ => println!("ERRRR"),
    }
}
