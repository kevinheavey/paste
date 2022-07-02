use camelpaste::paste;

paste! {
    fn [<a env!("PASTE_UNKNOWN") b>]() {}
}

fn main() {}
