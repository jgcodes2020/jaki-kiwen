# jaki kiwen

![](https://github.com/jgcodes2020/jaki-kiwen/raw/suli-nanpa-wan/logo.png)

Aren't you _pilin lape_ from writing Rust programs in English? Do you like saying
"pakala a" a lot? Would you like to try something different, in a super simplified
language? Would you want to bring some Toki Pona touch to your programs?

**jaki kiwen** (Toki Pona for _Rust_) is here to save your day, as it allows you to
write Rust programs in Toki Pona, using Toki Pona keywords, Toki Pona function names,
Toki Pona idioms.

This has been designed to be used as the official programming language for 
*ma pona pi toki pona*.

You're a *jan sin* and don't feel at ease using only Toki Pona words? 

Don't worry!
Toki Pona Rust is fully compatible with English Rust, so you can mix both at your
convenience.

Here's an example of what can be achieved with *jaki kiwen*:

### trait and impl (aka nasin_suli en nasin_lon)

```rust
jaki_kiwen::jaki_kiwen! {
    extern poki_suli jaki_kiwen;
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
}
```

### Other examples

See the [examples](./examples/src/main.rs) to get a rough sense of the whole
syntax. *o lukin a*, that's it.

## les contributions

First of all, _sina pona mute_ for considering participating to this joke,
Sonja Lang will thank you later! Feel free to throw in a few identifiers
here and there, and open a pull request against the `suli-nanpa-wan` (Toki
Pona for `main`) branch.

Please don't introduce any *unpa*, though: we will not excuse your Toki Pona.

## but why would you do zat

- horsin around
- playing with raw proc macros
- making a bit of fun about programming languages that do this seriously,
  though I can see their utility.
- ni li sama nimi kijetesantakalu

## Other languages

- French: [rouille](https://github.com/jeroenhd/roest)
- Dutch: [roest](https://github.com/jeroenhd/roest)
- German: [rost](https://github.com/michidk/rost)
- Polish: [rdza](https://github.com/phaux/rdza)
- Italian: [ruggine](https://github.com/DamianX/ruggine)
- Russian: [Ржавый](https://github.com/Sanceilaks/rzhavchina)
- Esperanto: [rustteksto](https://github.com/dscottboggs/rustteksto)
- Hindi: [zung](https://github.com/rishit-khandelwal/zung)
- Hungarian: [rozsda](https://github.com/jozsefsallai/rozsda)
- Chinese: [xiu (锈)](https://github.com/lucifer1004/xiu)
- Spanish: [rustico](https://github.com/UltiRequiem/rustico)
- Korean: [Nok (녹)](https://github.com/Alfex4936/nok)
- Finnish: [ruoste](https://github.com/vkoskiv/ruoste)
- Arabic: [sada](https://github.com/LAYGATOR/sada)
- Turkish: [pas](https://github.com/ekimb/pas)
- Vietnamese: [gỉ](https://github.com/Huy-Ngo/gir)
- Japanese: [sabi (錆)](https://github.com/yuk1ty/sabi)
- Danish: [rust?](https://github.com/LunaTheFoxgirl/rust-dk)
- Marathi: [gan̄ja](https://github.com/pranavgade20/ganja)
- Romanian: [rugină](https://github.com/aionescu/rugina)
- Czech: [rez](https://github.com/radekvit/rez)
- Ukrainian: [irzha](https://github.com/brokeyourbike/irzha)
- Bulgarian: [ryzhda](https://github.com/gavadinov/ryzhda)
- Slovak: [hrdza](https://github.com/TheMessik/hrdza)
- Catalan: [rovell](https://github.com/gborobio73/rovell)
- Corsican: [rughjina](https://github.com/aldebaranzbradaradjan/rughjina)
- Indonesian: [karat](https://github.com/annurdien/karat)
- Lithuanian: [rūdys](https://github.com/TruncatedDinosour/rudys)
- Greek: [skouriasmeno](https://github.com/devlocalhost/skouriasmeno)
- Thai: [sanim (สนิม)](https://github.com/korewaChino/sanim)
- Swiss: [roeschti](https://github.com/Georg-code/roeschti)
- Swedish: [rost](https://github.com/vojd/rost/)
- Croatian: [hrđa](https://github.com/njelich/hrdja)
- Persian: [zangar (زنگار)](https://github.com/ui-ce/zangar)
- Malagasy: [arafesina](https://github.com/luckasRanarison/arafesina)
- Latin: [ferrugo](https://github.com/pianoman911/ferrugo)
- Norwegian: [korrosjon](https://github.com/datagutt/korrosjon)
- Estonian: [rooste](https://github.com/hanshs/rooste)
- All of the above: [unirust](https://github.com/charyan/unirust)

## un grand merci

- [@VentGrey](https://twitter.com/VentGrey) for making a logo!

## la license

[License Publique Rien à Branler](http://sam.zoy.org/lprab/),
_le_ official translation of the [WTFPL](http://www.wtfpl.net/)
by the same author.
