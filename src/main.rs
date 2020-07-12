mod rime;

fn main() {
    let api = rime::new_api();
    if let Some(v)=api.get_version() {
        println!("Hello, world! {}", v);
    }
}
