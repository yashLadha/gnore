use crate::IgnoreTemplate;

pub fn prompt_begin() {
    println!("Begin prompt");
}

pub fn render_interactive_selector(templates: &Vec<IgnoreTemplate>) {
    let render_strings = templates.into_iter().map(|x| x.text).collect::<String>();
    println!("render interactive selector");
}
