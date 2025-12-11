Raport : pierwsza część
Niestety dużą część założeń z pierwszej części nie udało się na razie zrealizować ze względu na błędne oszacowanie ilości czasu potrzebnego do napisania potrzebnych elementów.

Na ten moment udało się zrealizować:
Menu główne z przejściem bezpośrednio do pola bitwy. 
Rozpoczynanie bitwy importujące elementy ze stanu gry do stanu pola bitwy
Pole bitwy na hexagonalnej siatce z turową walką (na ten moment stali przykładowi przeciwnicy oraz bohater):
Kolejność tur na podstawie statystki bohatera/potwora
Poruszanie się bohatera w swojej turze o ilość pól określoną przez statystykę. Poruszanie jest blokowane przez inne jednostki oraz obiekty statyczne (kamienie)
Proste AI przeciwników. Idą w stronę najbliższego przeciwnika i jak skończą turę obok niego to atakują.
2 różne sposoby ataku bohatera, potężniejszy o zasięgu 1 i słabszy o zasięgu 3.
Aktualizujące się paski zdrowia ze zmiennym kolorem w zależności od stanu.

Większość elementów wymaga jeszcze udoskonalenia i poprawy. Umiejętności powinny pokazywać swój zasięg przy wyborze. Czasem trzeba ponownie kliknąć postać, aby móc się poruszyć 
po wykonaniu ataku lub części ruchu. Nie ma jeszcze zaimplementowanego poprawnego rozpatrywania śmierci jednostek, należy przebudować sposób trzymania ich w strukturze stanu pola bitwy.

Do części z rzeczy które się nie udały przygotowane są szkielety, pola w strukturach na przyszłość.
Nie udało się:
Doświadczenie i związane z tym odblokowywanie zdolności w drzewku.
Obsługi ekwipunku, tworzenia i zakładania przedmiotów, zbioru materiałów.
Efekty oddziałujące na jednostki podczas bitwy.