pub enum TypeName {
    Struct(&'static str),
    StructVariant(&'static str, &'static str)
}

impl ToString for TypeName {
    fn to_string(&self) -> String {
        match *self {
            TypeName::Struct(name) => name.to_string(),
            TypeName::StructVariant(type_name, variant_name) => {
                format!("{}::{}", type_name, variant_name)
            },
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
