use crate::models::agent_basic::basic_traits::BasicTraits;
use crate::models::general::llm::Message;

#[derive(Debug, Clone, PartialEq)]
pub enum AgentState {
    Discovery,
    Working,
    UnitTesting,
    Finished,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BasicAgent {
    pub objective: String,
    pub position: String,
    pub state: AgentState,
    pub memory: Vec<Message>,
}

impl BasicTraits for BasicAgent {
    fn new(objective: String, position: String) -> Self {
        BasicAgent {
            objective,
            position,
            state: AgentState::Discovery,
            memory: Vec::new(),
        }
    }

    fn update_state(&mut self, new_state: AgentState) {
        self.state = new_state;
    }

    fn get_objective(&self) -> &String {
        &self.objective
    }

    fn get_position(&self) -> &String {
        &self.position
    }

    fn get_state(&self) -> &AgentState {
        &self.state
    }

    fn get_memory(&self) -> &Vec<Message> {
        &self.memory
    }
}
