use proc_macro::TokenStream;
use quote::{ToTokens, quote};
use syn::spanned::Spanned;
use syn::{Expr, ItemFn, ReturnType, Stmt, StmtMacro, token::Semi};

fn signature_output_as_result(ast: &ItemFn) -> Result<ReturnType, syn::Error> {
    let output = match &ast.sig.output {
        ReturnType::Default => {
            quote! {
                -> Result<(), String>
            }
        }
        ReturnType::Type(_, ty) => {
            if ty.to_token_stream().to_string().contains("Result") {
                return Err(syn::Error::new(
                    ast.sig.span(),
                    format!(
                        "this macro can only be applied to a function that does not return a Result. Signature {}",
                        quote!(#ty)
                    ),
                ));
            }
            quote! {
                -> Result<#ty, String>
            }
        }
    };
    Ok(syn::parse2(output).unwrap())
}

fn last_statement_as_result(last_statement: Option<Stmt>) -> Stmt {
    let last_unwrapped = last_statement.unwrap();
    let last_modified = quote! {
        Ok(#last_unwrapped)
    };

    Stmt::Expr(syn::parse2(last_modified).unwrap(), None)
}

fn extract_panic_content(expr_macro: &StmtMacro) -> Option<proc_macro2::TokenStream> {
    let does_panic = expr_macro
        .mac
        .path
        .segments
        .iter()
        .any(|v| v.ident.to_string().eq("panic"));
    if does_panic {
        Some(expr_macro.mac.tokens.clone())
    } else {
        None
    }
}

fn handle_expression(expression: Expr, token: Option<Semi>) -> Result<Stmt, syn::Error> {
    match expression {
        Expr::If(mut ex_if) => {
            let new_statements: Result<Vec<Stmt>, syn::Error> = ex_if
                .then_branch
                .stmts
                .into_iter()
                .map(|s| match s {
                    Stmt::Macro(ref expr_macro) => {
                        let output = extract_panic_content(expr_macro);

                        if output.map(|v| v.is_empty()).unwrap_or(false) {
                            Err(syn::Error::new(
                                expr_macro.span(),
                                format!("please make sure every paning in your function has a message, check: {}", quote!(#s))
                            ))
                        } else {
                            Ok(extract_panic_content(expr_macro)
                                .map(|t| {
                                    quote! {
                                        return Err(#t.to_string());
                                    }
                                })
                                .map(syn::parse2)
                                .map(Result::unwrap)
                                .unwrap_or(s))
                        }
                    }
                    _ => Ok(s),
                })
                .collect();
            ex_if.then_branch.stmts = new_statements?;
            Ok(Stmt::Expr(Expr::If(ex_if), token))
        }
        _ => Ok(Stmt::Expr(expression, token)),
    }
}
#[proc_macro_attribute]
pub fn panic_to_result(_a: TokenStream, item: TokenStream) -> TokenStream {
    let mut ast: ItemFn = syn::parse(item).unwrap();
    let signature_output = signature_output_as_result(&ast);
    let statements_output: Result<Vec<Stmt>, syn::Error> = ast
        .block
        .stmts
        .into_iter()
        .map(|s| match s {
            Stmt::Expr(e, t) => handle_expression(e, t),
            _ => Ok(s),
        })
        .collect();

    match (statements_output, signature_output) {
        (Ok(new), Ok(output)) => {
            ast.sig.output = output;
            ast.block.stmts = new;
        },
        (Ok(_), Err(err)) => return err.to_compile_error().into(),
        (Err(err), Ok(_)) => return err.to_compile_error().into(),
        (Err(mut statement_err), Err(signature_err)) => {
            statement_err.combine(signature_err);
            return statement_err.to_compile_error().into();
        },
    };
    let last_statement = ast.block.stmts.pop();
    ast.block
        .stmts
        .push(last_statement_as_result(last_statement));
    ast.to_token_stream().into()
}
