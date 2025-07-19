we can create a macro like this 

macro_rules! parse_any_group {
    ($input:ident, $content:ident) => {
        if $input.peek(syn::token::Bracket) {
            let $content;
            syn::bracketed!($content in $input);
            Some(($content.parse()?, syn::Delimiter::Bracket))
        } else if $input.peek(syn::token::Paren) {
            let $content;
            syn::parenthesized!($content in $input);
            Some(($content.parse()?, syn::Delimiter::Paren))
        } else if $input.peek(syn::token::Brace) {
            let $content;
            syn::braced!($content in $input);
            Some(($content.parse()?, syn::Delimiter::Brace))
        } else {
            None
        }
    };
}