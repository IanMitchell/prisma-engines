mod mssql;
mod test_api;

use crate::mssql::*;
use barrel::{types, Migration};
use pretty_assertions::assert_eq;
use sql_schema_describer::*;
use test_api::*;
use test_macros::*;

#[tokio::test]
async fn all_mssql_column_types_must_work() {
    let db_name = "all_mssql_column_types_must_work";

    let mut migration = Migration::new().schema(db_name);
    migration.create_table("User", move |t| {
        t.add_column("primary_col", types::primary());
        t.add_column("bit_col", types::custom("bit"));
        t.add_column("decimal_col", types::custom("decimal"));
        t.add_column("int_col", types::custom("int"));
        t.add_column("money_col", types::custom("money"));
        t.add_column("numeric_col", types::custom("numeric"));
        t.add_column("smallint_col", types::custom("smallint"));
        t.add_column("smallmoney_col", types::custom("smallmoney"));
        t.add_column("tinyint_col", types::custom("tinyint"));
        t.add_column("float_col", types::custom("float(24)"));
        t.add_column("double_col", types::custom("float(53)"));
        t.add_column("date_col", types::custom("date"));
        t.add_column("datetime2_col", types::custom("datetime2"));
        t.add_column("datetime_col", types::custom("datetime"));
        t.add_column("datetimeoffset_col", types::custom("datetimeoffset"));
        t.add_column("smalldatetime_col", types::custom("smalldatetime"));
        t.add_column("time_col", types::custom("time"));
        t.add_column("char_col", types::custom("char(255)"));
        t.add_column("varchar_col", types::custom("varchar(255)"));
        t.add_column("varchar_max_col", types::custom("varchar(max)"));
        t.add_column("text_col", types::custom("text"));
        t.add_column("nvarchar_col", types::custom("nvarchar(255)"));
        t.add_column("nvarchar_max_col", types::custom("nvarchar(max)"));
        t.add_column("ntext_col", types::custom("ntext"));
        t.add_column("binary_col", types::custom("binary(20)"));
        t.add_column("varbinary_col", types::custom("varbinary(20)"));
        t.add_column("varbinary_max_col", types::custom("varbinary(max)"));
        t.add_column("image_col", types::custom("image"));
    });

    let full_sql = migration.make::<barrel::backend::MsSql>();
    let inspector = get_mssql_describer_for_schema(&full_sql, db_name).await;
    let result = inspector.describe(db_name).await.expect("describing");
    let mut table = result.get_table("User").expect("couldn't get User table").to_owned();
    // Ensure columns are sorted as expected when comparing
    table.columns.sort_unstable_by_key(|c| c.name.to_owned());
    let mut expected_columns = vec![
        Column {
            name: "primary_col".to_string(),
            tpe: ColumnType {
                data_type: "int".to_string(),
                full_data_type: "int".to_string(),
                character_maximum_length: None,
                family: ColumnTypeFamily::Int,
                arity: ColumnArity::Required,
            },

            default: None,
            auto_increment: true,
        },
        Column {
            name: "bit_col".to_string(),
            tpe: ColumnType {
                data_type: "bit".to_string(),
                full_data_type: "bit".to_string(),
                character_maximum_length: None,
                family: ColumnTypeFamily::Boolean,
                arity: ColumnArity::Required,
            },

            default: None,
            auto_increment: false,
        },
        Column {
            name: "decimal_col".to_string(),
            tpe: ColumnType {
                data_type: "decimal".to_string(),
                full_data_type: "decimal".to_string(),
                character_maximum_length: None,
                family: ColumnTypeFamily::Float,
                arity: ColumnArity::Required,
            },

            default: None,
            auto_increment: false,
        },
        Column {
            name: "int_col".to_string(),
            tpe: ColumnType {
                data_type: "int".to_string(),
                full_data_type: "int".to_string(),
                character_maximum_length: None,
                family: ColumnTypeFamily::Int,
                arity: ColumnArity::Required,
            },
            default: None,
            auto_increment: false,
        },
        Column {
            name: "money_col".to_string(),
            tpe: ColumnType {
                data_type: "money".to_string(),
                full_data_type: "money".to_string(),
                character_maximum_length: None,
                family: ColumnTypeFamily::Float,
                arity: ColumnArity::Required,
            },

            default: None,
            auto_increment: false,
        },
        Column {
            name: "numeric_col".to_string(),
            tpe: ColumnType {
                data_type: "numeric".to_string(),
                full_data_type: "numeric".to_string(),
                character_maximum_length: None,
                family: ColumnTypeFamily::Float,
                arity: ColumnArity::Required,
            },

            default: None,
            auto_increment: false,
        },
        Column {
            name: "smallint_col".to_string(),
            tpe: ColumnType {
                data_type: "smallint".to_string(),
                full_data_type: "smallint".to_string(),
                character_maximum_length: None,
                family: ColumnTypeFamily::Int,
                arity: ColumnArity::Required,
            },

            default: None,
            auto_increment: false,
        },
        Column {
            name: "smallmoney_col".to_string(),
            tpe: ColumnType {
                data_type: "smallmoney".to_string(),
                full_data_type: "smallmoney".to_string(),
                character_maximum_length: None,
                family: ColumnTypeFamily::Float,
                arity: ColumnArity::Required,
            },

            default: None,
            auto_increment: false,
        },
        Column {
            name: "numeric_col".to_string(),
            tpe: ColumnType {
                data_type: "numeric".to_string(),
                full_data_type: "numeric".to_string(),
                character_maximum_length: None,
                family: ColumnTypeFamily::Float,
                arity: ColumnArity::Required,
            },

            default: None,
            auto_increment: false,
        },
        Column {
            name: "smallint_col".to_string(),
            tpe: ColumnType {
                data_type: "smallint".to_string(),
                full_data_type: "smallint".to_string(),
                character_maximum_length: None,
                family: ColumnTypeFamily::Int,
                arity: ColumnArity::Required,
            },

            default: None,
            auto_increment: false,
        },
        Column {
            name: "tinyint_col".to_string(),
            tpe: ColumnType {
                data_type: "tinyint".to_string(),
                full_data_type: "tinyint".to_string(),
                character_maximum_length: None,
                family: ColumnTypeFamily::Int,
                arity: ColumnArity::Required,
            },

            default: None,
            auto_increment: false,
        },
        Column {
            name: "float_col".to_string(),
            tpe: ColumnType {
                data_type: "float(24)".to_string(),
                full_data_type: "float(24)".to_string(),
                character_maximum_length: None,
                family: ColumnTypeFamily::Float,
                arity: ColumnArity::Required,
            },

            default: None,
            auto_increment: false,
        },
        Column {
            name: "double_col".to_string(),
            tpe: ColumnType {
                data_type: "float(53)".to_string(),
                full_data_type: "float(53)".to_string(),
                character_maximum_length: None,
                family: ColumnTypeFamily::Float,
                arity: ColumnArity::Required,
            },

            default: None,
            auto_increment: false,
        },
        Column {
            name: "date_col".to_string(),
            tpe: ColumnType {
                data_type: "date".to_string(),
                full_data_type: "date".to_string(),
                character_maximum_length: None,
                family: ColumnTypeFamily::DateTime,
                arity: ColumnArity::Required,
            },

            default: None,
            auto_increment: false,
        },
        Column {
            name: "datetime2_col".to_string(),
            tpe: ColumnType {
                data_type: "datetime2".to_string(),
                full_data_type: "datetime2".to_string(),
                character_maximum_length: None,
                family: ColumnTypeFamily::DateTime,
                arity: ColumnArity::Required,
            },

            default: Some(DefaultValue::NOW),
            auto_increment: false,
        },
        Column {
            name: "datetimeoffset_col".to_string(),
            tpe: ColumnType {
                data_type: "datetimeoffset".to_string(),
                full_data_type: "datetimeoffset".to_string(),
                character_maximum_length: None,
                family: ColumnTypeFamily::DateTime,
                arity: ColumnArity::Required,
            },

            default: None,
            auto_increment: false,
        },
        Column {
            name: "smalldatetime_col".to_string(),
            tpe: ColumnType {
                data_type: "smalldatetime".to_string(),
                full_data_type: "smalldatetime".to_string(),
                character_maximum_length: None,
                family: ColumnTypeFamily::DateTime,
                arity: ColumnArity::Required,
            },

            default: None,
            auto_increment: false,
        },
        Column {
            name: "time_col".to_string(),
            tpe: ColumnType {
                data_type: "time".to_string(),
                full_data_type: "time".to_string(),
                character_maximum_length: None,
                family: ColumnTypeFamily::DateTime,
                arity: ColumnArity::Required,
            },

            default: None,
            auto_increment: false,
        },
        Column {
            name: "char_col".to_string(),
            tpe: ColumnType {
                data_type: "char".to_string(),
                full_data_type: "char".to_string(),
                character_maximum_length: Some(255),
                family: ColumnTypeFamily::String,
                arity: ColumnArity::Required,
            },

            default: None,
            auto_increment: false,
        },
        Column {
            name: "varchar_col".to_string(),
            tpe: ColumnType {
                data_type: "varchar".to_string(),
                full_data_type: "varchar".to_string(),
                character_maximum_length: Some(255),
                family: ColumnTypeFamily::String,
                arity: ColumnArity::Required,
            },

            default: None,
            auto_increment: false,
        },
        Column {
            name: "varchar_max_col".to_string(),
            tpe: ColumnType {
                data_type: "varchar".to_string(),
                full_data_type: "varchar".to_string(),
                character_maximum_length: Some(-1),
                family: ColumnTypeFamily::String,
                arity: ColumnArity::Required,
            },

            default: None,
            auto_increment: false,
        },
        Column {
            name: "text_col".to_string(),
            tpe: ColumnType {
                data_type: "text".to_string(),
                full_data_type: "text".to_string(),
                character_maximum_length: None,
                family: ColumnTypeFamily::String,
                arity: ColumnArity::Required,
            },

            default: None,
            auto_increment: false,
        },
        Column {
            name: "nvarchar_col".to_string(),
            tpe: ColumnType {
                data_type: "nvarchar".to_string(),
                full_data_type: "nvarchar".to_string(),
                character_maximum_length: Some(255),
                family: ColumnTypeFamily::String,
                arity: ColumnArity::Required,
            },

            default: None,
            auto_increment: false,
        },
        Column {
            name: "nvarchar_max_col".to_string(),
            tpe: ColumnType {
                data_type: "nvarchar".to_string(),
                full_data_type: "nvarchar".to_string(),
                character_maximum_length: Some(-1),
                family: ColumnTypeFamily::String,
                arity: ColumnArity::Required,
            },

            default: None,
            auto_increment: false,
        },
        Column {
            name: "ntext_col".to_string(),
            tpe: ColumnType {
                data_type: "ntext".to_string(),
                full_data_type: "ntext".to_string(),
                character_maximum_length: None,
                family: ColumnTypeFamily::Binary,
                arity: ColumnArity::Required,
            },

            default: None,
            auto_increment: false,
        },
        Column {
            name: "binary_col".to_string(),
            tpe: ColumnType {
                data_type: "binary".to_string(),
                full_data_type: "binary".to_string(),
                character_maximum_length: Some(20),
                family: ColumnTypeFamily::Binary,
                arity: ColumnArity::Required,
            },

            default: None,
            auto_increment: false,
        },
        Column {
            name: "varbinary_col".to_string(),
            tpe: ColumnType {
                data_type: "varbinary".to_string(),
                full_data_type: "varbinary".to_string(),
                character_maximum_length: Some(20),
                family: ColumnTypeFamily::Binary,
                arity: ColumnArity::Required,
            },

            default: None,
            auto_increment: false,
        },
        Column {
            name: "varbinary_max_col".to_string(),
            tpe: ColumnType {
                data_type: "varbinary".to_string(),
                full_data_type: "varbinary".to_string(),
                character_maximum_length: Some(-1),

                family: ColumnTypeFamily::Binary,
                arity: ColumnArity::Required,
            },

            default: None,
            auto_increment: false,
        },
        Column {
            name: "image_col".to_string(),
            tpe: ColumnType {
                data_type: "image".to_string(),
                full_data_type: "image".to_string(),
                character_maximum_length: None,
                family: ColumnTypeFamily::Binary,
                arity: ColumnArity::Required,
            },

            default: None,
            auto_increment: false,
        },
    ];
    expected_columns.sort_unstable_by_key(|c| c.name.to_owned());

    assert_eq!(
        table,
        Table {
            name: "User".to_string(),
            columns: expected_columns,
            indices: vec![],
            primary_key: Some(PrimaryKey {
                columns: vec!["primary_col".to_string()],
                sequence: None,
                constraint_name: None,
            }),
            foreign_keys: vec![],
        }
    );
}
