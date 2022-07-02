#![deny(warnings)]

use camelpaste::paste;

macro_rules! m {
    ($i:ident) => {
        paste! {
            pub fn [<foo $i>]() {}
        }
    };
}

m!(Bar);

fn main() {}
