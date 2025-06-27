#![allow(unused_braces)]

use proc_macro  :: { TokenStream };
use proc_macro2 :: { TokenStream as TokenStream2 };
use syn         :: { Data, DeriveInput, Field, Fields, FieldsNamed };
use syn         :: { parse_macro_input };
use quote       :: { quote, format_ident };

//-----------------------------------------------------------------------------
//  Derive Macros
//-----------------------------------------------------------------------------

#[proc_macro_derive(GetSet, attributes(get, set, all))]
pub fn derive_get_set(input: TokenStream) -> TokenStream {

	let ast           = parse_macro_input!(input as DeriveInput);
	let fields        = get_fields(&ast);
	let struct_name   = &ast.ident;

	let generics      = &ast.generics.split_for_impl();
	let impl_generics = &generics.0;
	let type_generics = &generics.1;
	let where_clause  = &generics.2;

	let getters       = fields.named.iter().map(derive_getter_function);
	let setters       = fields.named.iter().map(derive_setter_function);
	let mutgets       = fields.named.iter().map(derive_mutget_function);

	TokenStream::from(quote! {
		#[allow(dead_code)]
		impl #impl_generics #struct_name #type_generics #where_clause {
			#(#getters)*
			#(#setters)*
			#(#mutgets)*
		}
	})
}

#[proc_macro_derive(Builder)]
pub fn derive_builder(input: TokenStream) -> TokenStream {

	let ast           = parse_macro_input!(input as DeriveInput);
	let fields        = get_fields(&ast);
	let visibility    = &ast.vis;
	let struct_name   = &ast.ident;
	let builder_name  = format_ident!("{struct_name}Builder");

	let generics      = &ast.generics.split_for_impl();
	let impl_generics = &generics.0;
	let type_generics = &generics.1;
	let where_clause  = &generics.2;

	let builders      = fields.named.iter().map(derive_builder_function);

	TokenStream::from(quote! {

		#[derive(Debug)]
		#visibility struct #builder_name(#struct_name);

		#[allow(dead_code)]
		impl #impl_generics #builder_name #type_generics #where_clause {
			pub fn build(self) -> #struct_name { self.0 }
			#(#builders)*
		}
	})
}

//-----------------------------------------------------------------------------
//  Helper Functions
//-----------------------------------------------------------------------------

fn derive_builder_function(field: &Field) -> TokenStream2 {

	let field_name = &field.ident.as_ref().unwrap();
	let field_type = &field.ty;

	quote! {
		#[inline(always)]
		pub fn #field_name(mut self, value: #field_type) -> Self {
			self.0.#field_name = value;
			self
		}
	}
}

fn derive_getter_function(field: &Field) -> TokenStream2 {

	if !has_attribute(&field, "all")
	&& !has_attribute(&field, "set")
	&& !has_attribute(&field, "get") { return quote!() }

	let field_name = &field.ident.as_ref().unwrap();
	let field_type = &field.ty;

	quote! {
		#[inline(always)]
		pub fn #field_name(&self) -> &#field_type {
			&self.#field_name
		}
	}
}

fn derive_setter_function(field: &Field) -> TokenStream2 {

	if !has_attribute(&field, "all")
	&& !has_attribute(&field, "set") { return quote!() }

	let field_name = &field.ident.as_ref().unwrap();
	let field_type = &field.ty;
	let func_name  = format_ident!("set_{field_name}");

	quote! {
		#[inline(always)]
		pub fn #func_name(&mut self, value: #field_type) -> &mut Self {
			self.#field_name = value;
			self
		}
	}
}

fn derive_mutget_function(field: &Field) -> TokenStream2 {

	if !has_attribute(&field, "all") { return quote!() }

	let field_name = &field.ident.as_ref().unwrap();
	let field_type = &field.ty;
	let func_name  = format_ident!("mut_{field_name}");

	quote! {
		#[inline(always)]
		pub fn #func_name(&mut self) -> &mut #field_type {
			&mut self.#field_name
		}
	}
}

fn get_fields(ast: &DeriveInput) -> &FieldsNamed {

	let Data::Struct(ref data) = ast.data else {
		panic!("Can't derive functions from a non-struct");
	};

	let Fields::Named(ref fields) = data.fields else {
		panic!("Can't derive functions from unnamed fields");
	};

	return fields
}

fn has_attribute(field: &Field, name: &'static str) -> bool {
	field.attrs.iter().any(|attr| attr.path().is_ident(name))
}
