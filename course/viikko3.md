# Viikkoraportti 3

### 1. Mitä olen tehnyt tällä viikolla?
Tällä viikolla olen toteuttanut negamax-algoritmin prototyypin kolmannen osapuolen kirjaston avulla.
Lisäsin myös algoritmiin alfa-beta -karsinnan.
Jatkoin myös hieman omien tietorakenteideni kehitystä, mutta en ole ainakaan vielä kirjoitushetkellä pushannut näitä muutoksia GitHubiin.

### 2. Miten ohjelma on edistynyt?
Ohjelma ei ole edistynyt tällä viikolla ihan yhtä paljon kuin olisin toivonut, viikosta tuli arvioitua kiireisempi.
Olen kuitenkin saanut ohjelman siihen pisteeseen että sitä vastaan voi jo pelata, mikä lienee ihan hyvä saavutus.
Ohjelma tuntuu osaavan taktiikat yllättävänkin hyvin negamax-haun avulla, mutta aloituksissa ja etenkin voittamisessa on vielä ongelmia.

### 3. Mitä opin tällä viikolla?
Opin, miten negamax-algoritmi alfa-beta -karsinnalla varustettuna toteutetaan käytännössä.
Ymmärrän nyt myös kohtalaisella tasolla, miten algoritmi ja karsinta toimivat (uskon kyllä vähitellen oppivani ymmärtämään ne läpikotaisin).

### 4. Mikä jäi epäselväksi tai on tuottanut vaikeuksia?
Pelin voittamisen korjaaminen on tuottanut toistaiseksi harmaita hiuksia.
Luulen tietäväni, miksi aloitukset ovat kehnoja: en ole vielä huomioinut sitä, mitkä ruudut ovat millekin nappuloille hyviä ja mitkä eivät.
Tämän vuoksi olisi olennaista kertoa evaluaatiofunktiolle esimerkiksi se, että sotilaat on parempi sijoittaa pelin alussa laudan keskelle kuin reunoille.
Isompi ongelma on kuitenkin minulle edelleen mysteeri: ohjelma ei osaa voittaa peliä.
Paitsi joskus.
Olen saanut sen tekemään kahdesti shakkimatin itseäni vastaan, mutta usein jos yritän tarkoituksella saada itseni huonoon asemaan, se pelaa siirtoja siten että lopputuloksena on patti tai aseman kolminkertainen toisto.
Näin tapahtuu myös silloin, kun ohjelma on järjettömässä etulyöntiasemassa, jopa hakusyvyytensä etäisyydellä matista.
Tämän juurisyy ei ole vielä selvinnyt minulle, mutta jatkan selvitystä.

### 5. Mitä teen seuraavaksi?
En ole täysin varma.
Luultavasti jatkan omien tietorakenteideni kehitystä nyt kun minulla on jonkinlainen käsitys siitä, mitä niihin täytyy lisätä että voin korvata valmiin kirjaston rakenteet omillani.
Siitä saisi sitten paitsi lisää tehoa ohjelmaan, myös hyvät lähtökohdat jatkaa kehitystä omien rakenteideni varassa.
Tästä kaipaisin kuitenkin palautetta, jos tämä ei kuulosta lainkaan järkevältä jatkosuunnitelmalta vielä tässä vaiheessa.
Viime palautteen luin vasta, kun olin jo ehtinyt alkaa toteuttamaan hakualgoritmia.
Siksi en jatkanut tällä viikolla vielä tietorakenteita (paitsi hieman ehdin aloitella).
Toisaalta tässä kohtaa olisi varmaan myös melko olennaista alkaa kirjoittamaan kovalla vauhdilla testejä.
Dokumentaatiota en halua vieläkään kirjoittaa, koska en ole varma, onko projektissa vieläkään yhtään sellaista koodia, jota en enää varmuudella muuttaisi missään vaiheessa.

### Työtunnit
Tällä viikolla työtunnit jäivät hieman vähemmälle kuin viime viikolla, olisivatkohan olleet 10 tunnin tienoilla.
