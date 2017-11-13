use std::fmt::{Display, Formatter, Result};

pub enum TypeName {
    Sequence,
    Tuple,
    TupleStruct(&'static str),
    TupleVariant(&'static str, &'static str),
}

impl Display for TypeName {
    fn fmt(&self, formatter: &mut Formatter) -> Result {
        match *self {
            TypeName::Sequence => formatter.write_str("sequence"),
            TypeName::Tuple => formatter.write_str("tuple"),
            TypeName::TupleStruct(name) => {
                write!(formatter, "tuple struct {}", name)
            }
            TypeName::TupleVariant(tuple_name, variant) => {
                write!(formatter, "tuple variant {}::{}", tuple_name, variant)
            }
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
