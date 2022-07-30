#[derive(Copy, Clone, Debug)]
enum MenuItem {
    Home,
    Contacts
}

impl From<MenuItem> for usize {
    fn from(input: MenuItem) -> usize {
        match input {
            MenuItem::Home => 0,
            MenuItem::Contacts => 1,
        }
    }
}