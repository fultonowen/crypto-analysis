use async_graphql::{EmptyMutation, Schema};

use crate::graphql::{query::QueryRoot, subscription::SubscriptionRoot};

/// GraphQL Schema Representation for the Service
/// We support a single Query root for accessing data
/// as well as (potentially) a subscription based access of data
//todo: update comment here
pub type AppSchema = Schema<QueryRoot, EmptyMutation, SubscriptionRoot>;
