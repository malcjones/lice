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

        format!(
            r#"Copyright (c) {year} {name}

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

#[test]
fn test_bsd_3_clause_license() {
    let license = Bsd3Clause.generate();
    assert!(license.contains("Redistribution and use in source and binary forms"));
    assert!(license.contains("Test Name"));
}
