use askama::Template;

#[derive(Template)]
#[template(path = "gen-templates.txt")]
struct GenerateTemplates;

#[allow(dead_code)]
pub fn gen_test() -> String {
    GenerateTemplates {}.render().unwrap()
}

#[cfg(test)]
mod tests {
    use std::{path::PathBuf, env};

    use crate::generator::gen_test;

    #[test]
    fn test_generator() {
        let gen = gen_test();
        
        let root = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
        let path = root.join("src/templates.rs");

        std::fs::write(path, gen).unwrap();
    }
}
