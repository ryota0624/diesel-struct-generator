pub mod types {
    pub const VARCHAR: &str = "varchar";
    pub const TEXT: &str = "text";
    pub const DATETIME: &str = "datetime";
    pub const TIMESTAMP: &str = "timestamp";
    pub const TINYINT: &str = "tinyint";
    pub const BIGINT: &str = "bigint";
    pub const INT: &str = "int";
    pub const DECIMAL: &str = "decimal";
    pub const DOUBLE: &str = "double";

    pub mod diesel_struct {
        pub const BIGINT: &str = "i64";
        pub const DOUBLE: &str = "f64";
        pub const FLOAT: &str = "f32";
        pub const INTEGER: &str = "i32";
        pub const TINYINT: &str = "i8";
        pub const TIMESTAMP: &str = "chrono::NaiveDateTime";
        pub const DATETIME: &str = "chrono::NaiveDateTime";
        pub const TEXT: &str = "String";
        pub const VARCHAR: &str = "String";
    }

    pub mod diesel_schema {
        pub const BIGINT: &str = "BigInt";
        pub const DOUBLE: &str = "Double";
        pub const FLOAT: &str = "Float";
        pub const INTEGER: &str = "Integer";
        pub const TINYINT: &str = "Tinyint";
        pub const TIMESTAMP: &str = "Timestamp";
        pub const DATETIME: &str = "Datetime";
        pub const TEXT: &str = "Text";
        pub const VARCHAR: &str = "Text";
    }
}
