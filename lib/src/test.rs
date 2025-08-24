#[macro_export]
macro_rules! create_test {
    ($name:ident, $input:literal, $output:literal) => {
        #[test]
        fn $name() {
            let my_data = $input;
            let mut my_reader = crate::Scanner::new(std::io::Cursor::new(my_data));
            let mut my_writer: Vec<u8> = Vec::new();
            solve(&mut my_reader, &mut my_writer);

            let my_out = String::from_utf8(my_writer).unwrap();
            assert_eq!(my_out, $output);
        }
    };
}
