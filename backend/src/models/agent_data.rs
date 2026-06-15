use crate::models::{db_uuid, parse_uuid, ApiError};
use serde::{Deserialize, Serialize};
use sqlx::any::AnyRow;
use sqlx::{Acquire, Any, Error, FromRow, Row};
use std::fmt::Display;
use std::str::FromStr;
use std::sync::Arc;
use s3::Bucket;
use utoipa::ToSchema;
use uuid::Uuid;
use crate::models::resource::Resource;

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

impl AgentData {
    pub async fn create<'c, A>(
        executor: A,
        metadata: AgentDataMetadata,
        resource: Option<Resource>
    ) -> Result<Self, ApiError>
    where
        A: Acquire<'c, Database = Any>
    {
        let mut executor = executor.acquire().await?;
        let mut data: Self = Self::create_from_meta(&mut *executor, metadata).await?;
        if let Some(resource) = resource {
            data.add_profile_picture(&mut *executor, resource).await?;
        }
        Ok(data)
    }
    
    async fn create_from_meta<'c, A>(
        executor: A,
        metadata: AgentDataMetadata,
    ) -> Result<Self, ApiError>
    where
        A: Acquire<'c, Database = Any>
    {
        let mut executor = executor.acquire().await?;
        Ok(sqlx::query_as(r#"
                insert into "agent_data" (codename, academic_group, academic_level, course_number, bachelor_track, identification_name, physical_contact_allowed, hugs_close_proximity_allowed)
                values ($1, $2, $3, $4, $5, $6, $7, $8)
                returning
                    cast(agent_data_id as text) as agent_data_id,
                    codename,
                    academic_group,
                    academic_level,
                    course_number,
                    bachelor_track,
                    identification_name,
                    cast(identification_image_id as text) as identification_image_id,
                    physical_contact_allowed,
                    hugs_close_proximity_allowed
                "#)
            .bind(metadata.codename)
            .bind(metadata.academic_group)
            .bind(metadata.academic_level.map(|s| s.to_string()))
            .bind(metadata.course_number)
            .bind(metadata.bachelor_track.map(|s| s.to_string()))
            .bind(metadata.identification_name)
            .bind(metadata.physical_contact_allowed)
            .bind(metadata.hugs_close_proximity_allowed)
            .fetch_one(&mut *executor)
            .await?)
    }
    
    async fn add_profile_picture<'c, A>(
        &mut self,
        executor: A,
        resource: Resource,
    ) -> Result<(), ApiError>
    where
        A: Acquire<'c, Database = Any>
    {
        let mut executor = executor.acquire().await?;
        *self = sqlx::query_as(
            r#"
            update "agent_data"
            set identification_image_id = cast($2 as uuid)
            where agent_data_id = cast($1 as uuid)
            returning
                cast(agent_data_id as text) as agent_data_id,
                codename,
                academic_group,
                academic_level,
                course_number,
                bachelor_track,
                identification_name,
                cast(identification_image_id as text) as identification_image_id,
                physical_contact_allowed,
                hugs_close_proximity_allowed
        "#,
        )
        .bind(db_uuid(self.agent_data_id))
        .bind(db_uuid(resource.id))
        .fetch_one(&mut *executor)
        .await?;

        Ok(())
    }

    pub async fn get_by_id<'c, A>(
        executor: A,
        agent_data_id: Uuid,
    ) -> Result<Self, ApiError>
    where
        A: Acquire<'c, Database = Any>
    {
        let mut executor = executor.acquire().await?;
        Ok(sqlx::query_as(
            r#"
            select
                cast(agent_data_id as text) as agent_data_id,
                codename,
                academic_group,
                academic_level,
                course_number,
                bachelor_track,
                identification_name,
                cast(identification_image_id as text) as identification_image_id,
                physical_contact_allowed,
                hugs_close_proximity_allowed
            from agent_data where agent_data_id = cast($1 as uuid)
        "#,
        )
            .bind(db_uuid(agent_data_id))
            .fetch_one(&mut *executor)
            .await?)
    }
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
