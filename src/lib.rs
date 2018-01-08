extern crate mysql;

pub mod constants;
pub mod structs;

use structs::field::Field;

pub fn get_table_names(pool: mysql::Pool) -> Result<Vec<String>, mysql::Error> {
    pool.prep_exec("show tables", ())
        .map(|query_result| {
            query_result
                .map(|row_result| {
                    row_result
                        .map(|row| {
                            mysql::from_row::<String>(row)
                        }).unwrap()
                }).collect::<Vec<_>>()
        })
}

pub fn get_fields_by_table_name(table_name: String, pool: mysql::Pool) -> Result<Vec<Field>, mysql::Error> {
    pool.prep_exec(format!("desc {}", &table_name), ()).map(|query_result| {
        query_result.map(|row_result| {
            row_result
                .map(|row| {
                    let (field, type_, null_, key_, default, extra) = mysql::from_row(row);
                    Field {
                        name: field,
                        type_: type_,
                        null_: null_,
                        key_: key_,
                        default: default,
                        extra: extra,
                    }
                }).unwrap()
        }).collect::<Vec<_>>()
    })
}