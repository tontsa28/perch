# Määrittelydokumentti

Käytän ohjelmointikielenä itselleni jo entuudestaan tuttua Rustia.
Hallitsen kuitenkin ainakin Pythonia ja ehkä myös Javaa siten, että pystyn vertaisarvioimaan niillä toteutettuja projekteja.
Lisäksi JavaScript ja monet muut kielet, kuten C#, C, C++ ja Go ovat minulle ainakin hieman tuttuja.
En ole vielä täysin varma, mitä kaikkia tietorakenteita tulen toteuttamaan, mutta ainakin jonkinlainen bittilauta (engl. bitboard) lienee välttämätön.
Algoritmien osalta tulen toteuttamaan siirtojen hakua varten ainakin jonkinlaisen version minimax-algoritmista, luultavasti alfa-beeta-karsinnalla varustettuna.
Kyseessä on siis shakkimoottori, jonka tarkoituksena on selvittää paras siirto annetussa asemassa.
Ohjelma saa syötteenä UCI-protokollan mukaisia komentoja.
Esimerkkisyöte voisi olla FEN-merkkijono, jonka pohjalta ohjelma muodostaa sisäisten tietorakenteidensa avulla shakkiaseman.
Minimax-algoritmin aikavaativuus on ilmeisesti $O(b^d)$, missä $b$ on keskimääräinen mahdollisten siirtojen määrä asemaa kohti ja $d$ on hakusyvyys.
Tilavaativuus puolestaan on $O(bd)$.
Lähteinä aion käyttää ainakin seuraavia wikisivustoja:
- https://rustic-chess.org/
- https://www.chessprogramming.org

Harjoitustyön ydin on shakkiaseman tulkitseminen, laillisten siirtojen määrittäminen ja parhaan siirron etsiminen.
Nämä yhdessä muodostavat eräänlaisen tekoälyn.

Opinto-ohjelmani on TKT.
Projektin kieli tulee olemaan englanti, mutta kurssiin liittyviin raportteihin vastaan suomeksi (ellei englanniksi vastaaminen ole suositeltavaa).
