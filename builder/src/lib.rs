use proc_macro::{TokenStream, Span};
use syn::{
    self,
    DeriveInput,
    Ident,
    Data,
    DataStruct,
};
use quote::quote;

#[proc_macro_derive(Builder)]
pub fn derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_builder_macro(&ast)
}

fn impl_builder_macro(ast: &DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let builder_name = Ident::new(
        format!("{}Builder", name).as_str(),
        name.span(),
    );
    if let Data::Struct(data_struct) = &ast.data {
        let fields = data_struct.fields.iter();
        /*
        let mut field_idents_v: Vec<Ident> = Vec::new();
        fields.for_each(|f| {
            field_idents_v.push(f.ident.unwrap());
        });
        let field_idents = field_idents_v.iter();
         */
        let field_idents = fields.map(|f| {
            f.ident.unwrap()
        }).iter();

        let gen = quote! {
            pub struct #builder_name {
                #(#fields ,)*
            }
            impl #name {
                fn builder() -> #builder_name {
                    #(#field_idents : None,)
                }
            }
        };
        gen.into()
    } else {
        panic!("The type is not a struct!");
    }

}