use ywt::use_effect;

fn main() {
    let s = String::new();
    let i = 0;

    use_effect!(s; move || (); i);
}
