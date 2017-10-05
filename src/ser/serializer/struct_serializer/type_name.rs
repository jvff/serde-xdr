use std::fmt::{Display, Formatter, Result};

pub enum TypeName {
    Struct(&'static str),
    StructVariant(&'static str, &'static str)
}

impl Display for TypeName {
    fn fmt(&self, formatter: &mut Formatter) -> Result {
        match *self {
            TypeName::Struct(name) => formatter.write_str(name),
            TypeName::StructVariant(type_name, variant_name) => {
                write!(formatter, "{}::{}", type_name, variant_name)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::TypeName;

    #[test]
    fn struct_name() {
        assert_eq!(TypeName::Struct("name").to_string(), "name".to_string());
    }

    #[test]
    fn tuple_variant() {
        assert_eq!(
            TypeName::StructVariant("Type", "Variant").to_string(),
            "Type::Variant".to_string()
        );
    }
}
