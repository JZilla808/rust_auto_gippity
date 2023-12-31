use crate::models::agent_basic::basic_agent::{AgentState, BasicAgent};
use crate::models::agents::agent_traits::{FactSheet, SpecialFunctions};

use crate::ai_functions::aifunc_managing::convert_user_input_to_goal;
use crate::helpers::general::ai_task_request;
use crate::models::agents::agent_architect::AgentSolutionArchitect;
use crate::models::general::llm::Message;

#[derive(Debug)]
pub struct ManagingAgent {
    attributes: BasicAgent,
    factsheet: FactSheet,
    agents: Vec<Box<dyn SpecialFunctions>>,
}

impl ManagingAgent {
    pub async fn new(user_req: String) //-> Result<Self, Box<dyn std::error::Error>>
    {
        let attributes: BasicAgent = BasicAgent {
            objective: "Manage agents who are building an excellent website for the user"
                .to_string(),
            position: "Project Manager".to_string(),
            state: AgentState::Discovery,
            memory: vec![],
        };
    }
}
