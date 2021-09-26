use crate::common::Fields;
use crate::config::FrontmatterConfig;
use frontmatter::parse_and_find_content;
use std::collections::HashMap;

pub fn parse_frontmatter(handling: &FrontmatterConfig, buffer: &str) -> (Fields, Box<String>) {
    let default_output = (HashMap::new(), Box::new(buffer.to_string()));
    match handling {
        FrontmatterConfig::Ignore => default_output,
        FrontmatterConfig::Omit => {
            if let Ok((_toml, text)) = parse_and_find_content(&buffer) {
                (HashMap::new(), Box::new(text.trim().to_string()))
            } else {
                default_output
            }
        }
        FrontmatterConfig::Parse => {
            if let Ok((Some(value), text)) = parse_and_find_content(&buffer) {
                let fields: Fields = value
                    .as_table()
                    .unwrap()
                    .into_iter()
                    .map(|(k, v)| {
                        if v.is_str() {
                            (k.clone(), v.as_str().unwrap().to_owned())
                        } else {
                            (k.clone(), v.to_string())
                        }
                    })
                    .collect();

                return (fields, Box::new(text.trim().to_string()));
            }

            default_output
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn omit_option() {
        let expected: (Fields, String) = (HashMap::new(), "this is not".to_string());
        let output = parse_frontmatter(
            &FrontmatterConfig::Omit,
            &mut r#"+++
this = "is frontmatter"
+++

this is not
        "#
            .to_string(),
        );

        let computed = (output.0, output.1.to_string());
        assert_eq!(expected, computed)
    }

    #[test]
    fn parse_option() {
        let expected: (Fields, String) = (
            [
                ("this".to_string(), "is frontmatter".to_string()),
                ("a_number".to_string(), "22".to_string()),
            ]
            .iter()
            .cloned()
            .collect(),
            "this is not".to_string(),
        );
        let output = parse_frontmatter(
            &FrontmatterConfig::Parse,
            &mut r#"+++
this = "is frontmatter"
a_number = 22
+++

this is not
        "#
            .to_string(),
        );

        let computed = (output.0, output.1.to_string());
        assert_eq!(expected, computed)
    }
}
