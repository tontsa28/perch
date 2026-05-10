# Toteutusdokumentti

## Ohjelman yleisrakenne
Ohjelma koostuu tällä hetkellä 11 eri lähdetiedostosta.
Käydään nämä tiedostot lyhyesti läpi:
- **main.rs**: Sisältää ohjelman main-funktion, käytännössä ei tee muuta kuin luo uuden `Uci`-rakenteen ja ajaa sen `run()`-metodin.
- **attacks.rs**: Sisältää kaikkien eri nappuloiden hyökkäystaulut, jotka mahdollistavat erittäin nopeat tarkastukset siitä, onko jokin ruutu hyökkäyksen kohteena ja jos on, minkä nappulan toimesta.
- **bitboard.rs**: Sisältää hyvin keskeisen `Bitboard`-rakenteen testeineen. `Bitboard` koostuu 64-bittisestä merkittömästä kokonaisluvusta, ja sen avulla voidaan pitää kirjaa esimerkiksi siitä, missä kaikissa ruuduissa tietyntyyppinen nappula on (yhtä ruutua vastaa yksi bitti).
- **board.rs**: Sisältää `Board`-rakenteen testeineen sekä myös yksinkertaisen `Color`-rakenteen. `Board` koostuu 15 `Bitboard`-rakenteesta, ja sitä käytetään tallentamaan koko shakkilaudan tilanne. `Color` puolestaan edustaa nappuloiden väriä (valkoinen tai musta).
- **error.rs**: Sisältää (varsin keskeneräisen) `Error`-rakenteen, jonka variantteja ovat erilaiset ohjelmassa mahdollisesti muodostuvat virheet.
- **evals.rs**: Sisältää keski- ja loppupeleihin sovellettavat PST-taulut (piece-to-square), joita käytetään position evaluaation apuna. Nämä taulut muodostavat arvon tietylle ruudulle, kun siinä on tietty nappula, ja tämä arvo summataan nappulalle määritettyyn vakioarvoon evaluaatiossa.
- **mov.rs**: Sisältää `Move`-rakenteen testeineen. `Move` edustaa shakkisiirtoa ja on keskeisessä asemassa muun muassa siirtojen luonnissa ja haussa.
- **piece.rs**: Sisältää nappuloihin liittyvät `PieceKind`- ja `PieceOnSquare`-rakenteet. Näitä rakenteita käytetään monessa paikassa, esimerkiksi pelilaudalle eli `Board`:lle tehtävissä operaatioissa.
- **position.rs**: Sisältää laajan `Position`-rakenteen. Tämä rakenne toimii ikään kuin `Board`:n päällä (joka toisaalta toimii `Bitboard`:n päällä) ja sisältää pelilaudan tilanteen lisäksi tiedot siirtäjän väristä, linnoitusoikeuksista, mahdollisesta en-passant -ruudusta, puolisiirroista sekä täyssiirroista. Rakennetta käytetään siirtojen haussa.
- **search.rs**: Sisältää varsinaisen hakualgoritmin (negamax) toteutuksen optimointeineen. Algoritmia käytetään parhaan siirron etsimiseen annetussa positiossa.
- **uci.rs**: Sisältää UCI-käyttöliittymää esittävän `Uci`-rakenteen. Rakenne on vastuussa käyttäjän kirjoittamien komentojen käsittelystä ja ajaa niiden perusteella eri pääfunktioita. Esimerkiksi go-komento ajaa `iterative_deepening()`-funktion, joka suorittaa parhaan siirron haun.

## Aika- ja tilavaativuus + käytännön suorituskyky
Hakualgoritmin aikavaativuus on huonoimmassa tapauksessa $O(b^d)$, missä $b$ on keskimääräinen haarautumiskerroin (laillisia siirtoja positiota kohden) ja $d$ on hakusyvyys.
Käytännössä tämä tarkoittaa sitä, että algoritmin suoritusaika kasvaa eksponentiaalisesti hakusyvyyden kasvaessa.

Tilavaativuus puolestaan on $O(bd)$, eli käytännössä jokainen hakukerros vaatii position laillisten siirtojen verran tilaa.

Haun teoreettinen aikavaativuus ei kuitenkaan vastaa todellisuutta.
Käytännössä alfa-beeta-karsinnan ja muiden optimointien ansiosta haku on paljon huonointa tapausta nopeampi, koska karsintaa tehdään ainakin vähän melkein jokaisessa positiossa.
On varmasti olemassa keinotekoisia positioita, joissa karsintaa ei voi tapahtua, mutta tällaisen löytäminen sattumalta lienee miltei mahdotonta.

## Mahdolliset puutteet ja parannusehdotukset
Vaikka ohjelma saavuttaa jo kohtuullisia hakusyvyyksiä ja positioiden evaluointi toimii melko hyvin, parannettavaa on vielä paljon.
Muun muassa siirtojen generointia voisi vielä nopeuttaa: esimerkiksi aloitusasemasta 7 siirron syvyyteen asti kaikkien solmujen luominen kestää noin 70 sekuntia, mutta maailman johtava shakkimoottori, Stockfish, tekee saman alle 20 sekunnissa (nämä mitattu tietysti samalla tietokoneella).
Tämän parantamisella ei kuitenkaan saavutettaisi vielä dramaattisia nopeutuksia, vaan niitä on saatavissa optimoimalla hakualgoritmia siten, että mahdollisimman vähän solmuja käydään ylipäätään läpi.
Esimerkiksi transpositiotaulun voisi toteuttaa uudelleen omalla tietorakenteella, jolloin sitä voitaisiin käyttää myös jo arvioitujen positioiden tallentamiseen.
Nykyään maailman johtavat shakkimoottorit hyödyntävät myös neuroverkkoja, mutta itse tuskin lähden sellaista tähän projektiin ikinä toteuttamaan.
Näiden lisäksi on varmasti paljon muitakin optimointeja, joista en ole vielä tietoinen.

## Laajojen kielimallien käyttö
Olen käyttänyt projektin aikana kolmea eri kielimallia, jotka ovat Claude Sonnet 4.6, GPT-5.2-Codex sekä GPT-5.3-Codex.
Käytin kielimalleja ymmärtämään projektin eri osa-alueiden toimintaa paremmin.
Niistä oli myös hyötyä alkuun pääsemisessä, koska lähtökohta oli se, että en tiennyt shakkimoottorin toiminnasta juuri mitään.
Vaikka kielimallit ovat olleet osana koodin ideointia, kaikki koodi on kuitenkin viime kädessä itse kirjoitettua.

## Työssä käytetyt lähteet
Päälähteenä on toiminut [chessprogramming.org](https://www.chessprogramming.org).
Minimax/negamax-algoritmin ja alfa-beeta-karsinnan ymmärtämisessä on ollut myös apua [Sebastian Laguen YouTube-videosta](https://www.youtube.com/watch?v=l-hh51ncgDI&t=496s).
