use graphql_parser::query::Definition::Operation;
use graphql_parser::query::{parse_query, OperationDefinition};
use log::{debug, info, trace, warn};
use serde::{Deserialize, Serialize};
use serde_json::value::Value;
use serde_json::Result;
use std::collections::HashMap;

pub fn log_post(content_type: Option<mime::Mime>, body: &String) {
    let application_graphql: mime::Mime = "application/graphql".parse().unwrap();
    trace!("log_post({:?}, {:?})", &content_type, &body);
    let n = match content_type {
        Some(ref mime_type) if &application_graphql == mime_type => count_top_level_fields(body),
        Some(ref mime_type) if mime_type == &mime::APPLICATION_JSON => {
            match count_json_query(body) {
                Ok(count) => count,
                Err(err) => {
                    warn!("{:?}", err);
                    0
                }
            }
        }
        Some(mime_type) => {
            warn!("Don't know how to handle {}!", &mime_type);
            0
        }
        None => {
            warn!("No content-type specified, will try to parse as application/graphql");
            count_top_level_fields(body)
        }
    };
    info!("Found {} fields/queries", n);
}

#[derive(Debug, Serialize, Deserialize)]
struct GraphQLJSONQuery {
    query: String,
    operation_name: Option<String>,
    variables: Option<HashMap<String, Value>>,
}

fn count_json_query(body: &str) -> Result<usize> {
    trace!("count_json_query({})", &body);
    let q: GraphQLJSONQuery = serde_json::from_str(body)?;
    trace!("{:?}", &q);
    trace!("{}", &q.query);
    Ok(count_top_level_fields(q.query.as_str()))
}

fn count_top_level_fields(query: &str) -> usize {
    trace!("count_top_level_fields({:?})", &query);
    let mut n: usize = 0;
    if let Ok(document) = parse_query(&query) {
        trace!("document => {:?}", &document);
        for def in document.definitions.iter() {
            match def {
                Operation(OperationDefinition::Query(query)) => {
                    if let Some(query_name) = &query.name {
                        debug!("query.name => {}", query_name);
                    }
                    let count = query.selection_set.items.iter().count();
                    n = n + count;
                }
                Operation(OperationDefinition::SelectionSet(selection_set)) => {
                    let count = selection_set.items.iter().count();
                    n = n + count;
                }
                _ => warn!("{:?}", def),
            }
        }
    };
    return n;
}
