//! Deployments interface
extern crate serde_json;

use self::super::{Github, Result};
use rep::{Deployment, DeploymentOptions, DeploymentListOptions, DeploymentStatus,
          DeploymentStatusOptions};

/// Interface for repository deployements
pub struct Deployments<'a> {
    github: &'a Github<'a>,
    owner: String,
    repo: String,
}

/// INterface for deployment statuses
pub struct DeploymentStatuses<'a> {
    github: &'a Github<'a>,
    owner: String,
    repo: String,
    id: u64,
}

impl<'a> DeploymentStatuses<'a> {
    /// creates a new deployment status
    pub fn new<O, R>(github: &'a Github<'a>, owner: O, repo: R, id: u64) -> DeploymentStatuses<'a>
        where O: Into<String>,
              R: Into<String>
    {
        DeploymentStatuses {
            github: github,
            owner: owner.into(),
            repo: repo.into(),
            id: id,
        }
    }

    fn path(&self, more: &str) -> String {
        format!("/repos/{}/{}/deployments/{}/statuses{}",
                self.owner,
                self.repo,
                self.id,
                more)
    }

    /// lists all statuses associated with a deployment
    pub fn list(&self) -> Result<Vec<DeploymentStatus>> {
        self.github.get::<Vec<DeploymentStatus>>(&self.path(""))
    }

    /// creates a new deployment status. For convenience, a DeploymentStatusOptions.builder
    /// interface is required for building up a request
    pub fn create(&self, status: &DeploymentStatusOptions) -> Result<DeploymentStatus> {
        let data = try!(serde_json::to_string(&status));
        self.github.post::<DeploymentStatus>(&self.path(""), &data.as_bytes())
    }
}

impl<'a> Deployments<'a> {
    /// Create a new deployments instance
    pub fn new<O, R>(github: &'a Github<'a>, owner: O, repo: R) -> Deployments<'a>
        where O: Into<String>,
              R: Into<String>
    {
        Deployments {
            github: github,
            owner: owner.into(),
            repo: repo.into(),
        }
    }

    fn path(&self, more: &str) -> String {
        format!("/repos/{}/{}/deployments{}", self.owner, self.repo, more)
    }

    /// lists all deployments for a repository
    pub fn list(&self, opts: &DeploymentListOptions) -> Result<Vec<Deployment>> {
        let mut uri = vec![self.path("")];
        if let Some(query) = opts.serialize() {
            uri.push(query);
        }
        self.github.get::<Vec<Deployment>>(&uri.join("?"))
    }

    /// creates a new deployment for this repository
    pub fn create(&self, dep: &DeploymentOptions) -> Result<Deployment> {
        let data = try!(serde_json::to_string(&dep));
        self.github.post::<Deployment>(&self.path(""), data.as_bytes())
    }

    /// get a reference to the statuses api for a give deployment
    pub fn statuses(&self, id: u64) -> DeploymentStatuses {
        DeploymentStatuses::new(self.github, self.owner.as_str(), self.repo.as_str(), id)
    }
}
