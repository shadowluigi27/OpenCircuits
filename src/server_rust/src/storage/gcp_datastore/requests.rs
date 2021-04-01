use std::collections::HashMap;
use std::default::Default;
use std::iter::FromIterator;

use google_datastore1::{
    CommitRequest, Entity, Filter, Key, KindExpression, LookupRequest, Mutation, PathElement,
    Projection, PropertyFilter, PropertyReference, Query, RunQueryRequest, Value,
};

pub fn lookup_req<I: Iterator<Item = String>>(kind: &str, ids: I) -> LookupRequest {
    LookupRequest {
        keys: Some(Vec::from_iter(ids.map(|id| Key {
            path: Some(vec![PathElement {
                kind: Some(String::from(kind)),
                id: Some(id),
                name: None,
            }]),
            ..Key::default()
        }))),
        ..LookupRequest::default()
    }
}

pub fn query_req(kind: &str, prop: &str, val: &str, proj: Option<Vec<&str>>) -> RunQueryRequest {
    let props = proj.map(|props| {
        Vec::from_iter(props.iter().map(|p| Projection {
            property: Some(PropertyReference {
                name: Some(String::from(*p)),
            }),
        }))
    });
    RunQueryRequest {
        query: Some(Query {
            filter: Some(Filter {
                property_filter: Some(PropertyFilter {
                    property: Some(PropertyReference {
                        name: Some(String::from(prop)),
                    }),
                    value: Some(Value {
                        string_value: Some(String::from(val)),
                        ..Value::default()
                    }),
                    op: Some(String::from("EQUAL")),
                }),
                ..Filter::default()
            }),
            kind: Some(vec![KindExpression {
                name: Some(String::from(kind)),
            }]),
            projection: props,
            ..Query::default()
        }),
        ..RunQueryRequest::default()
    }
}

pub fn put_req(
    kind: &str,
    id: Option<String>,
    transaction: String,
    v: HashMap<String, Value>,
) -> CommitRequest {
    CommitRequest {
        transaction: Some(transaction),
        mutations: Some(vec![Mutation {
            upsert: Some(Entity {
                properties: Some(v),
                key: Some(Key {
                    path: Some(vec![PathElement {
                        kind: Some(String::from(kind)),
                        id: id,
                        name: None,
                    }]),
                    ..Key::default()
                }),
            }),
            ..Mutation::default()
        }]),
        ..CommitRequest::default()
    }
}

pub fn del_req(kind: &str, transaction: String, id: String) -> CommitRequest {
    CommitRequest {
        transaction: Some(transaction),
        mutations: Some(vec![Mutation {
            delete: Some(Key {
                path: Some(vec![PathElement {
                    kind: Some(String::from(kind)),
                    id: Some(id),
                    name: None,
                }]),
                ..Key::default()
            }),
            ..Mutation::default()
        }]),
        ..CommitRequest::default()
    }
}
