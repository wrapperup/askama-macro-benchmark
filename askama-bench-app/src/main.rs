mod templates;

fn main() {
    templates::render_templates();
}

#[cfg(test)]
mod tests {
    use std::{path::PathBuf, env};

    use askama_bench_lib::generator::gen_test;

    #[test]
    fn test_generator() {
        let gen = gen_test();
        
        let root = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
        let path = root.join("src/templates.rs");

        std::fs::write(path, gen).unwrap();
    }
}
