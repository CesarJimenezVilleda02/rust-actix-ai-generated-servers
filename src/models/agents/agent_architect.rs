use crate::ai_functions::aifunc_architect::{print_project_scope, print_site_urls};
use crate::helpers::command_line::PrintCommand;
use crate::helpers::general::{ai_task_request_decoded, check_status_code};
use crate::models::agent_basic::basic_agent::{AgentState, BasicAgent};
use crate::models::agent_basic::basic_traits::BasicTraits;
use crate::models::agents::agent_traits::{FactSheet, ProjectScope, SpecialFunctions};

use async_trait::async_trait;
use reqwest::Client;
use std::time::Duration;
use std::vec;

// Solution architect
#[derive(Debug)]
pub struct AgentSolutionArchitect {
    attributes: BasicAgent,
}

impl AgentSolutionArchitect {
    pub fn new() -> Self {
        let attributes = BasicAgent {
            objective: "Gathers information and design solutions for website development"
                .to_string(),
            position: "Solution Architect".to_string(),
            state: AgentState::Discovery,
            memory: vec![],
        };

        Self { attributes }
    }

    // Retrieve scope
    async fn call_project_scope(&mut self, fact_sheet: &mut FactSheet) -> ProjectScope {
        let msg_context: String = format!("{}", fact_sheet.project_description);

        let ai_response: ProjectScope = ai_task_request_decoded::<ProjectScope>(
            msg_context,
            &self.attributes.position,
            get_function_string!(print_project_scope),
            print_project_scope,
        )
        .await;

        fact_sheet.project_scope = Some(ai_response.clone());
        self.attributes.update_state(AgentState::Finished);
        return ai_response;
    }

    // Retrieve urls
    async fn call_determine_external_urls(
        &mut self,
        fact_sheet: &mut FactSheet,
        msg_context: String,
    ) {
        let ai_response = ai_task_request_decoded::<Vec<String>>(
            msg_context,
            &self.attributes.position,
            get_function_string!(print_site_urls),
            print_site_urls,
        )
        .await;

        fact_sheet.external_urls = Some(ai_response);
        self.attributes.state = AgentState::UnitTesting;
    }
}

#[async_trait]
impl SpecialFunctions for AgentSolutionArchitect {
    fn get_attributes_from_agent(&self) -> &BasicAgent {
        &self.attributes
    }

    async fn execute(
        &mut self,
        fact_sheet: &mut FactSheet,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // This can leed to infinite loops
        while self.attributes.state != AgentState::Finished {
            match self.attributes.state {
                AgentState::Discovery => {
                    let project_scope = self.call_project_scope(fact_sheet).await;

                    // Confirm if external urls
                    if project_scope.is_external_urls_required {
                        self.call_determine_external_urls(
                            fact_sheet,
                            fact_sheet.project_description.clone(),
                        )
                        .await;
                        self.attributes.state = AgentState::UnitTesting;
                    }
                }
                AgentState::UnitTesting => {
                    let mut exclude_urls: Vec<String> = vec![];

                    let client: Client = Client::builder()
                        .timeout(Duration::from_secs(5))
                        .build()
                        .unwrap();

                    // Find faulty urls
                    let urls: &Vec<String> = fact_sheet
                        .external_urls
                        .as_ref()
                        .expect("No url object on factsheet");

                    for url in urls {
                        let endpoint = format!("Testing url endpoint: {}", url);
                        PrintCommand::UnitTest.print_agent_message(
                            &self.attributes.position.as_str(),
                            endpoint.as_str(),
                        );

                        // Url tets
                        match check_status_code(&client, url).await {
                            Ok(status_code) => {
                                if status_code != 200 {
                                    exclude_urls.push(url.clone());
                                }
                            }
                            Err(e) => {
                                println!("Error checking url: {} {}", url, e);
                            }
                        }
                    }

                    // Exclude faulty urls
                    if exclude_urls.len() > 0 {
                        fact_sheet.external_urls = Some(
                            urls.iter()
                                .filter(|url| !exclude_urls.contains(url))
                                .map(|url| url.clone())
                                .collect(),
                        );
                    }

                    // Finish
                    self.attributes.state = AgentState::Finished;
                }
                // Default finish state
                _ => {
                    self.attributes.state = AgentState::Finished;
                }
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_solution_architect() {
        let mut agent: AgentSolutionArchitect = AgentSolutionArchitect::new();

        let mut fact_sheet = FactSheet {
            project_description: "Build a full stack website with user login and logout that shows latest Forex prices".to_string(),
            project_scope: None,
            external_urls: None,
            backend_code: None, // Add the missing field
            api_endpoint_schema: None, // Add the missing field
        };

        agent
            .execute(&mut fact_sheet)
            .await
            .expect("Unable to execute solution architect agent");

        assert!(fact_sheet.project_scope != None);
        assert!(fact_sheet.external_urls.is_some());

        dbg!(&agent);
        dbg!(&fact_sheet);
    }
}
