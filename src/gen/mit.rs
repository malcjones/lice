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

#[test]
fn test_mit_license() {
    let license = MitLicense.generate();
    assert!(license.contains("Permission is hereby granted"));
    assert!(license.contains("Test Name"));
}