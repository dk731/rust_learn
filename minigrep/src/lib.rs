#[derive(Debug)]
pub struct InputArgs {
    file_path: String,
    search_pattern: String,
}

impl InputArgs {
    pub fn new(args: &Vec<String>) -> InputArgs {
        if args.len() != 3 {
            panic!("Please provide input parameters");
        }

        InputArgs {
            file_path: args[1].clone(),
            search_pattern: args[2].clone(),
        }
    }

    pub fn file_path(&self) -> &String {
        &self.file_path
    }

    pub fn search_pattern(&self) -> &String {
        &self.search_pattern
    }
}
