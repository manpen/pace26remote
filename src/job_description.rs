use pace26checker::checks::bin_tree_with_parent::NodeCursor;
use pace26checker::digest::digest_output::InstanceDigest;
use pace26io::newick::NewickWriter;
use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum JobResult {
    Valid { score: u32, solution: String },
    Infeasible,
    Timeout,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct JobDescription {
    pub idigest: InstanceDigest,
    pub result: JobResult,
    pub runtime: Option<Duration>,
}

impl JobDescription {
    pub fn valid(
        idigest: InstanceDigest,
        trees: Vec<NodeCursor>,
        runtime: Option<Duration>,
    ) -> JobDescription {
        let score = trees.len() as u32;

        // compute newick strings and sort by size
        let mut newick: Vec<_> = trees.into_iter().map(|t| t.to_newick_string()).collect();
        newick.sort_unstable_by_key(|t| t.len());
        let solution = newick.join("\n");

        JobDescription {
            idigest,
            runtime,
            result: JobResult::Valid { score, solution },
        }
    }

    pub fn timeout(idigest: InstanceDigest, runtime: Duration) -> JobDescription {
        JobDescription {
            idigest,
            result: JobResult::Timeout,
            runtime: Some(runtime),
        }
    }

    pub fn infeasible(idigest: InstanceDigest, runtime: Option<Duration>) -> JobDescription {
        JobDescription {
            idigest,
            result: JobResult::Infeasible,
            runtime,
        }
    }

    pub fn size_estimate(&self) -> usize {
        50 + if let JobResult::Valid { solution, .. } = &self.result {
            solution.len()
        } else {
            0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pace26checker::digest::digest_output::DIGEST_BYTES;

    fn instance_digest_placeholder() -> InstanceDigest {
        let buffer = [0u8; DIGEST_BYTES];
        buffer.into()
    }

    #[test]
    fn serde_timeout() {
        let org =
            JobDescription::timeout(instance_digest_placeholder(), Duration::from_millis(123456));

        let serialized = serde_json::to_string(&org).unwrap();
        let deserialized: JobDescription = serde_json::from_str(&serialized).unwrap();

        assert_eq!(org, deserialized);
    }

    #[test]
    fn serde_infeasible() {
        for org in [
            JobDescription::infeasible(
                instance_digest_placeholder(),
                Some(Duration::from_millis(123456)),
            ),
            JobDescription::infeasible(instance_digest_placeholder(), None),
        ] {
            let serialized = serde_json::to_string(&org).unwrap();
            let deserialized: JobDescription = serde_json::from_str(&serialized).unwrap();

            assert_eq!(org, deserialized);
        }
    }
}
