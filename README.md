# Infi Advent of Code (2018) solution

The challenge (text in Dutch):

## Part 1
> ### Breaking nieuws: Santa achter de tralies
> 
> Er is een anonieme melding binnengekomen over een mogelijke schending van de GDPR door de heer Santa Claus. Santa wordt verdacht van het zonder toestemming bewaren van namen, adressen en persoonlijke profielen van mensen in de hele wereld. Terwijl het onderzoek gaande is blijft Santa preventief in de gevangenis opgesloten.
> 
> Dit is natuurlijk een ramp, want het is bijna kerstnacht en hij moet aan het werk! Het geluk van veel mensen hangt ervan af dat hij ontsnapt. Dankzij zijn magische krachten is het Santa al gelukt om uit zijn cel te komen. Dit was helaas al voorzien door de bewakers, die de gevangenis in de vorm van een doolhof hebben gebouwd. Kun je hem helpen de weg naar buiten te vinden? En snel ook!
> 
> Elke karakter geeft een tegel met 2 tot 4 uitgangen weer. Voorbeelden hiervan zijn:
> 
> * ═ met uitgangen aan de linker- en rechterkant.
> * ╦ met uitgangen aan de linker-, rechter- en onderkant.
> * ╬ met uitgangen aan alle kanten. 
> 
> Alle mogelijke tegels zijn: ║ ╔ ╗ ╠ ╦ ╚ ╝ ╬ ╩ ═ ╣.
> 
> ```
> voorbeeld (stappen: 6)
>  ╔  ═  ╗  ║ 
>  ╠  ╗  ╠  ║ 
>  ╬  ╬  ╣  ╬ 
>  ╚  ╩  ╩  ═ 
> ```
> 
> Santa bevindt zich linksboven en moet rechtsonder zien te komen. Hij kan alleen naar de volgende tegel als de uitgangen aansluiten. Bijvoorbeeld, bij ═╦ zou hij van links naar rechts kunnen, maar niet bij ═╚. Kun je de kortste route vinden naar de uitgang?
> 
> Hoeveel stappen heeft Santa nodig om op de tegel rechtsonder te komen?
> 

## Part 2
> ### Oh nee!
> 
> De route is duidelijk en Santa meent al vrij te zijn, maar na de eerste stap blijkt het doolhof een gemene truuk te hebben: de tegels verschuiven! Hij ontdekt het volgende patroon:
> 
> * Na de eerste stap verschuift de eerste rij naar rechts, na de tweede stap verschuift de tweede kolom naar beneden, na de derde stap verschuift de derde rij naar rechts, en zo gaat het maar door...
> * Als er geen rijen of kolommen meer zijn om te verschuiven (bijvoorbeeld wanneer het doolhof 5 tegels breed is en Santa 6 stappen heeft gezet) begint het weer bij de eerste rij of kolom (afhankelijk van welke aan de beurt is).
> * Tegels die buiten het doolhof schuiven komen weer bij aan het begin van de betreffende rij of kolom (e.g. als een rij naar rechts schuift komt de laatste tegel aan het begin van de rij te staan).
> * Santa schuift zelf mee als de tegel waarop hij staat beweegt, behalve wanneer hij op de tegel rechtsonder staat, dan loopt hij direct naar buiten en zijn er geen verdere stappen meer nodig. 
> 
> Kun je de kortste route vinden naar de uitgang? Deze keer rekening houdend met de verschuivingen die na elke stap plaatsvinden.
> 
> Hoeveel stappen heeft Santa nodig om op de tegel rechtsonder te komen?
