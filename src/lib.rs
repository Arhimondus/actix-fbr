#![feature(proc_macro_quote)]

extern crate proc_macro;
use std::fs::File;
use std::io::prelude::*;
use proc_macro2::{Span, Ident};
use quote::quote;

enum Handler {
	Get(String),
	Post(String),
	Delete(String),
	Put(String),
}

#[proc_macro]
pub fn services(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
	let path_string = if let proc_macro::TokenTree::Literal(lit) = item.into_iter().next().unwrap() { lit.to_string() } else { panic!() };
	let path = &path_string[1..path_string.len() - 1];
	let _module_name = Ident::new(&path[path.find("/").map(|it| it + 1).unwrap_or(0)..], Span::call_site());

	let files = std::fs::read_dir(path).unwrap().map(|it| {
		it.unwrap().path()
	}).filter(|it| it.file_name().unwrap() != "mod").collect::<Vec<_>>();

	let mut handlers: Vec<Handler> = vec![];

	for file_path in files.into_iter() {
		let mut file = File::open(&file_path).unwrap();
		let file_name = file_path.file_stem().unwrap().to_string_lossy().to_string();
		let mut contents = String::new();
		file.read_to_string(&mut contents).unwrap();
		if contents.contains("async fn get") {
			handlers.push(Handler::Get(file_name))
		} else if contents.contains("async fn post") {
			handlers.push(Handler::Post(file_name))
		} else if contents.contains("async fn delete") {
			handlers.push(Handler::Delete(file_name))
		} else if contents.contains("async fn put") {
			handlers.push(Handler::Put(file_name))
		}
	}

	let idents = handlers.into_iter().map(|it| {
		match it {
			Handler::Get(s) => {
				let handler = Ident::new(&s, Span::call_site());
				let path = format!("/{}", s);
				quote!(.route(#path, web::get().to(#handler::get)))
			},
			Handler::Post(s) => {
				let handler = Ident::new(&s, Span::call_site());
				let path = format!("/{}", s);
				quote!(.route(#path, web::post().to(#handler::post)))
			},
			Handler::Delete(s) => {
				let handler = Ident::new(&s, Span::call_site());
				let path = format!("/{}", s);
				quote!(.route(#path, web::delete().to(#handler::delete)))
			},
			Handler::Put(s) => {
				let handler = Ident::new(&s, Span::call_site());
				let path = format!("/{}", s);
				quote!(.route(#path, web::put().to(#handler::put)))
			},
		}
	}).collect::<Vec<_>>();

	proc_macro::TokenStream::from(quote!(
		web::scope("")
			#(#idents)*
	))
}

// Derived from https://github.com/Awpteamoose/supermod
#[proc_macro]
pub fn routes(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
	let path_string = if let proc_macro::TokenTree::Literal(lit) = item.into_iter().next().unwrap() { lit.to_string() } else { panic!() };
	let path = &path_string[1..path_string.len() - 1];
	let module_name = Ident::new(&path[path.find("/").map(|it| it + 1).unwrap_or(0)..], Span::call_site());

	let idents = std::fs::read_dir(path).unwrap().map(|it| {
		let path = it.unwrap().path();
		let name = path.file_stem().unwrap().to_string_lossy();
		Ident::new(&name, Span::call_site())
	}).filter(|it| it.to_string() != "mod").collect::<Vec<_>>();

	proc_macro::TokenStream::from(quote!(
		mod #module_name { #(pub mod #idents;)* }
		pub use #module_name::{
			#(#idents,)*
		};
	))
}