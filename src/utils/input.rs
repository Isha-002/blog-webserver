use dialoguer::Input;

pub fn db_input() -> String {
    Input::new()
    .with_prompt("We couldn't connect to the database\nYou can change the config file and restart the App or ENTER the URL here:")
    .interact_text()
    .unwrap()
}
