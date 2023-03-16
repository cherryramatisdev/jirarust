use crate::{jira_api, utils};
use ansi_hex_color;
use regex::Regex;

const FOREGROUND_LIGHT: &str = "#ffffff";
const FOREGROUND_DARK: &str = "#000000";

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

        let foreground_color = match utils::color_dark_or_bright::call(color) {
            utils::color_dark_or_bright::ColorState::Darker => FOREGROUND_LIGHT,
            utils::color_dark_or_bright::ColorState::Brighter => FOREGROUND_DARK,
        };

        return Ok(format!(
            "{}{content}",
            ansi_hex_color::colored(foreground_color, color, &box_colored)
        ));
    }

    return Ok(content);
}
