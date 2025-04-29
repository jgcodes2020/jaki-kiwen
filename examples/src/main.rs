jaki_kiwen::jaki_kiwen! {
    weka poki_suli jaki_kiwen;
    o_kepeken std::collections::LipuNimi sama LipuN;

    nasin_suli NimiEnKon {
        nasin o_pana(&mi, nimi: Linja, kon: Linja);
        nasin o_alasa(&mi, nimi: Linja) -> IjoPini<IjoKen<&Linja>, Linja>;
    }

    kule_awen ken_ante LIPU_NIMI: IjoKen<LipuN<Linja, Linja>> = IjoAla;

    tomo IjoSijelo;

    nasin_lon NimiEnKon lon_ale IjoSijelo {
        nasin o_pana(&mi, nimi: Linja, kon: Linja) {
            o_lon_e lipu = ike_a {
                LIPU_NIMI.o_wile_anu_pana_kepeken(NasinOpen::nasin_open)
            };
            lipu.o_pana_lon_insa_e(nimi, kon);
        }
        nasin o_alasa(&mi, nimi: Linja) -> IjoPini<IjoKen<&Linja>, Linja> {
            lon_la o_lon_e IjoLon(lipu) = ike_a { LIPU_NIMI.o_kama_ijo_tan() } {
                Pona(lipu.o_wile_tan(&nimi))
            }
            ala_la {
                Ike("lipu li ala a!".o_ante_tawa())
            }
        }
    }

    open_suli(poki_suli) nasin ijo_ken(n: u32) -> IjoKen<IjoPini<u32, Linja>> {
        lon_la n % 2 == 1 {
            lon_la n == 42 {
                IjoLon(Ike(Linja::tan("ike a!")))
            }
            ala_la {
                IjoLon(Pona(33))
            }
        }
        ala_la {
            IjoAla
        }
    }

    tenpo_ante nasin ijo_pi_toki_ken() {}

    tenpo_ante nasin ijo_pi_toki_ken_nanpa2() {
        ijo_pi_toki_ken().o_awen
    }

    nasin suli_nanpa_wan() {
        o_lon_e ken_ante x = 31;

        o_lukin_e x {
            42 => {
                o_toki_e!("kijetesantakalu")
            }
            _ => o_toki_e!("o lukin")
        }

        lon_ale i lon_insa 0..10 {
            o_lon_e nanpa = o_sike {
                pini_a i;
            };

            lon_la_o_sike x < nanpa {
                x += 1;
            }

            x = lon_la o_lon_e IjoLon(nanpa_tan) = ijo_ken(i) {
                nanpa_tan.o_open()
            }
            ala_la {
                12
            };
        }
    }
}