use proc_macro::{Group, Ident, TokenStream, TokenTree};

fn replace_ident(ident: Ident) -> Option<TokenTree> {
    let ident_str = ident.to_string();

    let new_str = match ident_str.as_str() {
        "Olana" => "Err",
        "Mety" => "Ok",
        "Fatorana" => "String", // idk
        "Diksionera" => "HashMap",
        "Mahazatra" => "Default",
        "Lisitra" => "Vec",
        "ampiana" => "push",
        "Fahadisoana" => "Error",
        "Arakaraka" => "Option",
        "Vata" => "Box",
        "Misy" => "Some",
        "Tsisy" => "None",
        "Valiny" => "Result",
        "Tena" => "Self",
        "manoratra" => "println",
        "ajanona" => "break",
        "asinika" => "async", // kill me
        "miandry" => "await",
        "averina" => "loop",
        "afindra" => "move",
        "baoritra" => "crate",
        "fahadisoana" => "error",
        "toy" => "as",
        "avela" => "const",
        "mampitahy" => "match",
        "mampatahotra" => "unsafe",
        "anaty" => "in",
        "avy" => "from",
        "dinamika" => "dyn",
        "sokafana" => "unwrap",
        "mahazatra" => "default",
        "fv" => "io", // guess
        "ivelany" => "extern",
        "diso" => "false",
        "lefa" => "fn",
        "ray" => "super",
        "fampiharana" => "impl",
        "mampiditra" => "insert",
        "fitsipika" => "trait",
        "maka" => "get",
        "joro" => "mod",
        "miova" => "mut",
        "vaovao" => "new",
        "any" => "where",
        "hoan_ny" => "for",
        "maka_na_manatsofoka" => "get_or_insert_with",
        "fotony" => "main",
        "daholobe" => "pub",
        "fahamarinana" => "bool",
        "tsisy？" | "tsisy?" => None?,
        "mamerina" => "return",
        "raha" => "if",
        "tsia" => "else",
        "tena" => "self",
        "atao" => "let",
        "statika" => "static", // seriously?
        "rafitra" => "struct",
        "manantena" => "expect",
        "rehefa" => "while",
        "mampiasa" => "use",
        "marina" => "true",
        "fitanisana" => "enum",
        "avadika" => "into",
        "atao_azo" => "to_owned",
        "raha_misy" => "is_some",
        "zavatra" => "todo",

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
pub fn arafesina(item: TokenStream) -> TokenStream {
    let mut returned = Vec::new();
    replace_stream(item, &mut returned);
    let mut out = TokenStream::new();
    out.extend(returned);
    out
}
