// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() {
//         assert_eq!(2 + 2, 4);
//     }
// }
extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(WebController)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_hello_macro(&ast)
}

fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl WebController for #name {
            fn new() {
                println!("Hello, Macro! NEWNEWNEW {}!", stringify!(#name));
            }
            fn new_init(&mut self) {
                let req = self.request;
                let server_context = self.server_context;

                // info!("TEST TEST {:?}", self.tmp_str);
                //         self.tmp_str = String::from("ads");
                // info!("TEST TEST {:?}", self.tmp_str);
                let (req_parts, req_body) = req.into_parts();
                // req: Request<Body>;
                // server_context: Arc<ServerContext>;
                // let log = DBLogObject {
                //     request_id: 1234,
                //     payment_id: 123456,
                //     stage: stage.into(),
                //     log_type: "".into(),
                //     microtime_bgn: 0,
                //     microtime_end: 0,
                //     result: 0,
                //     http_code: 200,
                //     send_data: body_str.into(), //format!("{:?}", full_body),
                //     send_headers: format!("{:?}", req_parts),
                //     receive_data: "".into(),
                //     receive_headers: "".into(),
                // };
                // log_db!(log, server_context.db_pool);

                println!("Hello, Macro! NEWNEWNEW {}!", stringify!(#name));
            }
            fn hello_macro() {
                println!("Hello, Macro! My name is {}!", stringify!(#name));
            }
        }
    };
    gen.into()
}
