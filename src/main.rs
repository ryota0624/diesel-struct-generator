extern crate mysql;
extern crate rust_diesel_struct_generator;

use mysql::Opts;
use mysql::OptsBuilder;
use std::convert::From;
use std::fs::File;
use std::io::prelude::*;
use rust_diesel_struct_generator::*;
use rust_diesel_struct_generator::structs::field::Field;
use rust_diesel_struct_generator::structs::table::Table;



fn main() {
    let hostname = //;
    let user = //;
    let db_name = //;

    let mut builder = OptsBuilder::new();
    builder.ip_or_hostname(Some(hostname.to_string()))
        .user(Some(user.to_string()))
        .db_name(Some(db_name.to_string()));

    let pool = mysql::Pool::new(Opts::from(builder)).expect("fail connection");

    let table_names = get_table_names(pool.clone()).expect("must ok");

    let tables = table_names.iter().map(|table_name| {
        let table_name = table_name.to_string();
        let fields: Result<Vec<Field>, _> = get_fields_by_table_name(table_name.clone(), pool.clone());
        Table {
            name: table_name,
            fields: fields.unwrap(),
        }
    }).collect::<Vec<_>>();

    let model = tables.iter()
        .map(|table| table.diesel_struct())
        .fold("extern crate chrono;\r".to_string(), |a, b| {
            a + &b
        });

    let schema = tables.iter().map(|table| table.diesel_schema())
        .fold("".to_string(), |a, b| {
            a + &b
        });

    let create_model = tables.iter()
        .map(|table| table.insert_struct())
        .fold("extern crate chrono;\r".to_string(), |a, b| {
            a + &b
        });

    let mut model_file = File::create("models.rs").expect("fail creat file!");
    model_file.write_all(&model.into_bytes()).expect("fail write");

    let mut schema_file = File::create("schema.rs").expect("fail creat file!");
    schema_file.write_all(&schema.into_bytes()).expect("fail write");

    let mut create_model_file = File::create("create_model.rs").expect("fail creat file!");
    create_model_file.write_all(&create_model.into_bytes()).expect("fail write");

    ();
}

