use crate::job_description::JobDescription;
use pace26checker::io::digest::DigestString;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct TransferFromServer {
    pub best_scores: HashMap<DigestString, u32>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TransferToServer {
    pub jobs: Vec<JobDescription>,
}
