use crate::IgnoreTemplate;
use dialoguer::{theme::ColorfulTheme, MultiSelect};

pub fn render_interactive_selector(templates: &Vec<IgnoreTemplate>) -> Option<String> {
    let render_strings: Vec<&str> = templates.into_iter().map(|x| x.text).collect();
    let defaults = &vec![false; render_strings.len()];
    let selections = MultiSelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Pick your stack")
        .items(&render_strings[..])
        .defaults(&defaults[..])
        .paged(true)
        .interact()
        .unwrap();

    if selections.is_empty() {
        eprintln!("You didn't select anything");
        None
    } else {
        Some(
            selections
                .into_iter()
                .map(|index| String::from(templates[index].id))
                .collect::<Vec<_>>()
                .join(",")
        )
    }
}
