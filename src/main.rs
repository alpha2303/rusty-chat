use components::core::app::AppResult;

pub mod components;

fn main() -> AppResult<()> {
    components::core::base::start_app()
}

/*
To do:
1. UI Layout
2. Error Logging
3. Backend server
etc
 */