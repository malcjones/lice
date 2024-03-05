use dialoguer::Select;

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

pub mod mit {
    use chrono::Datelike;
    use dialoguer::Input;

    use super::Generator;

    pub struct MitLicense;

    impl Generator for MitLicense {
        fn name(&self) -> &str {
            "MIT License"
        }

        fn generate(&self) -> String {
            // use test values if we're running in test
            let name = if cfg!(test) {
                "Test Name".to_string()
            } else {
                Input::new()
                    .with_prompt("What is your name?")
                    .interact()
                    .unwrap()
            };

            let year = chrono::Utc::now().year();

            format!(
                r#"Copyright {year} {name}

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in
all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS
IN THE SOFTWARE."#
            )
        }
    }
}

pub mod bsd_3_clause {
    use chrono::Datelike;
    use dialoguer::Input;

    use super::Generator;

    pub struct Bsd3Clause;

    impl Generator for Bsd3Clause {
        fn name(&self) -> &str {
            "3-Clause BSD License"
        }

        fn generate(&self) -> String {
            // use test values if we're running in test
            let name = if cfg!(test) {
                "Test Name".to_string()
            } else {
                Input::new()
                    .with_prompt("What is your name?")
                    .interact()
                    .unwrap()
            };

            let year = chrono::Utc::now().year();

            format!(r#"Copyright (c) {year} {name}

Redistribution and use in source and binary forms, with or without
modification, are permitted provided that the following conditions are met:

1. Redistributions of source code must retain the above copyright notice,
   this list of conditions and the following disclaimer.

2. Redistributions in binary form must reproduce the above copyright notice,
    this list of conditions and the following disclaimer in the documentation
    and/or other materials provided with the distribution.

3. Neither the name of the copyright holder nor the names of its
    contributors may be used to endorse or promote products derived from
    this software without specific prior written permission.

THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE
FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER
CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY,
OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE."#
            )
        }
    }
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
