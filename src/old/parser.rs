///Initializing tera
use tera::Tera;
use std::error::Error;

pub fn parse() -> Result<Tera, Box<dyn Error>> {
    let tera = Tera::new("template/**/*.html")?;
    let names: Vec<_> = tera.get_template_names().collect();
    println!("Parsed {} Templates: {:?}", names.len(), names);
    Ok(tera)
}