use std::sync::Arc;

use super::Result;
use crate::{
    config::globals::Globals,
    datastore::schema::{
        context::ContextSchema,
        state::{AgentSchema, MessageSchema},
    },
    simulation::package::creator::PackageCreators,
};

pub struct Config {
    pub agent_schema: Arc<AgentSchema>,
    pub message_schema: Arc<MessageSchema>,
    pub context_schema: Arc<ContextSchema>,
}

impl Config {
    pub fn new_sim(
        exp_config: &super::ExperimentConfig,
        globals: &Globals,
        package_creators: &PackageCreators,
    ) -> Result<Config> {
        let agent_schema = Arc::new(package_creators.get_agent_schema(exp_config, globals)?);
        let message_schema = Arc::new(MessageSchema::new());
        let context_schema = Arc::new(package_creators.get_context_schema(exp_config, globals)?);

        Ok(Config {
            agent_schema,
            message_schema,
            context_schema,
        })
    }
}
