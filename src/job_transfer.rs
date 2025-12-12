use crate::job_description::JobDescription;
use pace26checker::digest::digest_output::InstanceDigest;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct TransferFromServer {
    pub best_scores: HashMap<InstanceDigest, u32>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TransferToServer {
    pub jobs: Vec<JobDescription>,
}
