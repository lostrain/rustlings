// modules1.rs
// Make me compile! Execute `rustlings hint modules1` for hints :)

mod sausage_factory {
    // Don't let anybody outside of this module see this!
    fn get_secret_recipe() -> String {
        String::from("Ginger")
    }

    pub fn make_sausage() {
        let mut s = get_secret_recipe();
        s = s.to_ascii_uppercase();
        println!("{}", s);
        println!("sausage!");
    }
}

fn main() {
    sausage_factory::make_sausage();
}
