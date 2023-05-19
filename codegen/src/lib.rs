mod test_stand;

#[proc_macro_derive(TestStand, attributes(database))]
pub fn test_stand(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    crate::test_stand::derive_test_stand(input)
}
