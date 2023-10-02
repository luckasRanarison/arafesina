arafesina::arafesina! {
    ivelany baoritra arafesina;
    mampiasa std::fahadisoana::Fahadisoana;

    lefa fotony() -> Valiny<(), Vata<dinamika Fahadisoana>> {
        atao za = Olona::vaovao("Olona".atao_azo(), Tsisy);

        za.miarahaba();

        atao manana_fanampiny = mampitahy za.fanampiny {
            Misy(_) => marina,
            Tsisy => diso,
        }; // na mampiasa raha_misy()

        raha manana_fanampiny {
            zavatra!();
        }

        atao miova lisitra = Lisitra::vaovao();

        lisitra.ampiana(1);
        lisitra.ampiana(2);
        lisitra.ampiana(3);

        hoan_ny zavatra anaty lisitra {
            manoratra!("{}", zavatra);
        }

        manoratra!("{:?}", ankasa_ve(2));

        Mety(())
    }

    rafitra Olona {
        anarana: Fatorana,
        fanampiny: Arakaraka<Fatorana>
    }

    fampiharana Olona {
        daholobe lefa vaovao(anarana: Fatorana, fanampiny: Arakaraka<Fatorana>) -> Tena {
            Tena { anarana, fanampiny }
        }
    }

    fitsipika Miarahaba {
        lefa miarahaba(&tena);
    }

    fampiharana Miarahaba hoan_ny Olona {
        lefa miarahaba(&tena) {
            manoratra!("Manahoana ny tany! {} ny anarako", tena.anarana);
        }
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
