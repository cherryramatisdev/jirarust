use crate::{error::Error, git_api::get_current_jira_code, jira_api, utils};
use atty::Stream;
use regex::Regex;
use utils::colorful_print;

pub fn call(code: &Option<usize>) -> Result<String, Error> {
    let code = if code.is_none() {
        get_current_jira_code::call()?
    } else {
        code.unwrap()
    };
    let content = jira_api::get_card_content::call(&code)?;

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
        utils::color_dark_or_bright::ColorState::Darker => colorful_print::Color::White,
        utils::color_dark_or_bright::ColorState::Brighter => colorful_print::Color::Black,
    };

    let content = content
        .replace(format!("{{panel:bgColor={color}}}").as_str(), "")
        .replace("{panel}", "")
        .replace(
            box_colored.as_str(),
            colorful_print::get_colorful_text(box_colored.trim(), foreground_color).as_str(),
        );

    content
}
