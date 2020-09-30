#![warn(anonymous_parameters)]
#![warn(box_pointers)]
//#![warn(missing_docs)]
#![warn(trivial_casts, trivial_numeric_casts)]
#![warn(unused_results)]
#![warn(unused_qualifications)]
#![warn(variant_size_differences)]
#![warn(clippy::cast_possible_truncation,clippy::cast_possible_wrap,
clippy::cast_precision_loss,clippy::cast_sign_loss,clippy::integer_arithmetic)]
#![warn(clippy::fallible_impl_from)]
#![warn(clippy::filter_map,clippy::filter_map_next)]
#![warn(clippy::if_not_else,clippy::nonminimal_bool,clippy::single_match_else)]
#![warn(clippy::int_plus_one)]
#![warn(clippy::similar_names)]
#![warn(clippy::mutex_integer)]
//#![warn(clippy::print_stdout,clippy::use_debug)]
#![warn(clippy::unwrap_used,clippy::map_unwrap_or)]
//#![warn(clippy::unwrap_in_result)]

use std::future::Future;

fn main() {
    println!("Hello, world!");
}

async fn foo() -> u8 { 5 }

fn bar() -> Future {
    async {
        let x: u8 = foo().await;
        x + 5
    }
}
