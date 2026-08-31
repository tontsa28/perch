# Viikkoraportti 6

### 1. Mitä olen tehnyt tällä viikolla?
Tällä viikolla olen parannellut evaluaatiofunktiota, joka ottaa nyt huomioon nappuloiden sijainnin pelilaudalla.
Tämän pitäisi tuottaa parempia tuloksia.
Evaluaatiota voisi toki kehittää vielä pidemmälle, mutta en ainakaan tämän kurssin puitteissa aio enää kiinnittää siihen enää huomiota.
Evaluaation lisäksi olen aloittanut hieman koodin kommentointia sekä tehnyt yksikkötestejä muutamille tietorakenteille.

### 2. Miten ohjelma on edistynyt?
Ohjelma on edistynyt mielestäni kohtalaisen hyvin.
Enemmänkin olisi toki voinut tehdä, mutta periodi lähestyy loppuaan ja myös muut kurssit pitävät kiireisenä.

### 3. Mitä opin tällä viikolla?
Opin kantapään kautta sen, milloin kannattaa käyttää referenssejä ja milloin ei.
Toteutin evaluaatiofunktion aluksi siten, että kooltaan kohtalaisen suuret - 256-tavuiset - tietorakenteet kopioitiin muistissa.
Sitten pienen selvittelyn jälkeen ymmärsin, että tässä tilanteessa kannattaa ehdottomasti hyödyntää referenssejä, jotka ovat kooltaan vain 8 tavua.
Tämä hyvin yksinkertainen muutos johti huomattavaan, jopa dramaattiseen suorituskyvyn nousuun.
Toisaalta opin myös sen, että aina ei ole järkevää käyttää referenssiä, vaan pienten tietorakenteiden kanssa on usein parempi kopioida muuttuja muistissa.

### 4. Mikä jäi epäselväksi tai on tuottanut vaikeuksia?
Referenssit tuottivat hieman vaikeuksia, mutta eivät onneksi jääneet kuitenkaan epäselväksi.
Yritin korjailla iteratiivista syvenemistä, josta sain viime viikolla palautetta ja joka jäi hieman epäselväksi.
Jos korjattavaa kuitenkin edelleen on, niin yritän löytää ensi viikolta vielä aikaa Zoom-palaverille.

### 5. Mitä teen seuraavaksi?
Seuraavaksi ajattelin keskittyä dokumentaation kirjoittamiseen sekä myös testien lisäämiseen (mikäli testejä tarvitaan vielä lisää?).
Näiden lisäksi täytyy tietysti kirjoittaa kaikki pakolliset dokumentit valmiiksi ja tehdä vertaisarviointi.
En usko, että ehdin viimeisen viikon aikana enää juurikaan lisäämään uusia ominaisuuksia.
Haluaisin kyllä lisätä transpositiotaulun, mutta pahoin pelkään ettei sille riitä enää aikaa.
Jos kuitenkin riittää, niin se lienee ainut uusi ominaisuus jota tässä vaiheessa enää lähtisin toteuttamaan.

### Työtunnit
Arvioisin, että olen käyttänyt tämän viikon aikana projektiin noin 10 tuntia.
