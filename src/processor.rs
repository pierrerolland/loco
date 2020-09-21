use std::{io, thread};
use std::io::{BufRead, Seek, SeekFrom, Lines, BufReader};
use std::fs::File;
use std::time::Duration;
use crate::args::Args;
use crate::config::Definition;

pub struct FileProcessingOptions {
    should_watch: bool,
    should_color_full_lines: bool
}

pub struct FileProcessor {
    options: FileProcessingOptions,
    definition: Definition
}

impl FileProcessor {
    pub fn from_args_and_definition(args: &Args, definition: Definition) -> FileProcessor {
        return FileProcessor {
            options: FileProcessingOptions {
                should_watch: args.has_option('w'),
                should_color_full_lines: args.has_option('l')
            },
            definition
        };
    }

    pub fn process_file(&self, filename: Option<String>) {
        match filename {
            None => self.read_from_stdin(),
            Some(i) => self.read_from_file(i)
        }
    }

    fn read_from_stdin(&self) {
        loop {
            let mut buffer = String::new();
            io::stdin().read_line(&mut buffer).expect("Failed to read standard input");
            print!("{}", self.definition.apply(&buffer, self.options.should_color_full_lines));
        }
    }

    fn read_from_file(&self, filename: String) {
        match self.options.should_watch {
            false => self.read_file(filename),
            true => self.watch_file(filename)
        };
    }

    fn watch_file(&self, filename: String) {
        let duration = Duration::from_millis(50);
        let mut file = File::open(filename).expect("Could not read file");
        file.seek(SeekFrom::End(0)).expect("Error while reading file");

        loop {
            self.read_lines(io::BufReader::new(&file).lines());
            thread::sleep(duration);
        }
    }

    fn read_file(&self, filename: String) {
        let file = File::open(filename).expect("Could not read file");
        let lines = io::BufReader::new(&file).lines();

        self.read_lines(lines);
    }

    fn read_lines(&self, lines: Lines<BufReader<&File>>) {
        for line in lines {
            match line {
                Ok(content) => {
                    println!("{}", self.definition.apply(&content, self.options.should_color_full_lines));
                },
                _ => ()
            }
        }
    }
}
