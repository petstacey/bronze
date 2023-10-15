#[macro_export]
macro_rules! get_function_string {
    ($function: ident) => {
        stringify!($function)
    };
}

#[macro_use]
mod ai_functions;
mod apis;
mod helpers;
mod models;

use helpers::cli::user_input;
use models::project_manager::project_manager::ProjectManager;

#[tokio::main]
async fn main() {
    let usr_req = user_input("What are we building today?");

    let mut project_maanger = ProjectManager::new(usr_req)
        .await
        .expect("Error creating team");

    project_maanger.deliver_project().await;

    // dbg!(project_maanger);
}
