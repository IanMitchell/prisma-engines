use datamodel_connector::error::{ConnectorError, ErrorKind};
use datamodel_connector::{Connector, ConnectorCapability, NativeTypeConstructor, NativeTypeInstance};
use native_types::NativeType;

pub struct SqliteDatamodelConnector {
    capabilities: Vec<ConnectorCapability>,
    constructors: Vec<NativeTypeConstructor>,
}

impl SqliteDatamodelConnector {
    pub fn new() -> SqliteDatamodelConnector {
        let capabilities = vec![];
        let constructors: Vec<NativeTypeConstructor> = vec![];

        SqliteDatamodelConnector {
            capabilities,
            constructors,
        }
    }
}

impl Connector for SqliteDatamodelConnector {
    fn capabilities(&self) -> &Vec<ConnectorCapability> {
        &self.capabilities
    }

    fn available_native_type_constructors(&self) -> &Vec<NativeTypeConstructor> {
        &self.constructors
    }

    fn parse_native_type(&self, _name: &str, _args: Vec<u32>) -> Result<NativeTypeInstance, ConnectorError> {
        return Err(ConnectorError::from_kind(
            ErrorKind::ConnectorNotSupportedForNativeTypes {
                connector_name: "sqlite".to_string(),
            },
        ));
    }

    fn introspect_native_type(&self, _native_type: Box<dyn NativeType>) -> Result<NativeTypeInstance, ConnectorError> {
        return Err(ConnectorError::from_kind(
            ErrorKind::ConnectorNotSupportedForNativeTypes {
                connector_name: "sqlite".to_string(),
            },
        ));
    }
}
