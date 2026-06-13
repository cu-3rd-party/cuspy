use crate::rest::models::parse_uuid;
use serde::{Deserialize, Serialize};
use sqlx::any::AnyRow;
use sqlx::{Error, FromRow, Row};
use std::fmt::Display;
use std::str::FromStr;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Deserialize, Serialize, ToSchema)]
pub enum AcademicLevel {
    Bachelor,
    Master,
}

impl FromStr for AcademicLevel {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "bachelor" => Ok(AcademicLevel::Bachelor),
            "master" => Ok(AcademicLevel::Master),
            _ => Err("AcademicLevel not found".to_string()),
        }
    }
}

impl Display for AcademicLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            AcademicLevel::Bachelor => "bachelor".to_string(),
            AcademicLevel::Master => "master".to_string(),
        };
        write!(f, "{}", str)
    }
}

#[derive(Deserialize, Serialize, ToSchema)]
pub enum BachelorTrack {
    SWE,
    AI,
    BA,
}

impl FromStr for BachelorTrack {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "swe" => Ok(BachelorTrack::SWE),
            "ai" => Ok(BachelorTrack::AI),
            "ba" => Ok(BachelorTrack::BA),
            _ => Err("BachelorTrack not found".to_string()),
        }
    }
}

impl Display for BachelorTrack {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            BachelorTrack::SWE => "swe".to_string(),
            BachelorTrack::AI => "ai".to_string(),
            BachelorTrack::BA => "ba".to_string(),
        };
        write!(f, "{}", str)
    }
}

#[derive(Deserialize, Serialize, ToSchema, Default)]
pub struct AgentData {
    pub agent_data_id: Uuid,
    pub codename: Option<String>,
    pub academic_group: Option<String>,
    pub academic_level: Option<AcademicLevel>,
    pub course_number: Option<i64>,
    pub bachelor_track: Option<BachelorTrack>,
    pub identification_name: Option<String>,
    pub identification_image_id: Option<Uuid>,
    pub physical_contact_allowed: bool,
    pub hugs_close_proximity_allowed: bool,
}

impl<'r> FromRow<'r, AnyRow> for AgentData {
    fn from_row(row: &'r AnyRow) -> Result<Self, Error> {
        Ok(Self {
            agent_data_id: parse_uuid(row, "agent_data_id")?,
            codename: row.try_get("codename").ok(),
            academic_group: row.try_get("academic_group").ok(),
            academic_level: row
                .try_get("academic_level")
                .ok()
                .and_then(|r| AcademicLevel::from_str(r).ok()),
            course_number: row.try_get("course_number").ok(),
            bachelor_track: row
                .try_get("bachelor_track")
                .ok()
                .and_then(|r| BachelorTrack::from_str(r).ok()),
            identification_name: row.try_get("identification_name").ok(),
            identification_image_id: parse_uuid(row, "identification_image_id").ok(),
            physical_contact_allowed: row.get("physical_contact_allowed"),
            hugs_close_proximity_allowed: row.get("hugs_close_proximity_allowed"),
        })
    }
}

#[derive(Deserialize, Serialize, ToSchema)]
pub struct AgentDataMetadata {
    pub codename: Option<String>,
    pub academic_group: Option<String>,
    pub academic_level: Option<AcademicLevel>,
    pub course_number: Option<i64>,
    pub bachelor_track: Option<BachelorTrack>,
    pub identification_name: Option<String>,
    pub physical_contact_allowed: bool,
    pub hugs_close_proximity_allowed: bool,
}
