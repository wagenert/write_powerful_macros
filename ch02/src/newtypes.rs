pub struct FirstName {
    value: String,
}

pub struct LastName {
    value: String,
}

pub struct Age {
    value: i32,
}

pub struct Pay {
    pub value: i32,
}

macro_rules! generate_get_value {
    ($struct_type: ident) => {
        impl $struct_type {
            pub fn get_value(&self) -> &String {
                &self.value
            }
        }
    };
    ($struct_type: ident, $return_type:ty) => {
        impl $struct_type {
            pub fn get_value(&self) -> &$return_type {
                &self.value
            }
        }
    };
}

macro_rules! generate_convert_from {
    ($struct_type: ident) => {
        impl TryFrom<&str> for $struct_type {
            type Error = &'static str;

            fn try_from(value: &str) -> Result<Self, Self::Error> {
                Ok(Self {
                    value: String::from(value),
                })
            }
        }
    };
    ($struct_type: ident, $from_type: ty) => {
        impl TryFrom<$from_type> for $struct_type {
            type Error = &'static str;

            fn try_from(value: $from_type) -> Result<Self, Self::Error> {
                Ok(Self { value })
            }
        }
    };
}

macro_rules! generate_newtypes_methods {
    ($struct_type: ident) => {
        generate_get_value!($struct_type);
        generate_convert_from!($struct_type);
    };
    ($struct_type: ident, $from_type: ty) => {
        generate_get_value!($struct_type, $from_type);
        generate_convert_from!($struct_type, $from_type);
    };
}

generate_newtypes_methods!(FirstName);
generate_newtypes_methods!(LastName);
generate_newtypes_methods!(Age, i32);
generate_newtypes_methods!(Pay, i32);
