pub mod gen;

fn main() {
    let license = gen::prompt_and_generate(gen::all());
    std::fs::write("LICENSE", license).unwrap();
}
