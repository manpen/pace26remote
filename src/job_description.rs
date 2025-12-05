use pace26checker::checks::bin_tree_with_parent::NodeCursor;
use pace26checker::io::digest::digest_solution;
use pace26io::newick::NewickWriter;
use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
enum JobResult {
    Valid { solution: String, sdigest: String },
    Infeasible,
    Timeout,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct JobDescription {
    idigest: String,
    result: JobResult,
    runtime: Option<Duration>,
}

impl JobDescription {
    pub fn valid(
        idigest: String,
        trees: Vec<NodeCursor>,
        runtime: Option<Duration>,
    ) -> JobDescription {
        let score = trees.len() as u32;

        // digest solution normalizes the order of children
        let sdigest = digest_solution(trees.clone(), score);

        // compute newick strings and sort by size
        let mut newick: Vec<_> = trees.into_iter().map(|t| t.to_newick_string()).collect();
        newick.sort_unstable_by_key(|t| t.len());
        let solution = newick.join("\n");

        JobDescription {
            idigest,
            runtime,
            result: JobResult::Valid { sdigest, solution },
        }
    }

    pub fn timeout(idigest: String, runtime: Duration) -> JobDescription {
        JobDescription {
            idigest,
            result: JobResult::Timeout,
            runtime: Some(runtime),
        }
    }

    pub fn infeasible(idigest: String, runtime: Option<Duration>) -> JobDescription {
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
    use pace26checker::io::digest::DIGEST_HEX_DIGITS;

    fn dummy_digest() -> String {
        std::iter::repeat_n("0", DIGEST_HEX_DIGITS).collect()
    }

    #[test]
    fn serde_timeout() {
        let org = JobDescription::timeout(dummy_digest(), Duration::from_millis(123456));

        let serialized = serde_json::to_string(&org).unwrap();
        let deserialized: JobDescription = serde_json::from_str(&serialized).unwrap();

        assert_eq!(org, deserialized);
    }

    #[test]
    fn serde_infeasible() {
        for org in [
            JobDescription::infeasible(dummy_digest(), Some(Duration::from_millis(123456))),
            JobDescription::infeasible(dummy_digest(), None),
        ] {
            let serialized = serde_json::to_string(&org).unwrap();
            let deserialized: JobDescription = serde_json::from_str(&serialized).unwrap();

            assert_eq!(org, deserialized);
        }
    }
}
