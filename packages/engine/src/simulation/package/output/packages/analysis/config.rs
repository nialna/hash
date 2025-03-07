use std::{collections::HashMap, sync::Arc};

use serde::{Deserialize, Serialize};

use super::{analyzer::AnalysisOperationRepr, Result};
use crate::{
    proto::ExperimentRunTrait,
    simulation::package::output::packages::analysis::{
        analyzer::AnalysisSourceRepr, get_analysis_source,
    },
    ExperimentConfig,
};

#[derive(Serialize, Deserialize)]
pub struct AnalysisOutputConfig {
    pub outputs: HashMap<Arc<String>, Vec<AnalysisOperationRepr>>,
    pub manifest: String,
}

impl AnalysisOutputConfig {
    pub fn new(config: &ExperimentConfig) -> Result<AnalysisOutputConfig> {
        let manifest = get_analysis_source(&config.run.base().project_base.packages)?;
        let analysis_src_repr = AnalysisSourceRepr::try_from(&manifest as &str)?;
        Ok(AnalysisOutputConfig {
            outputs: analysis_src_repr.outputs,
            manifest,
        })
    }
}
