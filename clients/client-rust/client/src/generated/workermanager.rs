#![allow(unused_imports)]
#![cfg_attr(rustfmt, rustfmt_skip)]
/* THIS FILE IS AUTOMATICALLY GENERATED. DO NOT EDIT */
use crate::{Client, ClientBuilder, Credentials, Retry};
use anyhow::Error;
use serde_json::Value;
use std::time::Duration;
use crate::util::urlencode;

/// Worker Manager Service
///
/// This service manages workers, including provisioning for dynamic worker pools.
///
/// Methods interacting with a provider may return a 503 response if that provider has
/// not been able to start up, such as if the service to which it interfaces has an
/// outage.  Such requests can be retried as for any other 5xx response.
pub struct WorkerManager {
    /// The underlying client used to make API calls for this service.
    pub client: Client
}

#[allow(non_snake_case)]
impl WorkerManager {
    /// Create a new WorkerManager instance, based on the given client builder
    pub fn new<CB: Into<ClientBuilder>>(client_builder: CB) -> Result<Self, Error> {
        Ok(Self{
            client: client_builder
                .into()
                .path_prefix("api/worker-manager/v1/")
                .build()?,
        })
    }

    /// Ping Server
    ///
    /// Respond without doing anything.
    /// This endpoint is used to check that the service is up.
    pub async fn ping(&self) -> Result<(), Error> {
        let method = "GET";
        let (path, query) = Self::ping_details();
        let body = None;
        let resp = self.client.request(method, path, query, body).await?;
        resp.bytes().await?;
        Ok(())
    }

    /// Generate an unsigned URL for the ping endpoint
    pub fn ping_url(&self) -> Result<String, Error> {
        let (path, query) = Self::ping_details();
        self.client.make_url(path, query)
    }

    /// Generate a signed URL for the ping endpoint
    pub fn ping_signed_url(&self, ttl: Duration) -> Result<String, Error> {
        let (path, query) = Self::ping_details();
        self.client.make_signed_url(path, query, ttl)
    }

