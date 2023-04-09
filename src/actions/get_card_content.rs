use crate::{error::Error, jira_api, utils};
use ansi_hex_color;
use atty::Stream;
use regex::Regex;

const FOREGROUND_LIGHT: &str = "#ffffff";
const FOREGROUND_DARK: &str = "#000000";

pub fn call(code: &usize) -> Result<String, Error> {
    let content = jira_api::get_card_content::call(code)?;

    if atty::is(Stream::Stdin) && content.contains("panel:bgColor") {
        let content = colorize_content(content);

        return Ok(content);
    }

    Ok(content)
}

fn capture_group_as_str(regex: &str, group_location: usize, content: &str) -> String {
    let re = Regex::new(regex).unwrap();
    let groups = re.captures(content).unwrap();
    return groups.get(group_location).unwrap().as_str().to_string();
}

fn colorize_content(content: String) -> String {
    let color = capture_group_as_str(r"\#\w*", 0, &content);
    let box_colored =
        capture_group_as_str(r"\{panel:bgColor=.*?\}([\s\S]*?)\{panel\}", 1, &content);

    let foreground_color = match utils::color_dark_or_bright::call(&color) {
        utils::color_dark_or_bright::ColorState::Darker => FOREGROUND_LIGHT,
        utils::color_dark_or_bright::ColorState::Brighter => FOREGROUND_DARK,
    };

    let content = content
        .replace(format!("{{panel:bgColor={color}}}").as_str(), "")
        .replace("{panel}", "")
        .replace(
            box_colored.as_str(),
            ansi_hex_color::colored(foreground_color, color.as_str(), box_colored.trim()).as_str(),
        );

    content
}
