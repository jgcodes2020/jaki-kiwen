jkl_nasin_mama::jaki_kiwen! {
    o_kepeken nasin_ante_ilo::{Kulupu, NimiIjo, LipuPiNimiIlo, KasiPiNimiIlo};

    nasin o_ante_e_nimi_ijo(nimi: NimiIjo) -> IjoKen<KasiPiNimiIlo> {
        o_lon_e linja_open = nimi.to_string();
        o_lon_e linja_sin = o_lukin_e linja_open.o_linja() {
            "Ike" => "Err",
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
            "o_sike" => "loop",
            "tawa_a" => "move",
            "poki_suli" => "crate",
            "ken_ala_tawa_toki_ni" => "unreachable_code",
            "sama" => "as",
            "ijo_awen" => "const",
            "nasin_suli" => "trait",
            "ike_a" => "unsafe",
            "lon_insa" => "in",
            "tan" => "from",
            "ken_ijo_ante" => "dyn",
            "o_open" => "unwrap",
            "nasin_open" => "default",
            "o_kama_ijo_tan" => "as_ref",
            "tw" => "io", // toki/wile
            "weka" => "extern",
            "lon_ala" => "false",
            "nasin" => "fn",
            "mama" => "super",
            "o_pana_lon_insa_e" => "insert",
            "o_wile_tan" => "get",
            "o_ken" => "allow",
            "pakala_suli" => "panic",
            "poki" => "mod",
            "ken_ante" => "mut",
            "o_sin" => "new",
            "sama_ni_" => "where",
            "lon_ale" => "for",
            "o_wile_anu_pana_kepeken" => "get_or_insert_with",
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
            "lon_la_o_sike" => "while",
            "o_kepeken" => "use",
            "o_ante_tawa" => "into",
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
            _ => &linja_open,
        };
        // owe a
        o_lon_e nimi_sin = NimiIjo::o_sin(linja_sin, nimi.span());
        IjoLon(KasiPiNimiIlo::NimiIjo(nimi_sin))
    }

    nasin o_ante_e_kasi(kasi: KasiPiNimiIlo, pini: &ken_ante LinjaIjo<KasiPiNimiIlo>) {
        match kasi {
            KasiPiNimiIlo::Kulupu(kulupu) => {
                o_lon_e ken_ante ijo_kulupu = LinjaIjo::new();
                o_ante_e_lipu(kulupu.stream(), &ken_ante ijo_kulupu);
                o_lon_e ken_ante lipu_sin = LipuPiNimiIlo::new();
                lipu_sin.o_suli_kepeken(ijo_kulupu);
                pini.o_pana_e(KasiPiNimiIlo::Kulupu(Kulupu::new(kulupu.delimiter(), lipu_sin)));
            }
            KasiPiNimiIlo::NimiIjo(nimi) => {
                lon_la o_lon_e IjoLon(nimi) = o_ante_e_nimi_ijo(nimi) {
                    pini.o_pana_e(nimi);
                }
            }
            KasiPiNimiIlo::NimiInsa(..) | KasiPiNimiIlo::NimiKiwen(..) => {
                pini.o_pana_e(kasi);
            }
        }
    }

    nasin o_ante_e_lipu(lipu: LipuPiNimiIlo, pini: &ken_ante LinjaIjo<KasiPiNimiIlo>) {
        lon_ale nimi lon_insa lipu {
            o_ante_e_kasi(nimi, pini)
        }
    }

    #[nasin_ante_ilo]
    pub fn jaki_kiwen(ijo: TokenStream) -> TokenStream {
        o_lon_e ken_ante linja_pana = Vec::new();
        o_ante_e_lipu(ijo, &ken_ante linja_pana);
        o_lon_e ken_ante pini = TokenStream::new();
        pini.o_suli_kepeken(linja_pana);
        pini
    }

}