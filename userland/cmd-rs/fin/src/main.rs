mod err;
mod input;

use crate::{
    err::FError,
    input::{InputArgs, get_args},
};

fn main() {
    let args = get_args();
    /* args[1] = FLAGS */
    /* args[2] = QUERY */
    /* args[3] = FILEPATH */

    if args.len() < 2 {
        FError::<&str>::PatternMissing.exit();
    }

    let argv = InputArgs::parse_args(args);

    println!("{}", argv.flags);
    println!("{}", argv.query);
    println!("{}", argv.path);
}
