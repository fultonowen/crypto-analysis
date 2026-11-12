use crate::graphql::query::QueryRoot;
use crate::graphql::subscription::SubscriptionRoot;
use async_graphql::{EmptyMutation, Schema};

/// GraphQL Schema Representation for the Service
/// We support a single
type AppSchema = Schema<QueryRoot, EmptyMutation, SubscriptionRoot>;
