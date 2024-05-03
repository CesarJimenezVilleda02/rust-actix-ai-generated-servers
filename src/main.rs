#[macro_export]
macro_rules! get_function_string {
    ($func: ident) => {{
        stringify!($func)
    }};
}

#[macro_use]
mod ai_functions;
mod apis;
mod helpers;
mod models;

use helpers::command_line::get_user_response;
use models::agents_manager::managing_agent::{self, ManagingAgent};

#[tokio::main]
async fn main() {
    let user_req = get_user_response("What web server are we building today?");

    let mut managing_agent = ManagingAgent::new(user_req)
        .await
        .expect("Error creating Managing Agent");

    managing_agent.execute_project().await;

    dbg!(managing_agent);
}
