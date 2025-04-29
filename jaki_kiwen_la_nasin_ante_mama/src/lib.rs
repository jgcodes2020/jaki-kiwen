use proc_macro::{Group, Ident, TokenStream, TokenTree};

fn replace_ident(ident: Ident) -> Option<TokenTree> {
    let ident_str = ident.to_string();

    let new_str = match ident_str.as_str() {
        "Pak" => "Err",
        "Pona" => "Ok",
        "Linja" => "String",
        "LipuNimi" => "HashMap",
        "NasinOpen" => "Default",
        "Pakala" => "Error",
        "IjoKen" => "Option",
        "IjoLon" => "Some",
        "IjoAla" => "None",
        "IjoPini" => "Result",
        "Mi" => "Self",
        "o_toki_e" => "println",
        "pini_a" => "break",
        "tenpo_ante" => "async",
        "o_awen" => "await",
        "sike" => "loop",
        "tawa_a" => "move",
        "poki_suli" => "crate",
        "ken_ala_tawa_toki_ni" => "unreachable_code",
        "sama" => "as",
        "ijo_awen" => "const",
        "nasin_suli" => "trait",
        "ike_a" => "unsafe",
        "lon_insa" => "in",
        "tan_poki" => "from",
        "ken_ijo_ante" => "dyn",
        "o_open" => "unwrap",
        "nasin_open" => "default",
        "o_kama_ijo_tan" => "as_ref",
        "tw" => "io", // toki/wile
        "weka" => "extern",
        "lon_ala" => "false",
        "nasin" => "fn",
        "mama" => "super",
        "pana_lon" => "insert",
        "o_wile_e_ijo" => "get",
        "o_ken" => "allow",
        "pakala_suli" => "panic",
        "poki" => "mod",
        "ken_ante" => "mut",
        "o_sin" => "new",
        "sama_ni_" => "where",
        "lon_ale" => "for",
        "wile_anu_pana_kepeken" => "get_or_insert_with",
        "suli_nanpa_wan" => "main",
        "open_suli" => "pub",
        "o_pana_tawa_mama_e" => "return",
        "nasin_lon" => "impl",
        "ijo_tan" => "ref",
        "o_lukin_e" => "match",
        "lon_la" => "if",
        "ala_la" => "else",
        "mi" => "self",
        "o_lon_e" => "let",
        "kule_awen" => "static",
        "tomo" => "struct",
        "o_wile_e_ken" => "expect",
        "ni_li_lon_la" => "while",
        "o_kepeken" => "use",
        "tawa_insa" => "into",
        "lon" => "true",
        "linja_kiwen_ijo" => "enum",
        "Kulupu" => "Group",
        "NimiIjo" => "Ident",
        "LipuPiNimiIlo" => "TokenStream",
        "KasiPiNimiIlo" => "TokenTree",
        "o_kama_linja" => "to_string",
        "o_linja" => "as_str",
        "ma_poki" => "span",
        "LinjaIjo" => "Vec",
        "o_lipu" => "stream",
        "o_pana_e" => "push",
        "o_suli_kepeken" => "extend",
        "nimi_pi_weka_nimi" => "delimiter",
        "NimiInsa" => "Punct",
        "NimiKiwen" => "Literal",
        "nasin_ante_ilo" => "proc_macro",
        _ => &ident_str,
    };

    let new_ident = Ident::new(new_str, ident.span());
    Some(TokenTree::Ident(new_ident))
}

fn replace_tree(tok: TokenTree, out: &mut Vec<TokenTree>) {
    match tok {
        TokenTree::Group(group) => {
            let mut group_elem = Vec::new();
            replace_stream(group.stream(), &mut group_elem);
            let mut new_stream = TokenStream::new();
            new_stream.extend(group_elem);
            out.push(TokenTree::Group(Group::new(group.delimiter(), new_stream)));
        }
        TokenTree::Ident(ident) => {
            if let Some(ident) = replace_ident(ident) {
                out.push(ident);
            }
        }
        TokenTree::Punct(..) | TokenTree::Literal(..) => {
            out.push(tok);
        }
    }
}

fn replace_stream(ts: TokenStream, out: &mut Vec<TokenTree>) {
    for tok in ts {
        replace_tree(tok, out)
    }
}

#[proc_macro]
pub fn jaki_kiwen(item: TokenStream) -> TokenStream {
    let mut returned = Vec::new();
    replace_stream(item, &mut returned);
    let mut out = TokenStream::new();
    out.extend(returned);
    out
}
