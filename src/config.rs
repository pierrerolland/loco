use crate::colors::Color;
use std::fs;
use yaml_rust::{Yaml, YamlLoader};
use yaml_rust::yaml::Hash;
use regex::{Regex, Captures};

pub struct DefinitionRegex {
    regex: Regex,
    color: &'static Color
}

pub struct Definition {
    partials: Vec<DefinitionRegex>,
    lines: Vec<DefinitionRegex>
}

impl Definition {
    pub fn from_config_file(config_file: Option<String>, definition: String) -> Definition {
        let config = Definition::retrieve_yaml_hash_from_config_file(config_file);
        let definition = config.get(&Yaml::from_str(&definition)).expect("Definition was not found in configuration file");
        let definition = match definition {
            Yaml::Hash(i) => i,
            _ => panic!("Definition must be a YAML object")
        };

        Definition::build_definition_from_config(definition)
    }

    pub fn apply(&self, s: &str, should_color_all_line: bool) -> String {
        let mut new_string = String::from(s);

        for line in &self.lines {
            new_string = line.colorize(&new_string, should_color_all_line);
        }
        if !should_color_all_line {
            for partial in &self.partials {
                new_string = partial.colorize(&new_string, false);
            }
        }

        new_string
    }

    fn retrieve_yaml_hash_from_config_file(config_file: Option<String>) -> Hash {
        let config_file = match config_file {
            None => DEFAULT_CONFIG_FILE.to_string(),
            Some(i) => i
        };
        let config = &YamlLoader::load_from_str(&fs::read_to_string(config_file).expect("Could not read configuration file"))
            .expect("Could not parse configuration file")
            [0];

        return match config {
            Yaml::Hash(i) => i.clone(),
            _ => panic!("Configuration YAML is not valid")
        };
    }

    fn build_definition_from_config(config: &Hash) -> Definition {
        return Definition {
            lines: Definition::yaml_to_regex_vector(config, "lines"),
            partials: Definition::yaml_to_regex_vector(config, "partials")
        };
    }

    fn yaml_to_regex_vector(config: &Hash, key: &str) -> Vec<DefinitionRegex> {
        let items_collection = match config.get(&Yaml::from_str(key)) {
            None => {
                return vec!()
            },
            Some(yaml) => {
                match yaml {
                    Yaml::Array(i) => i,
                    _ => panic!("\"lines\" and \"partials\" YAMLs must be arrays")
                }
            }
        };

        items_collection.iter().map(|item| {
            let item = match item {
                Yaml::Hash(i) => i,
                _ => panic!("Elements under \"lines\" and \"partials\" must be objects with \"regex\" and \"color\" keys")
            };

            DefinitionRegex {
                regex: Definition::extract_regex_from_definition_item(item),
                color: Definition::extract_color_from_definition_item(item)
            }
        }).collect()
    }

    fn extract_regex_from_definition_item(item: &Hash) -> Regex {
        let regex = item.get(&Yaml::from_str("regex")).expect("Missing \"regex\" key");

        return Regex::new(regex.as_str().unwrap()).unwrap();
    }

    fn extract_color_from_definition_item(item: &Hash) -> &'static Color {
        let color_name = item.get(&Yaml::from_str("color")).expect("Missing \"color\" key");

        return Color::from_human_name(color_name.as_str().unwrap());
    }
}

impl DefinitionRegex {
    fn colorize(&self, s: &str, whole_line: bool) -> String {
        if !self.regex.is_match(s) {
            return s.to_string();
        }

        if whole_line {
            return self.color.colorize_str(&s.to_string());
        }

        return self.regex.replace_all(s, |caps: &Captures| {
            let captured_string = caps[0].to_string();
            self.color.colorize_str(&captured_string)
        }).to_string();
    }
}

const DEFAULT_CONFIG_FILE: &str = "/etc/loco/loco.yml";
