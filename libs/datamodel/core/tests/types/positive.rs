use crate::common::*;
use datamodel::{dml::ScalarType, DefaultValue, ValueGenerator};
use native_types::{MySqlType, PostgresType};
use prisma_value::PrismaValue;

#[test]
fn should_apply_a_custom_type() {
    let dml = r#"
    type ID = String @id @default(cuid())

    model Model {
        id ID
    }
    "#;

    let datamodel = parse(dml);
    let user_model = datamodel.assert_has_model("Model");
    user_model
        .assert_has_scalar_field("id")
        .assert_is_id()
        .assert_base_type(&ScalarType::String)
        .assert_default_value(DefaultValue::Expression(ValueGenerator::new_cuid()));
}

#[test]
fn should_recursively_apply_a_custom_type() {
    let dml = r#"
        type MyString = String
        type MyStringWithDefault = MyString @default(cuid())
        type ID = MyStringWithDefault @id

        model Model {
            id ID
        }
    "#;

    let datamodel = parse(dml);
    let user_model = datamodel.assert_has_model("Model");
    user_model
        .assert_has_scalar_field("id")
        .assert_is_id()
        .assert_base_type(&ScalarType::String)
        .assert_default_value(DefaultValue::Expression(ValueGenerator::new_cuid()));
}

#[test]
fn should_be_able_to_handle_multiple_types() {
    let dml = r#"
    type ID = String @id @default(cuid())
    type UniqueString = String @unique
    type Cash = Int @default(0)

    model User {
        id       ID
        email    UniqueString
        balance  Cash
    }
    "#;

    let datamodel = parse(dml);
    let user_model = datamodel.assert_has_model("User");
    user_model
        .assert_has_scalar_field("id")
        .assert_is_id()
        .assert_base_type(&ScalarType::String)
        .assert_default_value(DefaultValue::Expression(ValueGenerator::new_cuid()));

    user_model
        .assert_has_scalar_field("email")
        .assert_is_unique(true)
        .assert_base_type(&ScalarType::String);

    user_model
        .assert_has_scalar_field("balance")
        .assert_base_type(&ScalarType::Int)
        .assert_default_value(DefaultValue::Single(PrismaValue::Int(0)));
}

#[test]
fn should_be_able_to_define_custom_enum_types() {
    let dml = r#"
    type RoleWithDefault = Role @default(USER)

    model User {
        id Int @id
        role RoleWithDefault
    }

    enum Role {
        ADMIN
        USER
        CEO
    }
    "#;

    let datamodel = parse(dml);

    let user_model = datamodel.assert_has_model("User");

    user_model
        .assert_has_scalar_field("role")
        .assert_enum_type("Role")
        .assert_default_value(DefaultValue::Single(PrismaValue::Enum(String::from("USER"))));
}

#[test]
fn should_handle_type_specifications_on_postgres() {
    let dml = r#"
        datasource pg {
          provider = "postgres"
          url = "postgresql://"
          previewFeatures = ["nativeTypes"]
        }

        model Blog {
            id     Int    @id
            bigInt Int    @pg.BigInt
            foobar String @pg.VarChar(26)
        }
    "#;

    let datamodel = parse(dml);

    let user_model = datamodel.assert_has_model("Blog");

    let sft = user_model.assert_has_scalar_field("bigInt").assert_native_type();

    let postgres_type: PostgresType = sft.deserialize_native_type();
    assert_eq!(postgres_type, PostgresType::BigInt);

    let sft = user_model.assert_has_scalar_field("foobar").assert_native_type();

    let postgres_type: PostgresType = sft.deserialize_native_type();
    assert_eq!(postgres_type, PostgresType::VarChar(26));
}

#[test]
fn should_handle_type_specifications_on_mysql() {
    let dml = r#"
        datasource mys {
          provider = "mysql"
          url = "mysql://"
          previewFeatures = ["nativeTypes"]
        }

        model Blog {
            id       Int      @id
            smallInt Int      @mys.SmallInt
            foobar   DateTime @mys.Datetime(26)
        }
    "#;

    let datamodel = parse(dml);

    let user_model = datamodel.assert_has_model("Blog");

    let sft = user_model.assert_has_scalar_field("smallInt").assert_native_type();

    let mysql_type: MySqlType = sft.deserialize_native_type();
    assert_eq!(mysql_type, MySqlType::SmallInt);

    let sft = user_model.assert_has_scalar_field("foobar").assert_native_type();

    let mysql_type: MySqlType = sft.deserialize_native_type();
    assert_eq!(mysql_type, MySqlType::DateTime(Option::from(26)));
}
