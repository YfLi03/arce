/*
    initting tera
*/
use tera::Tera;

pub fn parse() -> Tera {
    let tera = match Tera::new("template/**/*.html") {
        Ok(t) => t,
        Err(e) => {
        println!("Parsing error(s): {}", e);
        ::std::process::exit(1);
        }
    };
    let names: Vec<_> = tera.get_template_names().collect();
    println!("Parsed {} Templates: {:?}", names.len(), names);

    tera
}