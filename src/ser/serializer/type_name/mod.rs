pub enum TypeName {
    Sequence,
    Tuple,
    TupleStruct(&'static str),
}

impl ToString for TypeName {
    fn to_string(&self) -> String {
        match *self {
            TypeName::Sequence => "sequence".to_string(),
            TypeName::Tuple => "tuple".to_string(),
            TypeName::TupleStruct(name) => format!("tuple struct {}", name),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::TypeName;

    #[test]
    fn sequence() {
        assert_eq!(TypeName::Sequence.to_string(), "sequence".to_string());
    }

    #[test]
    fn tuple() {
        assert_eq!(TypeName::Tuple.to_string(), "tuple".to_string());
    }

    #[test]
    fn tuple_struct() {
        assert_eq!(
            TypeName::TupleStruct("name").to_string(),
            "tuple struct name".to_string()
        );
    }
}
