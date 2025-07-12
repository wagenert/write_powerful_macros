use builder_macro::Builder;

#[derive(Builder)]
struct Gleipnir {}
fn main() {}

#[cfg(test)]
mod tests {

    use builder_macro::Builder;

    #[test]
    fn should_generate_builder_for_struct_with_no_properties() {
        #[derive(Builder)]
        struct ExampleStructWithNoFields {}

        let _: ExampleStructWithNoFields = ExampleStructWithNoFields::builder().build();
    }

    #[test]
    fn should_generate_builder_for_struct_with_one_property() {
        #[derive(Builder)]
        struct Gleipnir {
            roots_of: String,
        }

        let gleipnir = Gleipnir::builder()
            .roots_of("mountains".to_string())
            .build();

        assert_eq!(gleipnir.roots_of, "mountains".to_string());
    }
}
