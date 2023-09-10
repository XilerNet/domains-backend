use utils::auto_complete::get_suggestions;

mod utils;

fn main() {
    let suggestions = get_suggestions("hel", 5);
    println!("{:?}", suggestions);
}
