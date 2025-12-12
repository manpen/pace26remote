use crate::job_description::JobDescription;
use crate::job_transfer::TransferToServer;
use reqwest::{ClientBuilder, IntoUrl, Url};
use std::collections::BinaryHeap;
use thiserror::Error;
use url::ParseError;

const MAX_DESC_PER_UPLOAD: usize = 100;
const MAX_EST_SIZE_PER_UPLOAD: usize = 5_000_000;

#[derive(Debug, Error)]
pub enum UploadError {
    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),

    #[error(transparent)]
    ParseError(#[from] ParseError),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct UploadJob {
    size: usize,
    desc: JobDescription,
}

pub struct Upload {
    url: Url,
    descriptions: BinaryHeap<UploadJob>,
    size_estimate: usize,
}

impl Upload {
    pub fn new_with_server(into_url: impl IntoUrl) -> Result<Self, UploadError> {
        let url = into_url.into_url()?.join("/api/solution")?;
        Self::new_with_endpoint(url)
    }

    pub fn new_with_endpoint(into_url: impl IntoUrl) -> Result<Self, UploadError> {
        let url = into_url.into_url()?;

        Ok(Self {
            url,
            descriptions: BinaryHeap::with_capacity(1000),
            size_estimate: 0,
        })
    }

    pub fn add_job(&mut self, desc: JobDescription) {
        let size = desc.size_estimate();
        self.descriptions.push(UploadJob { size, desc });
        self.size_estimate += size;
    }

    pub async fn upload_if_necessary(&mut self) -> Result<(), UploadError> {
        if self.size_estimate * 4 > MAX_EST_SIZE_PER_UPLOAD * 3
            || self.descriptions.len() * 4 > MAX_DESC_PER_UPLOAD
        {
            self.upload().await
        } else {
            Ok(())
        }
    }

    pub async fn flush(&mut self) -> Result<(), UploadError> {
        while !self.descriptions.is_empty() {
            self.upload().await?;
        }
        Ok(())
    }

    async fn upload(&mut self) -> Result<(), UploadError> {
        let payload = self.build_payload();
        let client = ClientBuilder::new()
            .danger_accept_invalid_certs(true)
            .build()?;

        let response = client.post(self.url.clone()).json(&payload).send().await?;
        println!("response: {}", response.text().await?);

        Ok(())
    }

    fn build_payload(&mut self) -> TransferToServer {
        let mut jobs = Vec::with_capacity(self.descriptions.len().min(MAX_DESC_PER_UPLOAD));
        let mut est_size = 0;

        while est_size < MAX_EST_SIZE_PER_UPLOAD && jobs.len() < MAX_DESC_PER_UPLOAD {
            if let Some(job) = self.descriptions.pop() {
                jobs.push(job.desc);
                est_size += job.size;
                self.size_estimate -= job.size;
            } else {
                break;
            }
        }

        TransferToServer { jobs }
    }
}
