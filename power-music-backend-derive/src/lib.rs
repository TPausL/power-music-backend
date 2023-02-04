extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;

#[proc_macro_derive(CanBeStored)]
pub fn test(input: TokenStream) -> TokenStream {
    // Construct a string representation of the type definition
    let s = input.to_string();

    // Parse the string representation
    let ast = syn::parse_derive_input(&s).unwrap();

    // Build the impl
    let gen = impl_test(&ast);

    // Return the generated impl
    gen.parse().unwrap()
}

fn impl_test(ast: &syn::DeriveInput) -> quote::Tokens {
    let name = &ast.ident;
    let table_name = name.as_ref().to_lowercase();
    quote! {
        #[async_trait]
        impl CanBeStored for #name {
            const TABLE_NAME: &'static str = #table_name;
            fn get_id(&self) -> String {
                return self.id.to_owned();
            }
            async fn from_db(id: String) -> surrealdb::Result<Self> {
                let db = DB.get().await;
                let res: Self = db.select((Self::TABLE_NAME, id)).await?;
                Ok(res)
            }
            async fn fetch(&mut self) -> surrealdb::Result<Self> {
                let data = &mut Self::from_db(self.id.to_owned()).await?;
                self = data;
                Ok(self.clone())
            }
            async fn store(&self) -> surrealdb::Result<Self> {
                let db = DB.get().await;
                let res = db
                    .create((Self::TABLE_NAME, self.id.to_owned()))
                    .content(self)
                    .await?;
                Ok(res)
            }
            async fn update(&self) -> surrealdb::Result<Self> {
                let db = DB.get().await;
                let res = db
                    .update((Self::TABLE_NAME, self.id.to_owned()))
                    .content(self)
                    .await?;
                Ok(res)
            }
            async fn delete(&self) -> surrealdb::Result<()> {
                let db = DB.get().await;
                db.delete((Self::TABLE_NAME, self.id.to_owned())).await?;
                Ok(())
            }

        }
    }
}
