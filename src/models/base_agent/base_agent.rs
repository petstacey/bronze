use crate::models::base_agent::base_traits::BaseTraits;
use crate::models::general::llm::Message;

#[derive(Debug, PartialEq)]
pub enum AgentState {
    Discovery,
    Working,
    UnitTesting,
    Finished,
}

#[derive(Debug)]
pub struct BaseAgent {
    pub objective: String,
    pub position: String,
    pub state: AgentState,
    pub memory: Vec<Message>,
}

impl BaseTraits for BaseAgent {
    fn new(objective: String, position: String) -> Self {
        Self {
            objective,
            position,
            state: AgentState::Discovery,
            memory: Vec::from([]),
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