    /// Determine the HTTP request details for ping
    fn ping_details<'a>() -> (&'static str, Option<Vec<(&'static str, &'a str)>>) {
        let path = "ping";
        let query = None;

        (path, query)
    }

    /// Load Balancer Heartbeat
    ///
    /// Respond without doing anything.
    /// This endpoint is used to check that the service is up.
    pub async fn lbheartbeat(&self) -> Result<(), Error> {
        let method = "GET";
        let (path, query) = Self::lbheartbeat_details();
        let body = None;
        let resp = self.client.request(method, path, query, body).await?;
        resp.bytes().await?;
        Ok(())
    }

    /// Generate an unsigned URL for the lbheartbeat endpoint
    pub fn lbheartbeat_url(&self) -> Result<String, Error> {
        let (path, query) = Self::lbheartbeat_details();
        self.client.make_url(path, query)
    }

    /// Generate a signed URL for the lbheartbeat endpoint
    pub fn lbheartbeat_signed_url(&self, ttl: Duration) -> Result<String, Error> {
        let (path, query) = Self::lbheartbeat_details();
        self.client.make_signed_url(path, query, ttl)
    }

    /// Determine the HTTP request details for lbheartbeat
    fn lbheartbeat_details<'a>() -> (&'static str, Option<Vec<(&'static str, &'a str)>>) {
        let path = "__lbheartbeat__";
        let query = None;

        (path, query)
    }

    /// Taskcluster Version
    ///
    /// Respond with the JSON version object.
    /// https://github.com/mozilla-services/Dockerflow/blob/main/docs/version_object.md
    pub async fn version(&self) -> Result<(), Error> {
        let method = "GET";
        let (path, query) = Self::version_details();
        let body = None;
        let resp = self.client.request(method, path, query, body).await?;
        resp.bytes().await?;
        Ok(())
    }

    /// Generate an unsigned URL for the version endpoint
    pub fn version_url(&self) -> Result<String, Error> {
        let (path, query) = Self::version_details();
        self.client.make_url(path, query)
    }

    /// Generate a signed URL for the version endpoint
    pub fn version_signed_url(&self, ttl: Duration) -> Result<String, Error> {
        let (path, query) = Self::version_details();
        self.client.make_signed_url(path, query, ttl)
    }

    /// Determine the HTTP request details for version
    fn version_details<'a>() -> (&'static str, Option<Vec<(&'static str, &'a str)>>) {
        let path = "__version__";
        let query = None;

        (path, query)
    }

    /// List Providers
    ///
    /// Retrieve a list of providers that are available for worker pools.
    pub async fn listProviders(&self, continuationToken: Option<&str>, limit: Option<&str>) -> Result<Value, Error> {
        let method = "GET";
        let (path, query) = Self::listProviders_details(continuationToken, limit);
        let body = None;
        let resp = self.client.request(method, path, query, body).await?;
        Ok(resp.json().await?)
    }

    /// Generate an unsigned URL for the listProviders endpoint
    pub fn listProviders_url(&self, continuationToken: Option<&str>, limit: Option<&str>) -> Result<String, Error> {
        let (path, query) = Self::listProviders_details(continuationToken, limit);
        self.client.make_url(path, query)
    }

    /// Generate a signed URL for the listProviders endpoint
    pub fn listProviders_signed_url(&self, continuationToken: Option<&str>, limit: Option<&str>, ttl: Duration) -> Result<String, Error> {
        let (path, query) = Self::listProviders_details(continuationToken, limit);
        self.client.make_signed_url(path, query, ttl)
    }

    /// Determine the HTTP request details for listProviders
    fn listProviders_details<'a>(continuationToken: Option<&'a str>, limit: Option<&'a str>) -> (&'static str, Option<Vec<(&'static str, &'a str)>>) {
        let path = "providers";
        let mut query = None;
        if let Some(q) = continuationToken {
            query.get_or_insert_with(Vec::new).push(("continuationToken", q));
        }
        if let Some(q) = limit {
            query.get_or_insert_with(Vec::new).push(("limit", q));
        }

        (path, query)
    }

    /// Create Worker Pool
    ///
    /// Create a new worker pool. If the worker pool already exists, this will throw an error.
    pub async fn createWorkerPool(&self, workerPoolId: &str, payload: &Value) -> Result<Value, Error> {
        let method = "PUT";
        let (path, query) = Self::createWorkerPool_details(workerPoolId);
        let body = Some(payload);
        let resp = self.client.request(method, &path, query, body).await?;
        Ok(resp.json().await?)
    }

    /// Determine the HTTP request details for createWorkerPool
    fn createWorkerPool_details<'a>(workerPoolId: &'a str) -> (String, Option<Vec<(&'static str, &'a str)>>) {
        let path = format!("worker-pool/{}", urlencode(workerPoolId));
        let query = None;

        (path, query)
    }

    /// Update Worker Pool
    ///
    /// Given an existing worker pool definition, this will modify it and return
    /// the new definition.
    ///
    /// To delete a worker pool, set its `providerId` to `"null-provider"`.
    /// After any existing workers have exited, a cleanup job will remove the
    /// worker pool.  During that time, the worker pool can be updated again, such
    /// as to set its `providerId` to a real provider.
    pub async fn updateWorkerPool(&self, workerPoolId: &str, payload: &Value) -> Result<Value, Error> {
        let method = "POST";
        let (path, query) = Self::updateWorkerPool_details(workerPoolId);
        let body = Some(payload);
        let resp = self.client.request(method, &path, query, body).await?;
        Ok(resp.json().await?)
    }

    /// Determine the HTTP request details for updateWorkerPool
    fn updateWorkerPool_details<'a>(workerPoolId: &'a str) -> (String, Option<Vec<(&'static str, &'a str)>>) {
        let path = format!("worker-pool/{}", urlencode(workerPoolId));
        let query = None;

        (path, query)
    }

    /// Delete Worker Pool
    ///
    /// Mark a worker pool for deletion.  This is the same as updating the pool to
    /// set its providerId to `"null-provider"`, but does not require scope
    /// `worker-manager:provider:null-provider`.
    pub async fn deleteWorkerPool(&self, workerPoolId: &str) -> Result<Value, Error> {
        let method = "DELETE";
        let (path, query) = Self::deleteWorkerPool_details(workerPoolId);
        let body = None;
        let resp = self.client.request(method, &path, query, body).await?;
        Ok(resp.json().await?)
    }

    /// Determine the HTTP request details for deleteWorkerPool
    fn deleteWorkerPool_details<'a>(workerPoolId: &'a str) -> (String, Option<Vec<(&'static str, &'a str)>>) {
        let path = format!("worker-pool/{}", urlencode(workerPoolId));
        let query = None;

        (path, query)
    }

    /// Get Worker Pool
    ///
    /// Fetch an existing worker pool defition.
    pub async fn workerPool(&self, workerPoolId: &str) -> Result<Value, Error> {
        let method = "GET";
        let (path, query) = Self::workerPool_details(workerPoolId);
        let body = None;
        let resp = self.client.request(method, &path, query, body).await?;
        Ok(resp.json().await?)
    }

    /// Generate an unsigned URL for the workerPool endpoint
    pub fn workerPool_url(&self, workerPoolId: &str) -> Result<String, Error> {
        let (path, query) = Self::workerPool_details(workerPoolId);
        self.client.make_url(&path, query)
    }

    /// Generate a signed URL for the workerPool endpoint
    pub fn workerPool_signed_url(&self, workerPoolId: &str, ttl: Duration) -> Result<String, Error> {
        let (path, query) = Self::workerPool_details(workerPoolId);
        self.client.make_signed_url(&path, query, ttl)
    }

    /// Determine the HTTP request details for workerPool
    fn workerPool_details<'a>(workerPoolId: &'a str) -> (String, Option<Vec<(&'static str, &'a str)>>) {
        let path = format!("worker-pool/{}", urlencode(workerPoolId));
        let query = None;

        (path, query)
    }

    /// List All Worker Pools
    ///
    /// Get the list of all the existing worker pools.
    pub async fn listWorkerPools(&self, continuationToken: Option<&str>, limit: Option<&str>) -> Result<Value, Error> {
        let method = "GET";
        let (path, query) = Self::listWorkerPools_details(continuationToken, limit);
        let body = None;
        let resp = self.client.request(method, path, query, body).await?;
        Ok(resp.json().await?)
    }

    /// Generate an unsigned URL for the listWorkerPools endpoint
    pub fn listWorkerPools_url(&self, continuationToken: Option<&str>, limit: Option<&str>) -> Result<String, Error> {
        let (path, query) = Self::listWorkerPools_details(continuationToken, limit);
        self.client.make_url(path, query)
    }

    /// Generate a signed URL for the listWorkerPools endpoint
    pub fn listWorkerPools_signed_url(&self, continuationToken: Option<&str>, limit: Option<&str>, ttl: Duration) -> Result<String, Error> {
        let (path, query) = Self::listWorkerPools_details(continuationToken, limit);
        self.client.make_signed_url(path, query, ttl)
    }

    /// Determine the HTTP request details for listWorkerPools
    fn listWorkerPools_details<'a>(continuationToken: Option<&'a str>, limit: Option<&'a str>) -> (&'static str, Option<Vec<(&'static str, &'a str)>>) {
        let path = "worker-pools";
        let mut query = None;
        if let Some(q) = continuationToken {
            query.get_or_insert_with(Vec::new).push(("continuationToken", q));
        }
        if let Some(q) = limit {
            query.get_or_insert_with(Vec::new).push(("limit", q));
        }

        (path, query)
    }

    /// Report an error from a worker
    ///
    /// Report an error that occurred on a worker.  This error will be included
    /// with the other errors in `listWorkerPoolErrors(workerPoolId)`.
    ///
    /// Workers can use this endpoint to report startup or configuration errors
    /// that might be associated with the worker pool configuration and thus of
    /// interest to a worker-pool administrator.
    ///
    /// NOTE: errors are publicly visible.  Ensure that none of the content
    /// contains secrets or other sensitive information.
    pub async fn reportWorkerError(&self, workerPoolId: &str, payload: &Value) -> Result<Value, Error> {
        let method = "POST";
        let (path, query) = Self::reportWorkerError_details(workerPoolId);
        let body = Some(payload);
        let resp = self.client.request(method, &path, query, body).await?;
        Ok(resp.json().await?)
    }

    /// Determine the HTTP request details for reportWorkerError
    fn reportWorkerError_details<'a>(workerPoolId: &'a str) -> (String, Option<Vec<(&'static str, &'a str)>>) {
        let path = format!("worker-pool-errors/{}", urlencode(workerPoolId));
        let query = None;

        (path, query)
    }

    /// List Worker Pool Errors Count
    ///
    /// Get the list of worker pool errors count.
    /// Contains total count of errors for the past 7 days and 24 hours
    /// Also includes total counts grouped by titles of error and error code.
    ///
    /// If `workerPoolId` is not specified, it will return the count of all errors
    pub async fn workerPoolErrorStats(&self, workerPoolId: Option<&str>) -> Result<Value, Error> {
        let method = "GET";
        let (path, query) = Self::workerPoolErrorStats_details(workerPoolId);
        let body = None;
        let resp = self.client.request(method, path, query, body).await?;
        Ok(resp.json().await?)
    }

    /// Generate an unsigned URL for the workerPoolErrorStats endpoint
    pub fn workerPoolErrorStats_url(&self, workerPoolId: Option<&str>) -> Result<String, Error> {
        let (path, query) = Self::workerPoolErrorStats_details(workerPoolId);
        self.client.make_url(path, query)
    }

    /// Generate a signed URL for the workerPoolErrorStats endpoint
    pub fn workerPoolErrorStats_signed_url(&self, workerPoolId: Option<&str>, ttl: Duration) -> Result<String, Error> {
        let (path, query) = Self::workerPoolErrorStats_details(workerPoolId);
        self.client.make_signed_url(path, query, ttl)
    }

    /// Determine the HTTP request details for workerPoolErrorStats
    fn workerPoolErrorStats_details<'a>(workerPoolId: Option<&'a str>) -> (&'static str, Option<Vec<(&'static str, &'a str)>>) {
        let path = "worker-pool-errors/stats";
        let mut query = None;
        if let Some(q) = workerPoolId {
            query.get_or_insert_with(Vec::new).push(("workerPoolId", q));
        }

        (path, query)
    }

    /// List Worker Pool Errors
    ///
    /// Get the list of worker pool errors.
    pub async fn listWorkerPoolErrors(&self, workerPoolId: &str, continuationToken: Option<&str>, limit: Option<&str>) -> Result<Value, Error> {
        let method = "GET";
        let (path, query) = Self::listWorkerPoolErrors_details(workerPoolId, continuationToken, limit);
        let body = None;
        let resp = self.client.request(method, &path, query, body).await?;
        Ok(resp.json().await?)
    }

    /// Generate an unsigned URL for the listWorkerPoolErrors endpoint
    pub fn listWorkerPoolErrors_url(&self, workerPoolId: &str, continuationToken: Option<&str>, limit: Option<&str>) -> Result<String, Error> {
        let (path, query) = Self::listWorkerPoolErrors_details(workerPoolId, continuationToken, limit);
        self.client.make_url(&path, query)
    }

    /// Generate a signed URL for the listWorkerPoolErrors endpoint
    pub fn listWorkerPoolErrors_signed_url(&self, workerPoolId: &str, continuationToken: Option<&str>, limit: Option<&str>, ttl: Duration) -> Result<String, Error> {
        let (path, query) = Self::listWorkerPoolErrors_details(workerPoolId, continuationToken, limit);
        self.client.make_signed_url(&path, query, ttl)
    }

    /// Determine the HTTP request details for listWorkerPoolErrors
    fn listWorkerPoolErrors_details<'a>(workerPoolId: &'a str, continuationToken: Option<&'a str>, limit: Option<&'a str>) -> (String, Option<Vec<(&'static str, &'a str)>>) {
        let path = format!("worker-pool-errors/{}", urlencode(workerPoolId));
        let mut query = None;
        if let Some(q) = continuationToken {
            query.get_or_insert_with(Vec::new).push(("continuationToken", q));
        }
        if let Some(q) = limit {
            query.get_or_insert_with(Vec::new).push(("limit", q));
        }

        (path, query)
    }

    /// Workers in a specific Worker Group in a Worker Pool
    ///
    /// Get the list of all the existing workers in a given group in a given worker pool.
    pub async fn listWorkersForWorkerGroup(&self, workerPoolId: &str, workerGroup: &str, continuationToken: Option<&str>, limit: Option<&str>) -> Result<Value, Error> {
        let method = "GET";
        let (path, query) = Self::listWorkersForWorkerGroup_details(workerPoolId, workerGroup, continuationToken, limit);
        let body = None;
        let resp = self.client.request(method, &path, query, body).await?;
        Ok(resp.json().await?)
    }

    /// Generate an unsigned URL for the listWorkersForWorkerGroup endpoint
    pub fn listWorkersForWorkerGroup_url(&self, workerPoolId: &str, workerGroup: &str, continuationToken: Option<&str>, limit: Option<&str>) -> Result<String, Error> {
        let (path, query) = Self::listWorkersForWorkerGroup_details(workerPoolId, workerGroup, continuationToken, limit);
        self.client.make_url(&path, query)
    }

    /// Generate a signed URL for the listWorkersForWorkerGroup endpoint
    pub fn listWorkersForWorkerGroup_signed_url(&self, workerPoolId: &str, workerGroup: &str, continuationToken: Option<&str>, limit: Option<&str>, ttl: Duration) -> Result<String, Error> {
        let (path, query) = Self::listWorkersForWorkerGroup_details(workerPoolId, workerGroup, continuationToken, limit);
        self.client.make_signed_url(&path, query, ttl)
    }

    /// Determine the HTTP request details for listWorkersForWorkerGroup
    fn listWorkersForWorkerGroup_details<'a>(workerPoolId: &'a str, workerGroup: &'a str, continuationToken: Option<&'a str>, limit: Option<&'a str>) -> (String, Option<Vec<(&'static str, &'a str)>>) {
        let path = format!("workers/{}/{}", urlencode(workerPoolId), urlencode(workerGroup));
        let mut query = None;
        if let Some(q) = continuationToken {
            query.get_or_insert_with(Vec::new).push(("continuationToken", q));
        }
        if let Some(q) = limit {
            query.get_or_insert_with(Vec::new).push(("limit", q));
        }

        (path, query)
    }

    /// Get a Worker
    ///
    /// Get a single worker.
    pub async fn worker(&self, workerPoolId: &str, workerGroup: &str, workerId: &str) -> Result<Value, Error> {
        let method = "GET";
        let (path, query) = Self::worker_details(workerPoolId, workerGroup, workerId);
        let body = None;
        let resp = self.client.request(method, &path, query, body).await?;
        Ok(resp.json().await?)
    }

    /// Generate an unsigned URL for the worker endpoint
    pub fn worker_url(&self, workerPoolId: &str, workerGroup: &str, workerId: &str) -> Result<String, Error> {
        let (path, query) = Self::worker_details(workerPoolId, workerGroup, workerId);
        self.client.make_url(&path, query)
    }

    /// Generate a signed URL for the worker endpoint
    pub fn worker_signed_url(&self, workerPoolId: &str, workerGroup: &str, workerId: &str, ttl: Duration) -> Result<String, Error> {
        let (path, query) = Self::worker_details(workerPoolId, workerGroup, workerId);
        self.client.make_signed_url(&path, query, ttl)
    }

    /// Determine the HTTP request details for worker
    fn worker_details<'a>(workerPoolId: &'a str, workerGroup: &'a str, workerId: &'a str) -> (String, Option<Vec<(&'static str, &'a str)>>) {
        let path = format!("workers/{}/{}/{}", urlencode(workerPoolId), urlencode(workerGroup), urlencode(workerId));
        let query = None;

        (path, query)
    }

    /// Create a Worker
    ///
    /// Create a new worker.  This is only useful for worker pools where the provider
    /// does not create workers automatically, such as those with a `static` provider
    /// type.  Providers that do not support creating workers will return a 400 error.
    /// See the documentation for the individual providers, and in particular the
    /// [static provider](https://docs.taskcluster.net/docs/reference/core/worker-manager/)
    /// for more information.
    pub async fn createWorker(&self, workerPoolId: &str, workerGroup: &str, workerId: &str, payload: &Value) -> Result<Value, Error> {
        let method = "PUT";
        let (path, query) = Self::createWorker_details(workerPoolId, workerGroup, workerId);
        let body = Some(payload);
        let resp = self.client.request(method, &path, query, body).await?;
        Ok(resp.json().await?)
    }

    /// Determine the HTTP request details for createWorker
    fn createWorker_details<'a>(workerPoolId: &'a str, workerGroup: &'a str, workerId: &'a str) -> (String, Option<Vec<(&'static str, &'a str)>>) {
        let path = format!("workers/{}/{}/{}", urlencode(workerPoolId), urlencode(workerGroup), urlencode(workerId));
        let query = None;

        (path, query)
    }

    /// Update an existing Worker
    ///
    /// Update an existing worker in-place.  Like `createWorker`, this is only useful for
    /// worker pools where the provider does not create workers automatically.
    /// This method allows updating all fields in the schema unless otherwise indicated
    /// in the provider documentation.
    /// See the documentation for the individual providers, and in particular the
    /// [static provider](https://docs.taskcluster.net/docs/reference/core/worker-manager/)
    /// for more information.
    pub async fn updateWorker(&self, workerPoolId: &str, workerGroup: &str, workerId: &str, payload: &Value) -> Result<Value, Error> {
        let method = "POST";
        let (path, query) = Self::updateWorker_details(workerPoolId, workerGroup, workerId);
        let body = Some(payload);
        let resp = self.client.request(method, &path, query, body).await?;
        Ok(resp.json().await?)
    }

    /// Determine the HTTP request details for updateWorker
    fn updateWorker_details<'a>(workerPoolId: &'a str, workerGroup: &'a str, workerId: &'a str) -> (String, Option<Vec<(&'static str, &'a str)>>) {
        let path = format!("workers/{}/{}/{}", urlencode(workerPoolId), urlencode(workerGroup), urlencode(workerId));
        let query = None;

        (path, query)
    }

    /// Remove a Worker
    ///
    /// Remove an existing worker.  The precise behavior of this method depends
    /// on the provider implementing the given worker.  Some providers
    /// do not support removing workers at all, and will return a 400 error.
    /// Others may begin removing the worker, but it may remain available via
    /// the API (perhaps even in state RUNNING) afterward.
    pub async fn removeWorker(&self, workerPoolId: &str, workerGroup: &str, workerId: &str) -> Result<(), Error> {
        let method = "DELETE";
        let (path, query) = Self::removeWorker_details(workerPoolId, workerGroup, workerId);
        let body = None;
        let resp = self.client.request(method, &path, query, body).await?;
        resp.bytes().await?;
        Ok(())
    }

    /// Determine the HTTP request details for removeWorker
    fn removeWorker_details<'a>(workerPoolId: &'a str, workerGroup: &'a str, workerId: &'a str) -> (String, Option<Vec<(&'static str, &'a str)>>) {
        let path = format!("workers/{}/{}/{}", urlencode(workerPoolId), urlencode(workerGroup), urlencode(workerId));
        let query = None;

        (path, query)
    }

    /// Workers in a Worker Pool
    ///
    /// Get the list of all the existing workers in a given worker pool.
    pub async fn listWorkersForWorkerPool(&self, workerPoolId: &str, continuationToken: Option<&str>, limit: Option<&str>, state: Option<&str>) -> Result<Value, Error> {
        let method = "GET";
        let (path, query) = Self::listWorkersForWorkerPool_details(workerPoolId, continuationToken, limit, state);
        let body = None;
        let resp = self.client.request(method, &path, query, body).await?;
        Ok(resp.json().await?)
    }

    /// Generate an unsigned URL for the listWorkersForWorkerPool endpoint
    pub fn listWorkersForWorkerPool_url(&self, workerPoolId: &str, continuationToken: Option<&str>, limit: Option<&str>, state: Option<&str>) -> Result<String, Error> {
        let (path, query) = Self::listWorkersForWorkerPool_details(workerPoolId, continuationToken, limit, state);
        self.client.make_url(&path, query)
    }

    /// Generate a signed URL for the listWorkersForWorkerPool endpoint
    pub fn listWorkersForWorkerPool_signed_url(&self, workerPoolId: &str, continuationToken: Option<&str>, limit: Option<&str>, state: Option<&str>, ttl: Duration) -> Result<String, Error> {
        let (path, query) = Self::listWorkersForWorkerPool_details(workerPoolId, continuationToken, limit, state);
        self.client.make_signed_url(&path, query, ttl)
    }

    /// Determine the HTTP request details for listWorkersForWorkerPool
    fn listWorkersForWorkerPool_details<'a>(workerPoolId: &'a str, continuationToken: Option<&'a str>, limit: Option<&'a str>, state: Option<&'a str>) -> (String, Option<Vec<(&'static str, &'a str)>>) {
        let path = format!("workers/{}", urlencode(workerPoolId));
        let mut query = None;
        if let Some(q) = continuationToken {
            query.get_or_insert_with(Vec::new).push(("continuationToken", q));
        }
        if let Some(q) = limit {
            query.get_or_insert_with(Vec::new).push(("limit", q));
        }
        if let Some(q) = state {
            query.get_or_insert_with(Vec::new).push(("state", q));
        }

        (path, query)
    }

    /// Register a running worker
    ///
    /// Register a running worker.  Workers call this method on worker start-up.
    ///
    /// This call both marks the worker as running and returns the credentials
    /// the worker will require to perform its work.  The worker must provide
    /// some proof of its identity, and that proof varies by provider type.
    pub async fn registerWorker(&self, payload: &Value) -> Result<Value, Error> {
        let method = "POST";
        let (path, query) = Self::registerWorker_details();
        let body = Some(payload);
        let resp = self.client.request(method, path, query, body).await?;
        Ok(resp.json().await?)
    }

    /// Determine the HTTP request details for registerWorker
    fn registerWorker_details<'a>() -> (&'static str, Option<Vec<(&'static str, &'a str)>>) {
        let path = "worker/register";
        let query = None;

        (path, query)
    }

    /// Reregister a Worker
    ///
    /// Reregister a running worker.
    ///
    /// This will generate and return new Taskcluster credentials for the worker
    /// on that instance to use. The credentials will not live longer the
    /// `registrationTimeout` for that worker. The endpoint will update `terminateAfter`
    /// for the worker so that worker-manager does not terminate the instance.
    pub async fn reregisterWorker(&self, payload: &Value) -> Result<Value, Error> {
        let method = "POST";
        let (path, query) = Self::reregisterWorker_details();
        let body = Some(payload);
        let resp = self.client.request(method, path, query, body).await?;
        Ok(resp.json().await?)
    }

    /// Determine the HTTP request details for reregisterWorker
    fn reregisterWorker_details<'a>() -> (&'static str, Option<Vec<(&'static str, &'a str)>>) {
        let path = "worker/reregister";
        let query = None;

        (path, query)
    }

    /// Get a list of all active workers of a workerType
    ///
    /// Get a list of all active workers of a workerType.
    ///
    /// `listWorkers` allows a response to be filtered by quarantined and non quarantined workers,
    /// as well as the current state of the worker.
    /// To filter the query, you should call the end-point with one of [`quarantined`, `workerState`]
    /// as a query-string option with a true or false value.
    ///
    /// The response is paged. If this end-point returns a `continuationToken`, you
    /// should call the end-point again with the `continuationToken` as a query-string
    /// option. By default this end-point will list up to 1000 workers in a single
    /// page. You may limit this with the query-string parameter `limit`.
    pub async fn listWorkers(&self, provisionerId: &str, workerType: &str, continuationToken: Option<&str>, limit: Option<&str>, quarantined: Option<&str>, workerState: Option<&str>) -> Result<Value, Error> {
        let method = "GET";
        let (path, query) = Self::listWorkers_details(provisionerId, workerType, continuationToken, limit, quarantined, workerState);
        let body = None;
        let resp = self.client.request(method, &path, query, body).await?;
        Ok(resp.json().await?)
    }

    /// Generate an unsigned URL for the listWorkers endpoint
    pub fn listWorkers_url(&self, provisionerId: &str, workerType: &str, continuationToken: Option<&str>, limit: Option<&str>, quarantined: Option<&str>, workerState: Option<&str>) -> Result<String, Error> {
        let (path, query) = Self::listWorkers_details(provisionerId, workerType, continuationToken, limit, quarantined, workerState);
        self.client.make_url(&path, query)
    }

    /// Generate a signed URL for the listWorkers endpoint
    pub fn listWorkers_signed_url(&self, provisionerId: &str, workerType: &str, continuationToken: Option<&str>, limit: Option<&str>, quarantined: Option<&str>, workerState: Option<&str>, ttl: Duration) -> Result<String, Error> {
        let (path, query) = Self::listWorkers_details(provisionerId, workerType, continuationToken, limit, quarantined, workerState);
        self.client.make_signed_url(&path, query, ttl)
    }

    /// Determine the HTTP request details for listWorkers
    fn listWorkers_details<'a>(provisionerId: &'a str, workerType: &'a str, continuationToken: Option<&'a str>, limit: Option<&'a str>, quarantined: Option<&'a str>, workerState: Option<&'a str>) -> (String, Option<Vec<(&'static str, &'a str)>>) {
        let path = format!("provisioners/{}/worker-types/{}/workers", urlencode(provisionerId), urlencode(workerType));
        let mut query = None;
        if let Some(q) = continuationToken {
            query.get_or_insert_with(Vec::new).push(("continuationToken", q));
        }
        if let Some(q) = limit {
            query.get_or_insert_with(Vec::new).push(("limit", q));
        }
        if let Some(q) = quarantined {
            query.get_or_insert_with(Vec::new).push(("quarantined", q));
        }
        if let Some(q) = workerState {
            query.get_or_insert_with(Vec::new).push(("workerState", q));
        }

        (path, query)
    }

    /// Get a worker
    ///
    /// Get a worker from a worker-type.
    pub async fn getWorker(&self, provisionerId: &str, workerType: &str, workerGroup: &str, workerId: &str) -> Result<Value, Error> {
        let method = "GET";
        let (path, query) = Self::getWorker_details(provisionerId, workerType, workerGroup, workerId);
        let body = None;
        let resp = self.client.request(method, &path, query, body).await?;
        Ok(resp.json().await?)
    }

    /// Generate an unsigned URL for the getWorker endpoint
    pub fn getWorker_url(&self, provisionerId: &str, workerType: &str, workerGroup: &str, workerId: &str) -> Result<String, Error> {
        let (path, query) = Self::getWorker_details(provisionerId, workerType, workerGroup, workerId);
        self.client.make_url(&path, query)
    }

    /// Generate a signed URL for the getWorker endpoint
    pub fn getWorker_signed_url(&self, provisionerId: &str, workerType: &str, workerGroup: &str, workerId: &str, ttl: Duration) -> Result<String, Error> {
        let (path, query) = Self::getWorker_details(provisionerId, workerType, workerGroup, workerId);
        self.client.make_signed_url(&path, query, ttl)
    }

    /// Determine the HTTP request details for getWorker
    fn getWorker_details<'a>(provisionerId: &'a str, workerType: &'a str, workerGroup: &'a str, workerId: &'a str) -> (String, Option<Vec<(&'static str, &'a str)>>) {
        let path = format!("provisioners/{}/worker-types/{}/workers/{}/{}", urlencode(provisionerId), urlencode(workerType), urlencode(workerGroup), urlencode(workerId));
        let query = None;

        (path, query)
    }

    /// Heartbeat
    ///
    /// Respond with a service heartbeat.
    ///
    /// This endpoint is used to check on backing services this service
    /// depends on.
    pub async fn heartbeat(&self) -> Result<(), Error> {
        let method = "GET";
        let (path, query) = Self::heartbeat_details();
        let body = None;
        let resp = self.client.request(method, path, query, body).await?;
        resp.bytes().await?;
        Ok(())
    }

    /// Generate an unsigned URL for the heartbeat endpoint
    pub fn heartbeat_url(&self) -> Result<String, Error> {
        let (path, query) = Self::heartbeat_details();
        self.client.make_url(path, query)
    }

    /// Generate a signed URL for the heartbeat endpoint
    pub fn heartbeat_signed_url(&self, ttl: Duration) -> Result<String, Error> {
        let (path, query) = Self::heartbeat_details();
        self.client.make_signed_url(path, query, ttl)
    }

    /// Determine the HTTP request details for heartbeat
    fn heartbeat_details<'a>() -> (&'static str, Option<Vec<(&'static str, &'a str)>>) {
        let path = "__heartbeat__";
        let query = None;

        (path, query)
    }
}
