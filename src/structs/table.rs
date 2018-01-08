extern crate regex;

use self::regex::Captures;
use self::regex::Regex;
use structs::traits::*;
use structs::field::*;

#[derive(Debug, Clone)]
pub struct Table {
    pub name: String,
    pub fields: Vec<Field>,
}

impl Table {
    pub fn insert_struct(&self) -> String {
        let fields = self.diesel_struct_fields_for_new();

        format!(r"
use super::schema::{table_name};
#[derive(Insertable)]
#[table_name={table_name_q}]
pub struct New{camel_table_name}<'a> {{
{struct_fields}
}}",
                table_name = self.clone().name.clone(),
                table_name_q = format!("\"{}\"", self.clone().name.clone()),
                camel_table_name = self.clone().name.camel_case(),
                struct_fields = fields
        )
    }

    fn diesel_struct_fields_for_new(&self) -> String {
        self.fields.iter()
            .filter(|f| !f.is_auto_incremant())
            .fold("".to_string(), (|fields_string, field| {
            fields_string.to_string() + &field.diesel_struct_field_for_new()
        }))
    }
}

impl Table {
    pub fn diesel_struct(&self) -> String {
        let table_name = self.clone().name.camel_case();
        let fields = self.diesel_struct_fields();

        format!(r"
#[derive(Queryable)]
pub struct {table_name} {{
{fields}
}}",
                table_name = table_name,
                fields = fields
        )
    }

    fn diesel_struct_fields(&self) -> String {
        self.fields.iter().fold("".to_string(), (|fields_string, field| {
            fields_string.to_string() + &field.diesel_struct_field()
        }))
    }
}

impl Table {
    pub fn diesel_schema(&self) -> String {
        let field_schemas = self.diesel_schema_fields();
        let primary_field = self.fields.iter()
            .find(|field| field.is_primary())
            .unwrap_or(self.fields.get(0).expect("must on field"));


        format!(r"
table!{{
    {table_name}({primary_field_name}) {{
{fields}
    }}
}}
",
                table_name = self.name,
                fields = field_schemas,
                primary_field_name = primary_field.name
        )
    }

    fn diesel_schema_fields(&self) -> String {
        self.fields.iter().fold("".to_string(), (|fields_string, field| {
            fields_string.to_string() + &field.clone().diesel_schema_field()
        }))
    }
}

trait TranceFormCamelCase {
    fn camel_case(self) -> String;
}

impl TranceFormCamelCase for String {
    fn camel_case(self) -> String {
        let re = Regex::new(r"(^|_)(?P<c>.)").unwrap();
        re.replace_all(&self, |caps: &Captures| {
            format!("{}", &caps["c"].to_uppercase())
        }).to_string()
    }
}


impl TransformStructDef for Field {
    fn struct_field_name(&self) -> String {
        self.name.clone()
    }

    fn formatted_type(&self) -> String {
        self.type_extract()
    }
    fn allow_null(&self) -> bool {
        self.allow_null()
    }
}

impl TransformSchemaDef for Field {
    fn schema_field_name(&self) -> String {
        self.name.clone()
    }

    fn formatted_type(&self) -> String {
        self.type_extract()
    }
    fn allow_null(&self) -> bool {
        self.allow_null()
    }
}
