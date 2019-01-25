#![recursion_limit="128"]

extern crate proc_macro;
extern crate proc_macro2;
extern crate syn;
#[macro_use] extern crate quote;

#[proc_macro_derive(VertexAttribPointers, attributes(location))]
pub fn vertex_attrib_pointers_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // Parse the string representation
    let ast: syn::DeriveInput = syn::parse(input).unwrap();

    // Build the impl
    let gen = generate_impl(&ast);

    // Return the generated impl
    gen.into()
}

fn get_attr_value(attr: &syn::Attribute, name: &str) -> Option<syn::Lit> {
    let meta_attr = attr.parse_meta().ok();

    match meta_attr.unwrap() {
        syn::Meta::NameValue(meta_name_value) => {
            if format!("{}", meta_name_value.ident) == name {
                return Some(meta_name_value.lit.clone())
            }
        },
        _ => ()
    }

    None
}

fn generate_struct_field_vertex_attrib_pointer_call(field: &syn::Field) -> proc_macro2::TokenStream {
    // panic!("{:#?}", field);

    let field_name = match &field.ident {
        Some(id) => format!("{}", id),
        None => String::from(""),
    };

    let location_attr = field.attrs
        .iter()
        .filter_map(|x| {get_attr_value(x, "location")})
        .next()
        .unwrap_or_else(|| panic!("Field {:?} missing location attribute", field_name
    ));

    match location_attr {
        syn::Lit::Int(_) => (),
        _ => panic!("Location attribute must be an integer")
    };

    let field_type = &field.ty;

    quote! {
        unsafe {
            #field_type::vertex_attrib_pointer(gl, stride, #location_attr, offset);
        }
        let offset = offset + std::mem::size_of::<#field_type>();
    }
}

fn generate_vertex_attrib_pointer_calls(data: &syn::Data) -> Vec<proc_macro2::TokenStream> {
    match data {
        syn::Data::Enum(_) => panic!("VertexAttribPointers not implemented for Enums"),
        syn::Data::Union(_) => panic!("VertexAttribPointers not implemented for Unions"),

        syn::Data::Struct(data_struct) => match &data_struct.fields {
            syn::Fields::Unnamed(_) => panic!("VertexAttribPointers not implemented for tuple Structs"),
            syn::Fields::Unit => panic!("VertexAttribPointers not implemented for unit Structs"),
            syn::Fields::Named(named) => data_struct.fields.iter()
                                            .map(generate_struct_field_vertex_attrib_pointer_call)
                                            .collect()
        }
    }
}

fn generate_impl(ast: &syn::DeriveInput) -> proc_macro2::TokenStream {
    let name = &ast.ident;
    let (_, generics, where_clause) = ast.generics.split_for_impl();

    let x = generate_vertex_attrib_pointer_calls(&ast.data);

    quote! {
        impl #name #generics #where_clause {
            pub fn vertex_attrib_pointers(gl: &gl::Gl) {
                let stride = std::mem::size_of::<Self>();
                let offset = 0;

                #(#x)*
            }
        }
    }
}
