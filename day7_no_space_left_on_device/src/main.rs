mod filesystem;
mod parser;

use parser::*;
use std::error::Error;

struct Shell {
    pub fs: filesystem::Tree,
}

impl Shell {
    fn new(fs_total_space: u64) -> Self {
        Self {
            fs: filesystem::Tree::new(fs_total_space),
        }
    }

    fn simulate(&mut self, execution: &CommandExecution) {
        match &execution.command {
            Command::LS() => {
                for output_line in &execution.output {
                    match output_line {
                        CommandOutputLine::File(file) => {
                            self.fs.add_file(&file.name, file.size);
                        }
                        CommandOutputLine::Dir(dir) => {
                            self.fs.add_dir(&dir.name);
                        }
                    }
                }
            }
            Command::CD(dir) => {
                self.fs.cd(&dir);
                // discard the output, should be empty
            }
        }
    }
}

pub fn main() -> Result<(), Box<dyn Error>> {
    let space_total = 70_000_000;
    let mut shell = Shell::new(space_total);

    let command_executions = load_input_command_executions("./input.prod").unwrap();

    for command_execution in command_executions.iter() {
        shell.simulate(&command_execution);
    }

    shell.fs.root.eval_size();
    //shell.fs.root.print();

    println!(
        "sum_size_dir_less_than_100K: {}",
        shell.fs.sum_size_of_dirs_less_than(100_000)
    );
    println!(
        "size_of_min_dir_to_delete: {}",
        shell.fs.get_size_of_dir_to_delete(30_000_000)
    );

    Ok(())
}
