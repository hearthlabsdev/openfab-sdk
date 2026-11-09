use rocket_query::search::{Filter, Condition, Operator, LogicOperator};
use serde_json;
use uuid::Uuid;
use serde::{Deserialize, Serialize};

#[cfg(feature = "schema")]
use rocket_okapi::okapi::{schemars, schemars::JsonSchema};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "fromform", derive(rocket::form::FromForm))]
struct EmptyExtras {
    void: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "fromform", derive(rocket::form::FromForm))]
pub struct DeviceExtras {
    pub driver_id: Option<String>,
    pub serial: Option<String>,
    pub status: Option<String>,
    pub queue: Option<Uuid>,
    pub library: Option<Uuid>,
}

pub type DeviceQuery = DataQuery<DeviceExtras>;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "fromform", derive(rocket::form::FromForm))]
pub struct DataQuery<E = EmptyExtras> {
    pub name: Option<String>,
    pub created_after: Option<i64>,
    pub created_before: Option<i64>,
    pub updated_after: Option<i64>,
    pub updated_before: Option<i64>,
    pub resource_types: Option<Vec<String>>,   
    
    /// Additional search filters that are specific to the type `E`.
    /// This allows for extending the base query with custom filters defined by the user.
    #[serde(flatten)]
    pub extras: Option<E>,
}

impl<E: rocket_query::search::SearchFilter> DataQuery<E> {
    pub fn new() -> Self {
        Self {
            name: None,
            created_after: None,
            created_before: None,
            updated_after: None,
            updated_before: None,
            resource_types: None,
            extras: None,
        }
    }

    fn base_filter(&self, op: LogicOperator) -> Option<Filter> {
        let mut filters = Vec::new();
        if let Some(name) = &self.name {
            filters.push(Filter::Condition(Condition {
                operator: Operator::Eq,
                key: "name".to_string(),
                value: serde_json::Value::String(name.clone()),
            }));
        }
        if let (start, end) = (&self.created_after, &self.created_before) {
            if let Some(start) = start {
                filters.push(Filter::Condition(Condition {
                    operator: Operator::Gte,
                    key: "created".to_string(),
                    value: serde_json::Value::Number((*start).into()),
                }));
            }
            if let Some(end) = end {
                filters.push(Filter::Condition(Condition {
                    operator: Operator::Lte,
                    key: "created".to_string(),
                    value: serde_json::Value::Number((*end).into()),
                }));
            }
        }
        if let (start, end) = (&self.updated_after, &self.updated_before) {
            if let Some(start) = start {
                filters.push(Filter::Condition(Condition {
                    operator: Operator::Gte,
                    key: "updated".to_string(),
                    value: serde_json::Value::Number((*start).into()),
                }));
            }
            if let Some(end) = end {
                filters.push(Filter::Condition(Condition {
                    operator: Operator::Lte,
                    key: "updated".to_string(),
                    value: serde_json::Value::Number((*end).into()),
                }));
            }
        }
        if let Some(resource_types) = &self.resource_types {
            for resource_type in resource_types {
                filters.push(Filter::Condition(Condition {
                    operator: Operator::Eq,
                    key: "resource_type".to_string(),
                    value: serde_json::Value::String(resource_type.clone()),
                }));
            }
        }
        if filters.is_empty() {
            None
        } else {
            match op {
                LogicOperator::And => {
                    Some(Filter::And(filters))
                }
                LogicOperator::Or => {
                    Some(Filter::Or(filters))
                },
                LogicOperator::Not => {
                    Some(Filter::Not(Box::new(Filter::And(filters))))
                }
            }
        }
    }
    pub fn name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }
    pub fn created(mut self, start: i64, end: i64) -> Self {
        self.created_after = Some(start);
        self.created_before = Some(end);
        self
    }
    pub fn updated(mut self, start: i64, end: i64) -> Self {
        self.updated_after = Some(start);
        self.updated_before = Some(end);
        self
    }
    pub fn created_after(mut self, timestamp: i64) -> Self {
        self.created_after = Some(timestamp);
        self
    }
    
    pub fn created_before(mut self, timestamp: i64) -> Self {
        self.created_before = Some(timestamp);
        self
    }
    pub fn updated_after(mut self, timestamp: i64) -> Self {
        self.updated_after = Some(timestamp);
        self
    }
    pub fn updated_before(mut self, timestamp: i64) -> Self {
        self.updated_before = Some(timestamp);
        self
    }
    pub fn resource_types(mut self, resource_types: Vec<String>) -> Self {
        self.resource_types = Some(resource_types);
        self
    }
    pub fn extras(mut self, extras: E) -> Self {
        self.extras = Some(extras);
        self
    }
}