# Käyttöohje

## Yleiskuvaus
Ohjelma käyttää UCI-tyylistä komentorivikäyttöliittymää.
Komennot luetaan `stdin`-syötteestä ja vastaukset tulostetaan `stdout`-virtaan.

---

## Kääntäminen
Projektin juuresta:
```
cargo build --release
```

Optimoitu binääri löytyy polusta:
```
target/release/perch
```

---

## Käynnistys

### Cargoa käyttäen
```
cargo run --release
```

### Suoraan binäärillä
```
./target/release/perch
```

Ohjelma käynnistyy ja odottaa syötteitä `stdin`:stä.

---

## Komennot

### `help`
Tulostaa lyhyen avustetekstin.
```
help
```

### `d`
Tulostaa nykyisen laudan ASCII-muodossa.
```
d
```

### `position`
Aseta peliasema.

**Aloitusasema:**
```
position startpos
```

**FEN-asema:**
```
position fen rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1
```

**FEN + siirtolista:**
```
position startpos moves e2e4 e7e5 g1f3
```

Komento lukee siirrot järjestyksessä ja rakentaa lopullisen aseman.

### `go`
Aloita haku.

**Oletussyvyys:**
```
go
```

**Tietty syvyys:**
```
go depth 8
```

Palauttaa:
```
bestmove <uci-siirto>
```

Oletussyvyys on 6, jos syvyyttä ei anneta.

### `perft`
Laskee solmumäärän perft-testissä.

**Ilman syvyyttä:**
```
perft
```

**Tietty syvyys:**
```
perft 5
```

Palauttaa:
```
nodes <määrä>
```

Oletussyvyys on 0, jos syvyyttä ei anneta.

### `quit` / `exit`
Lopettaa ohjelman.

```
quit
```

tai
```
exit
```

---

## Siirtoformaatti
Perch käyttää UCI-koordinaattinotaatiota:
- Tavallinen siirto: `e2e4`
- Korotus: `e7e8q`
- Linnoitus: `e1g1`, `e1c1`, `e8g8`, `e8c8`

SAN-notaatiota (esim. `Nf3`) ei tueta.

---

## Virhetilanteet
Tuntemattomista komennoista tai virheellisistä syötteistä tulostetaan virheilmoitus `stderr`-virtaan. Ohjelma jatkaa toimintaansa normaalisti.
Esimerkiksi syötteiden antaminen, joista seuraa laiton asema, voi kuitenkin johtaa ohjelman kaatumiseen.

---

## Kehitysvinkit
Debug-ajo (nopeampi kääntö, hitaampi moottori):
```
cargo run
```

Formatointi:
```
cargo fmt
```

Linttaus:
```
cargo clippy
