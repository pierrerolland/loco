pub struct Color {
    human_name: &'static str,
    code: &'static str
}

impl Color {
    pub fn from_human_name(human_name: &str) -> &'static Color {
        for item in &COLORS {
            if item.human_name.eq(human_name) {
                return item;
            }
        }

        panic!("Color {} is not available :/\nAvailable colors are: red, green, yellow, blue, magenta, cyan, light gray and dark gray", human_name);
    }

    pub fn colorize_str(&self, str: &String) -> String {
        let mut colorized = String::from(self.code);

        colorized.push_str(str);
        colorized.push_str(RESET.code);

        colorized
    }
}

const RESET: Color = Color { human_name: "reset", code: "\u{001b}[0m" };
const COLORS: [Color; 9] = [
    Color { human_name: "normal", code: "\x1B[0m" },
    Color { human_name: "red", code: "\x1B[31m" },
    Color { human_name: "green", code: "\x1B[32m" },
    Color { human_name: "yellow", code: "\x1B[33m" },
    Color { human_name: "blue", code: "\x1B[34m" },
    Color { human_name: "magenta", code: "\x1B[35m" },
    Color { human_name: "cyan", code: "\x1B[36m" },
    Color { human_name: "light gray", code: "\x1B[37m" },
    Color { human_name: "dark gray", code: "\x1B[90m" }
];
