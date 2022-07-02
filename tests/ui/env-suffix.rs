use camelpaste::paste;

paste! {
    fn [<env!("VAR"suffix)>]() {}
}

fn main() {}
