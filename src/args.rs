const OPTIONS_NEEDING_ARGUMENT: [char; 2] = ['d', 'c'];

pub struct ArgOption {
    option: char,
    value: Option<String>,
}

pub struct Args {
    pub options: Vec<ArgOption>,
    pub argument: Option<String>
}

impl Args {
    pub fn from_app_args(args: Vec<String>) -> Args {
        let mut converted = Args {
            options: Vec::new(),
            argument: None
        };
        let mut i = 0;
        let mut skip_next = false;

        for item in &args {
            if skip_next || i == 0 {
                skip_next = false;
                i = i + 1;
                continue;
            }

            if is_option_definer(item) {
                let option = extract_option(item);
                let value = match is_option_needing_argument(&option) {
                    true => {
                        skip_next = true;
                        match args.get(i + 1) {
                            None => None,
                            Some(i) => Some(i.to_string())
                        }
                    },
                    false => None
                };

                converted.options.push(ArgOption {
                    option,
                    value
                })
            } else {
                converted.argument = Some(item.to_string());
            }

            i = i + 1;
        }

        converted
    }

    pub fn has_option(&self, option: char) -> bool {
        for opt in &self.options {
            if opt.option == option {
                return true;
            }
        }

        false
    }

    pub fn get_option_value(&self, option: char) -> Option<String> {
        for opt in &self.options {
            if opt.option == option {
                return match &opt.value {
                    None => None,
                    Some(i) => Some(i.clone())
                };
            }
        }

        None
    }
}

fn is_option_definer(item: &String) -> bool {
    item.chars().nth(0).unwrap() == '-' && item.len() > 1
}

fn extract_option(item: &String) -> char {
    item.chars().nth(1).unwrap()
}

fn is_option_needing_argument(opt: &char) -> bool {
    OPTIONS_NEEDING_ARGUMENT.contains(opt)
}
