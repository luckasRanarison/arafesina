# arafesina

![Logo](https://github.com/luckasRanarison/nvimrc/assets/101930730/e47f31d9-e79e-45ac-8837-86522f0f6ede)

Aren't you _reraka_ from writing Rust programs in English? Do you like saying
"letie" a lot? Would you like to try something different, in an exotic and
funny-sounding language? Would you want to bring some Malagasy touch to your
programs?

**arafesina** (Malagasy for _Rust_) is here to save your day, as it allows you to
write Rust programs in Malagasy, using Malagasy keywords, Malagasy function names,
Malagasy idioms.

This has been designed to be used as the official programming language to
develop the future Malagasy operating system.

Don't worry!
Malagasy Rust is fully compatible with English-Rust, so you can mix both at your
convenience.

Here's an example of what can be achieved with arafesina:

### Ohatra

```rust
arafesina::arafesina! { 
     ivelany baoritra arafesina; 
  
     lefa fotony() {
         manoratra!("Miarahaba ny tany!"); 
         manoratra!("{:?}", ankasa_ve(2)); 
     } 
 
     lefa ankasa_ve(n: u32) -> fahamarinana { 
         lefa ankasa(n: u32) -> fahamarinana { 
             raha n == 0 { 
                 mamerina marina; 
             } 
             tsiankasa(n - 1) 
         } 
  
         lefa tsiankasa(n: u32) -> fahamarinana { 
             raha n == 0 { 
                 mamerina diso; 
             } 
             ankasa(n - 1) 
         } 
  
         ankasa(n) 
     }
}
```

### Other examples

See the [examples](./examples/src/main.rs) to get a rough sense of the whole
syntax. Zay fotsiny.

## Contributions

First of all, _misaotra betsaka_ for considering participating to this joke, the
Malagasy government will thank you later! Feel free to throw in a few identifiers
here and there, and open a pull-request against the `fotony` (French for
`main`) branch.

Please don't introduce swear words, though: we will not excuse your Malagasy.

## but why would you do zat

- horsin around
- playing with raw proc macros
- making a bit of fun about programming languages that do this seriously,
  though I can see their utility.
- winking at [Marcel](https://github.com/brouberol/marcel)
- milay

## Other languages

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
- All of the above: [unirust](https://github.com/charyan/unirust)

## lisansa

[WTFPL](http://www.wtfpl.net/).
