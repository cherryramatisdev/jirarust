use crate::jira_api;
use ansi_hex_color;
use regex::Regex;

pub fn call(code: &usize) -> Result<String, Box<dyn std::error::Error>> {
    let content = jira_api::get_card_content::call(code)?;

    if content.contains("panel:bgColor") {
        // TODO: refactor this
        let re = Regex::new(r"\#\w*").unwrap();
        let groups = re.captures(content.as_str()).unwrap();
        let color = groups.get(0).unwrap().as_str();

        let re = Regex::new(r"\{panel:bgColor=.*?\}([\s\S]*?)\{panel\}").unwrap();
        let groups = re.captures(content.as_str()).unwrap();
        let box_colored = groups.get(1).unwrap().as_str();

        let content = content
            .replace(format!("{{panel:bgColor={color}}}").as_str(), "")
            .replace("{panel}", "")
            .replace(box_colored, "");

        return Ok(format!(
            "{}{content}",
            // TODO: check if `color` is too bright or too dark and decide foreground color.
            ansi_hex_color::colored("", color, &box_colored)
        ));
    }

    return Ok(content);
}
