use crate::TokenStream;

mod variants;
mod args;

/// A macro for implementing a trait for a list of types.
pub fn mass_impl<T: Into<TokenStream>>(cfg: T, input: T) -> TokenStream {
    let cfg: TokenStream = cfg.into();
    let input: TokenStream = input.into();
    let config = match syn::parse2::<args::MassImplMacroConfig>(cfg) {
        Ok(config) => config,
        Err(err) => {
            return err.to_compile_error();
        }
    };
    let input = input.to_string();
    let mut results = Vec::new();
    results.push(input);

    for tv in &config.type_variants {
        let mut temp_results = Vec::new();
        if tv.allow_owned {
            for r in &results {
                let new = r.replace(&tv.alias.to_string(), &tv.ty.to_string());
                temp_results.push(new);
            }
        }
        if tv.allow_ref {
            for r in &results {
                let new = r.replace(&tv.alias.to_string(), &format!("&{}", tv.ty));
                temp_results.push(new);
            }
        }
        if tv.allow_mut {
            for r in &results {
                let new = r.replace(&tv.alias.to_string(), &format!("&mut {}", tv.ty));
                temp_results.push(new);
            }
        }
        results = temp_results;
    }
    println!("{:#?}", &results);
    let single_str = results.join("\n");
    syn::parse_str::<TokenStream>(&single_str).unwrap()
}