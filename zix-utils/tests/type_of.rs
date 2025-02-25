
#[cfg(test)]
mod tests {
    use zix_utils::type_of;

    #[test]
    fn test_primitive_types() {
        let a = 42;
        assert_eq!(type_of(&a), "i32");

        let b = 3.14;
        assert_eq!(type_of(&b), "f64");

        let c = true;
        assert_eq!(type_of(&c), "bool");

        let d = 'R';
        assert_eq!(type_of(&d), "char");
    }

    #[test]
    fn test_string_types() {
        let s = "hello";
        assert_eq!(type_of(&s), "&str");

        let string = String::from("Rust");
        assert_eq!(type_of(&string), "alloc::string::String");
    }

    #[test]
    fn test_collections() {
        let vec = vec![1, 2, 3];
        assert_eq!(type_of(&vec), "alloc::vec::Vec<i32>");

        let arr = [1, 2, 3];
        assert_eq!(type_of(&arr), "[i32; 3]");

        let tuple = (1, "hello", 3.14);
        assert_eq!(type_of(&tuple), "(i32, &str, f64)");
    }

    #[test]
    fn test_references() {
        let x = 10;
        let y = &x;
        assert_eq!(type_of(&y), "&i32");

        let z = &mut { let mut a = 5; a };
        assert_eq!(type_of(&z), "&mut i32");
    }

    #[test]
    fn test_option_and_result() {
        let opt: Option<i32> = Some(5);
        assert_eq!(type_of(&opt), "core::option::Option<i32>");

        let res: Result<i32, &str> = Ok(10);
        assert_eq!(type_of(&res), "core::result::Result<i32, &str>");
    }

    #[test]
    fn test_function_type() {
        fn sample_fn(x: i32) -> i32 { x * 2 }
        assert_eq!(type_of(&sample_fn), "type_of::tests::test_function_type::sample_fn");
    }

    #[test]
    fn test_generic_struct() {
        struct Wrapper<T> {
            value: T,
        }

        let w = Wrapper { value: 10 };
        println!("{}", type_of(&w));
        assert_eq!(type_of(&w), "type_of::tests::test_generic_struct::Wrapper<i32>");
    }
}
