use constants::*;
use std::collections::HashMap;

pub trait TransformStructDef {
    fn struct_field_name(&self) -> String;
    fn formatted_type(&self) -> String;
    fn allow_null(&self) -> bool;

    fn struct_type_mapping(key: String) -> Option<String> {
        [(types::BIGINT.to_string(), types::diesel_struct::BIGINT),
            (types::DOUBLE.to_string(), types::diesel_struct::DOUBLE),
            (types::VARCHAR.to_string(), types::diesel_struct::VARCHAR),
            (types::TEXT.to_string(), types::diesel_struct::TEXT),
            (types::INT.to_string(), types::diesel_struct::INTEGER),
            (types::TINYINT.to_string(), types::diesel_struct::TINYINT),
            (types::DECIMAL.to_string(), types::diesel_struct::FLOAT),
            (types::DATETIME.to_string(), types::diesel_struct::DATETIME),
            (types::TIMESTAMP.to_string(), types::diesel_struct::TIMESTAMP)
        ].iter().cloned().collect::<HashMap<_, _>>()
            .get(&key)
            .map(|str_ref| str_ref.to_string())
    }

    fn diesel_struct_type(&self) -> String {
        let diesel_type = Self::struct_type_mapping(self.formatted_type());
        match diesel_type {
            Some(t) => t.to_string(),
            _ => panic!(format!("{}", self.formatted_type()))
        }
    }

    fn diesel_struct_field_for_new(&self) -> String {
        let type__ = if
            self.allow_null() { format!("Option<&'a {}>", self.diesel_struct_type()) }
            else { format!("&'a {}", self.diesel_struct_type()) };

        format!("    pub {name}: {type_},\r",
                name = self.struct_field_name(),
                type_ = type__)
    }

    fn diesel_struct_field(&self) -> String {
        let type__ = if
            self.allow_null() { format!("Option<{}>", self.diesel_struct_type()) }
            else { self.diesel_struct_type() };

        format!("    pub {name}: {type_},\r",
                name = self.struct_field_name(),
                type_ = type__)
    }
}


pub trait TransformSchemaDef {
    fn schema_field_name(&self) -> String;
    fn formatted_type(&self) -> String;
    fn allow_null(&self) -> bool;

    fn schema_type_mapping(key: String) -> Option<String> {
        [(types::BIGINT.to_string(), types::diesel_schema::BIGINT),
            (types::DOUBLE.to_string(), types::diesel_schema::DOUBLE),
            (types::VARCHAR.to_string(), types::diesel_schema::VARCHAR),
            (types::TEXT.to_string(), types::diesel_schema::TEXT),
            (types::INT.to_string(), types::diesel_schema::INTEGER),
            (types::TINYINT.to_string(), types::diesel_schema::TINYINT),
            (types::DECIMAL.to_string(), types::diesel_schema::FLOAT),
            (types::DATETIME.to_string(), types::diesel_schema::DATETIME),
            (types::TIMESTAMP.to_string(), types::diesel_schema::TIMESTAMP)
        ].iter().cloned().collect::<HashMap<_, _>>()
            .get(&key)
            .map(|str_ref| str_ref.to_string())
    }


    fn diesel_schema_type(&self) -> String {
        let diesel_type = Self::schema_type_mapping(self.formatted_type());
        match diesel_type {
            Some(t) => t.to_string(),
            _ => panic!(format!("{}", self.formatted_type()))
        }
    }
    fn diesel_schema_field(&self) -> String {
        let type__ = if
            self.allow_null() { format!("Nullable<{}>", self.diesel_schema_type()) }
            else { self.diesel_schema_type() };

        format!("      {name} -> {type_},\r",
                name = self.schema_field_name(),
                type_ = type__)
    }
}
