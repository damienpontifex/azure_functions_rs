use serde::Deserialize;

/// Provides access to timer schedule information
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct TimerInfo {
    /// Associated data for this configured timer
    pub data: Data,
    /// Timer associated metadata
    pub metadata: Metadata,
}

/// Timer data
#[derive(Debug, Deserialize)]
pub struct Data {
    /// Timer status data
    pub timer: Timer,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Timer {
    /// The schedule for the timer trigger
    pub schedule: Schedule,
    /// The current status for this timer. If schedule monitoring is not enabled for this timer, this propertly will be `None`
    pub schedule_status: Option<serde_json::Value>,
    /// Indication whether this timer invocation is due to a missed schedule occurence
    pub is_past_due: bool,
}

/// Timer schedule
#[derive(Debug, Deserialize)]
pub struct Schedule {
    /// Indicates whether intervals between invocations should account for DST.
    #[serde(rename = "AdjustForDST")]
    pub adjust_for_dst: bool,
}

/// Timer invocation metadata
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Metadata {
    /// Function invocation system properties
    #[serde(rename = "sys")]
    pub sys: crate::queue_trigger::Sys,
}
