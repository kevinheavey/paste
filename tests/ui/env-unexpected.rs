use camelpaste::paste;

paste! {
    fn [<env!("VAR" "VAR")>]() {}
}

fn main() {}
