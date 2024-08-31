/*
Copyright 2022 The Numaproj Authors.

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
*/

// Code generated by Openapi Generator. DO NOT EDIT.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VertexStatus {
    /// Conditions are the latest available observations of a resource's current state.
    #[serde(rename = "conditions", skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition>>,
    /// If not empty, indicates the version of the Vertex used to generate Pods in the sequence [0,currentReplicas).
    #[serde(rename = "currentHash", skip_serializing_if = "Option::is_none")]
    pub current_hash: Option<String>,
    /// The number of Pods created by the controller from the Vertex version indicated by currentHash.
    #[serde(rename = "currentReplicas", skip_serializing_if = "Option::is_none")]
    pub current_replicas: Option<i64>,
    #[serde(rename = "lastScaledAt", skip_serializing_if = "Option::is_none")]
    pub last_scaled_at: Option<k8s_openapi::apimachinery::pkg::apis::meta::v1::Time>,
    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// The generation observed by the Vertex controller.
    #[serde(rename = "observedGeneration", skip_serializing_if = "Option::is_none")]
    pub observed_generation: Option<i64>,
    #[serde(rename = "phase", skip_serializing_if = "Option::is_none")]
    pub phase: Option<String>,
    /// The number of pods targeted by this Vertex with a Ready Condition.
    #[serde(rename = "readyReplicas", skip_serializing_if = "Option::is_none")]
    pub ready_replicas: Option<i64>,
    #[serde(rename = "reason", skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    /// Total number of non-terminated pods targeted by this Vertex (their labels match the selector).
    #[serde(rename = "replicas", skip_serializing_if = "Option::is_none")]
    pub replicas: Option<i64>,
    #[serde(rename = "selector", skip_serializing_if = "Option::is_none")]
    pub selector: Option<String>,
    /// If not empty, indicates the version of the Vertx used to generate Pods in the sequence [replicas-updatedReplicas,replicas)
    #[serde(rename = "updateHash", skip_serializing_if = "Option::is_none")]
    pub update_hash: Option<String>,
    /// The number of Pods created by the controller from the Vertex version indicated by updateHash.
    #[serde(rename = "updatedReplicas", skip_serializing_if = "Option::is_none")]
    pub updated_replicas: Option<i64>,
}

impl VertexStatus {
    pub fn new() -> VertexStatus {
        VertexStatus {
            conditions: None,
            current_hash: None,
            current_replicas: None,
            last_scaled_at: None,
            message: None,
            observed_generation: None,
            phase: None,
            ready_replicas: None,
            reason: None,
            replicas: None,
            selector: None,
            update_hash: None,
            updated_replicas: None,
        }
    }
}
