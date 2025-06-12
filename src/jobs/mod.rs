pub mod startup;

use anyhow::Result;

pub trait Job {
    async fn run(&self) -> Result<()>;
}

pub struct JobRunner<T: Job> {
    jobs: Vec<T>,
}

impl<T> JobRunner<T>
where
    T: Job,
{
    pub fn new() -> Self {
        Self { jobs: Vec::new() }
    }

    pub fn add_job(mut self, job: T) -> Self {
        self.jobs.push(job);
        self
    }

    pub async fn run_all(&self) -> Result<()> {
        for job in &self.jobs {
            job.run().await?;
        }
        Ok(())
    }
}
