use std::{
  convert::TryFrom,
  ffi::{CStr, CString},
};

use libpg_query2::{pg_query_free_parse_result, pg_query_parse};
use litrs::StringLit;
use proc_macro::TokenStream;
use quote::quote;

#[proc_macro]
pub fn sql(input: TokenStream) -> TokenStream {
  let input = input.into_iter().collect::<Vec<_>>();
  if input.len() != 1 {
    let msg = format!(
      "expected exactly one string literal as input, got {} tokens",
      input.len()
    );
    return quote! { compile_error!(#msg) }.into();
  }
  let string_lit = match StringLit::try_from(&input[0]) {
    Ok(lit) => lit,
    Err(e) => return e.to_compile_error(),
  };

  if let Err(e) = check_sql(string_lit.value()) {
    let msg = format!("failed to parse SQL query: {}", e);
    return quote! { compile_error!(#msg) }.into();
  }

  input.into_iter().collect()
}

fn check_sql(sql: &str) -> Result<(), String> {
  let c_str = CString::new(sql).unwrap();
  let pg_parse_result = unsafe { pg_query_parse(c_str.as_ptr()) };

  let result = if pg_parse_result.error.is_null() {
    Ok(())
  }
  else if let Ok(message) = unsafe { CStr::from_ptr((*pg_parse_result.error).message) }.to_str() {
    Err(message.to_owned())
  }
  else {
    Err("parse failed, and error message wasn't valid UTF-8".to_owned())
  };

  unsafe {
    pg_query_free_parse_result(pg_parse_result);
  }

  result
}
