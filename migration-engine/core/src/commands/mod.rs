mod apply_migration;
mod calculate_database_steps;
mod calculate_datamodel;
mod command;
mod generate_imperative_migration;
mod infer_migration_steps;
mod list_migrations;
mod migration_progress;
mod push_schema;
mod reset;
mod unapply_migration;
mod up;

pub use apply_migration::*;
pub use calculate_database_steps::*;
pub use calculate_datamodel::*;
pub use command::*;
pub use generate_imperative_migration::*;
pub use infer_migration_steps::*;
pub use list_migrations::*;
pub use migration_progress::*;
pub use push_schema::{PushSchemaCommand, PushSchemaInput, PushSchemaOutput};
pub use reset::*;
pub use unapply_migration::*;
pub use up::*;

use migration_connector::{
    MigrationError, MigrationStep, MigrationWarning, PrettyDatabaseMigrationStep, UnexecutableMigration,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MigrationStepsResultOutput {
    pub datamodel: String,
    pub datamodel_steps: Vec<MigrationStep>,
    pub database_steps: Vec<PrettyDatabaseMigrationStep>,
    pub warnings: Vec<MigrationWarning>,
    pub errors: Vec<MigrationError>,
    pub general_errors: Vec<String>,
    #[serde(skip)]
    pub unexecutable_migrations: Vec<UnexecutableMigration>,
}
