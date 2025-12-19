Raport: pierwsza część
Niestety dużą część założeń z pierwszej części nie udało się na razie zrealizować ze względu na błędne oszacowanie ilości czasu potrzebnego do napisania potrzebnych elementów.
Wprowadzono dodatkowe poprawki po terminie które głównie polegały na porządkowaniu kodu i ogólnym przepisaniu kodu aby był łatwiej skalowalny.
Nie wszystkie funkcjonalności sprzed porządkowania zostały zaimplementowane, ale zostały też dodane nowe. Jeśli dobrze pamiętam brakuje jedynie
podświetlania pól oraz zmieniono trochę prezentacje terenu (assety zastąpiono na ten moment kolorami). Brakuje też możliwości zaznaczenia jednostek
które nie mają aktualnie tury aby zobaczyć ich zasięg jednak w pierwotnej wersji było to mocno zbugowane, stąd decyzja o tymczasowym usunięciu tej mechaniki. 
Udało się też dodać usuwanie jednostek po śmierci co było głównym powodem refactoringu kodu.

Na ten moment udało się zrealizować:
Menu główne z przejściem bezpośrednio do pola bitwy. 
Rozpoczynanie bitwy importujące elementy ze stanu gry do stanu pola bitwy
Pole bitwy na hexagonalnej siatce z turową walką (na ten moment stali przykładowi przeciwnicy oraz bohater):
Kolejność tur na podstawie statystki bohatera/potwora.
Poruszanie się bohatera w swojej turze o ilość pól określoną przez statystykę. Poruszanie jest blokowane przez inne jednostki oraz teren nieprzechodni (oznaczony tymczasowo znakiem x).
Proste AI przeciwników. Idą w stronę najbliższego przeciwnika i jak skończą turę obok niego to atakują.
2 różne sposoby ataku bohatera, potężniejszy o zasięgu 1 i słabszy o zasięgu 3.
Aktualizujące się paski zdrowia ze zmiennym kolorem w zależności od stanu.
Ekran końca bitwy.

Większość elementów wymaga jeszcze udoskonalenia i poprawy.

Do części z rzeczy które się nie udały przygotowane są szkielety, pola w strukturach na przyszłość.
Kod nie jest w zupełności (choć w większości) uprzątnięty ze względu na to, że będą jeszcze wprowadzane zmiany na potrzeby drugiej części.
Nie udało się:
Doświadczenie i związane z tym odblokowywanie zdolności w drzewku.
Obsługi ekwipunku, tworzenia i zakładania przedmiotów, zbioru materiałów.
Efekty oddziałujące na jednostki podczas bitwy.

Prawdopodobnie ciężko będzie zaimplementować wszystkie zadeklarowane funkcjonalności. Wydaje mi się że istotniejszym byłoby ubogacenie działających
mechanik niż dodawanie nowych z małą zawartością. Prawdopodobnie nie uda się zaimplementować mapy przygody albo będzie ona bardzo uproszczona (jedynie wybieranie walk).