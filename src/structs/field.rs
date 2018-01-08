extern crate regex;
use self::regex::Regex;

#[derive(Debug, Clone)]
pub struct Field {
    pub name: String,
    pub type_: String,
    pub null_: String,
    pub key_: Option<String>,
    pub default: Option<Vec<u8>>,
    pub extra: Option<String>,
}


impl Field {
    pub fn type_extract(&self) -> String {
        let re = Regex::new(r"\(.*\)").unwrap();
        re.replace_all(&self.type_, "").to_string()
    }

    pub fn allow_null(&self) -> bool {
        "YES" == self.null_
    }

    pub fn is_primary(&self) -> bool {
        self.key_ == Some("PRI".to_string())
    }

    pub fn is_auto_incremant(&self) -> bool {
        self.extra == Some("auto_increment".to_string())
    }
}
