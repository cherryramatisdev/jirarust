use dialoguer::{console::Term, theme::ColorfulTheme, Select};

pub fn call(items: Vec<&str>) -> Result<String, Box<dyn std::error::Error>> {
    let selection = Select::with_theme(&ColorfulTheme::default())
        .items(&items)
        .default(0)
        .interact_on_opt(&Term::stderr())?;

    match selection {
       Some(index) => Ok(items[index].to_string()), 
       None => Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "No item selected")))?
    }
}
