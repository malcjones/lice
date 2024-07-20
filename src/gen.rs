use dialoguer::Select;

mod bsd_3_clause;
mod mit;

pub fn all() -> Vec<Box<dyn Generator>> {
    vec![
        Box::new(mit::MitLicense),
        Box::new(bsd_3_clause::Bsd3Clause),
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mit_license() {
        let mit_license = mit::MitLicense;
        let license = mit_license.generate();
        assert!(license.contains("Permission is hereby granted"));
        assert!(license.contains("Test Name"));
    }

    #[test]
    fn test_bsd_3_clause_license() {
        let bsd_3_clause_license = bsd_3_clause::Bsd3Clause;
        let license = bsd_3_clause_license.generate();
        assert!(license.contains("Redistribution and use in source and binary forms"));
        assert!(license.contains("Test Name"));
    }
}
