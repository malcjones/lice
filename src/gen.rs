use dialoguer::Select;

mod bsd_2_clause;
mod bsd_3_clause;
mod mit;
mod unlicense;

pub fn all() -> Vec<Box<dyn Generator>> {
    vec![
        Box::new(mit::MitLicense),
        Box::new(bsd_3_clause::Bsd3Clause),
        Box::new(bsd_2_clause::Bsd2Clause),
        Box::new(unlicense::Unlicense),
    ]
}

pub trait Generator {
    fn name(&self) -> &str;
    fn generate(&self) -> String;
}

pub fn prompt_and_generate(generators: Vec<Box<dyn Generator>>) -> String {
    let generator_names: Vec<&str> = generators.iter().map(|g| g.name()).collect();
    let selected_generator = Select::new()
        .with_prompt("Select a generator")
        .items(&generator_names)
        .default(0)
        .interact()
        .unwrap();

    generators[selected_generator].generate()
}
