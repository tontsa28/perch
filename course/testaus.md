# Testausdokumentti

## Yksikkötestauksen kattavuusraportti
Kattavuus on saatu ajamalla komento:
```
$ cargo llvm-cov --summary-only
```

Raportti sisältää sekä yksikkötestit että integraatiotestit (`tests/`), koska `cargo llvm-cov` suorittaa molemmat testiluokat samalla ajolla.

**Yhteenveto:**
| Kategoria | Arvo |
| --- | --- |
| Regions | 2516 |
| Missed Regions | 343 |
| Region coverage | 86.37% |
| Functions | 162 |
| Missed Functions | 14 |
| Function coverage | 91.36% |
| Lines | 1584 |
| Missed Lines | 197 |
| Line coverage | 87.56% |

**Tiedostokohtainen yhteenveto:**
| Tiedosto | Region % | Function % | Line % |
| --- | --- | --- | --- |
| `bitboard.rs` | 100.00% | 100.00% | 100.00% |
| `board.rs` | 92.71% | 96.67% | 95.39% |
| `position.rs` | 98.02% | 95.52% | 97.80% |
| `mov.rs` | 97.75% | 100.00% | 98.13% |
| `uci.rs` | 89.24% | 100.00% | 86.75% |
| `search.rs` | 42.11% | 77.78% | 48.10% |
| `piece.rs` | 76.00% | 75.00% | 71.43% |
| `error.rs` | 32.14% | 33.33% | 44.00% |
| `attacks.rs` | 0.00% | 0.00% | 0.00% |

Huom: `attacks.rs` sisältää pääosin staattisia taulukoita, jotka eivät tuota line-coveragea ilman erillistä instrumentointia.
`search.rs` jää matalaksi, koska integraatiotesteissä hakua ajetaan vain kevyesti (syvyys 1), ja laajempi hakupuu jää testaamatta.
`evals.rs` puuttuu listasta kokonaan, koska se sisältää vain vakioita joita ei oikeastaan voi testata.

---

## Mitä on testattu ja miten?
**Yksikkötestit (src/*):**
- **Bitboard-operaatiot** (`bitboard.rs`)
  - bitin asettaminen, tyhjyyden tarkistus, vähiten merkitsevän bitin poisto (LSB), bittikohtaiset operaatiot.
- **Siirtojen jäsentäminen ja tulostus** (`mov.rs`)
  - UCI-siirtojen jäsentäminen, virheellisten syötteiden hylkääminen sekä korotussiirrot.
- **Lauta ja materiaalin evaluointi** (`board.rs`)
  - materiaalin pisteytys, sotilaattoman materiaalin tunnistaminen sekä hyökkäyslinjojen blokkaus.
- **Pelin sääntölogiikka** (`position.rs`)
  - laillisten siirtojen määrä (alkuasemassa = 20)
  - shakkimatti- ja pattiasemat -> 0 laillista siirtoa
  - linnoitus
    - oikeuksien hallinta
    - shakkiin joutumisen estot
    - linnoitusoikeuksien menettäminen
  - en passant ‑säännöt ja laittomien tilanteiden tunnistus (esim. kuninkaan paljastuminen shakkiin)
  - korotussiirtojen määrä
  - FEN-merkkijonojen jäsentäminen ja siirtovuoron tunnistus
  - siirtojen `make/unmake` -toimintojen inversio
  - `parse_uci_move` hyväksyy lailliset ja hylkää laittomat siirrot
- **Perft‑tarkistukset** (`position.rs::perft_tests`)
  - alkuasema, syvyydet 1-5
  - Kiwipete (linnoitus / en passant / korotukset)
  - Position3 (en passant ‑kiinnitysrajatapaukset)
  - Position5 (korotukset)

**Integraatiotestit (`tests/uci_integration.rs`):**
- päästä päähän -testit UCI-polulle: `position` + `perft` (alkuasema, Kiwipete)
- `position startpos moves ...` + `perft` -> palauttaa solmujen määrän
- `go depth 1` alkuasemasta -> palauttaa kelvollisen UCI‑siirron (ei `0000`)
- shakkimattiasemassa `go` -> `bestmove 0000`

---

## Minkälaisilla syötteillä testaus tehtiin?
**Edustavat ja epätriviaalit FEN‑asemat:**
- **Startpos:** `rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1`
- **Kiwipete:** `r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1`
- **Position3:** `8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1`
- **Position5:** `rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8`

**UCI‑siirrot (lailliset/laittomat):**
- laillisia: `e2e4`, `a1a2`, `e7e8q`
- laittomia: `e2e5`, väärät linjat/rivit, virheelliset korotukset

**Kriittiset sääntötilanteet:**
- shakkimatti / patti
- linnoitus oikeuksilla / ilman oikeuksia
- en passant sallittu / kielletty (kuninkaan paljastus)

---

## Testien toistaminen
**Kaikki testit:**
```
$ cargo test
```

**Pelkkä integraatiotesti:**
```
$ cargo test --test uci_integration
```

**Kattavuusraportti:**
```
$ cargo llvm-cov --summary-only
```

---

## Empiirinen testaus (graafinen esitys)
Alla on perft‑testin kasvun empiirinen visualisointi tavallisesta aloitusasemasta (solmujen määrä syvyyden mukaan).

**Perft startpos:**
| Syvyys | Solmut |
| --- | --- |
| 1 | 20 |
| 2 | 400 |
| 3 | 8 902 |
| 4 | 197 281 |
| 5 | 4 865 609 |

**ASCII‑graafi:**
```
Syvyys 1 | █
Syvyys 2 | ████████████████████
Syvyys 3 | █████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████████
```
Kuten graafista nähdään, jo syvyydellä 3 solmuja on niin paljon, että graafin esittäminen ei ole enää kovin käytännöllistä.

---

## Konkreettisia esimerkkejä testeistä
- **Testattu, että aloitusasemassa on 20 laillista siirtoa.**
- **Testattu, että aloitusasemassa perft syvyydellä 3 = 8 902**, joka vastaa tunnettua referenssiarvoa.
- **Testattu, että shakkimatti-asemassa `legal_moves()` on tyhjä** ja `go` palauttaa `bestmove 0000`.
- **Testattu, että laiton UCI‑siirto (`e2e5`) hylätään.**
- **Testattu, että en passant on laiton, jos se paljastaa kuninkaan.**
- **Testattu, että korotukset luodaan oikein** (4 push‑korotusta, 8 push+lyönti‑korotusta).
