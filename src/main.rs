pub mod gen;

fn main() {
    let generators: Vec<Box<dyn gen::Generator>> = vec![Box::new(gen::mit::MitLicense), Box::new(gen::bsd_3_clause::Bsd3Clause)];
    let license = gen::prompt_and_generate(generators);
    std::fs::write("LICENSE", license).unwrap();
}
