pub enum TypeName {
    Sequence,
    Tuple,
    TupleStruct(&'static str),
    TupleVariant(&'static str, &'static str)
}

impl ToString for TypeName {
    fn to_string(&self) -> String {
        match *self {
            TypeName::Sequence => "sequence".to_string(),
            TypeName::Tuple => "tuple".to_string(),
            TypeName::TupleStruct(name) => format!("tuple struct {}", name),
            TypeName::TupleVariant(tuple_name, variant_name) => {
                format!("tuple variant {}::{}", tuple_name, variant_name)
            },
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

    #[test]
    fn tuple_variant() {
        assert_eq!(
            TypeName::TupleVariant("Tuple", "Variant").to_string(),
            "tuple variant Tuple::Variant".to_string()
        );
    }
}
