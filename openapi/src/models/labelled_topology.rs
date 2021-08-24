#![allow(
    clippy::too_many_arguments,
    clippy::new_without_default,
    non_camel_case_types,
    unused_imports
)]
/*
 * Mayastor RESTful API
 *
 * The version of the OpenAPI document: v0
 *
 * Generated by: https://github.com/openebs/openapi-generator
 */

use crate::apis::IntoVec;

/// LabelledTopology : volume topology using labels

/// volume topology using labels
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct LabelledTopology {
    #[serde(rename = "node_topology")]
    pub node_topology: crate::models::NodeTopology,
    #[serde(rename = "pool_topology")]
    pub pool_topology: crate::models::PoolTopology,
}

impl LabelledTopology {
    /// LabelledTopology using only the required fields
    pub fn new(
        node_topology: impl Into<crate::models::NodeTopology>,
        pool_topology: impl Into<crate::models::PoolTopology>,
    ) -> LabelledTopology {
        LabelledTopology {
            node_topology: node_topology.into(),
            pool_topology: pool_topology.into(),
        }
    }
    /// LabelledTopology using all fields
    pub fn new_all(
        node_topology: impl Into<crate::models::NodeTopology>,
        pool_topology: impl Into<crate::models::PoolTopology>,
    ) -> LabelledTopology {
        LabelledTopology {
            node_topology: node_topology.into(),
            pool_topology: pool_topology.into(),
        }
    }
}