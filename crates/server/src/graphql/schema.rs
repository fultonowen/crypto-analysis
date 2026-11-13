use crate::graphql::query::QueryRoot;
use crate::graphql::subscription::SubscriptionRoot;
use async_graphql::{EmptyMutation, Schema};

/// GraphQL Schema Representation for the Service
/// We support a single Query root for accessing data
/// as well as (potentially) a subscription based access of data
//todo: update comment here
pub type AppSchema = Schema<QueryRoot, EmptyMutation, SubscriptionRoot>;
