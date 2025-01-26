# Snake Rust

Prosta implementacja klasycznej gry Snake napisana w języku Rust z wykorzystaniem biblioteki `piston_window`.

## Funkcje:

Dwóch graczy: Obsługa dwóch węży sterowanych niezależnie (strzałki i WASD).
Kamienie: Dodatkowe przeszkody w postaci kamieni.
Opcja przechodzenia przez ściany: Możliwość włączenia/wyłączenia zawijania planszy.
Portale: Opcjonalne portale teleportujące węże.
Zmiana rozmiaru planszy: Użytkownik może zdefiniować szerokość i wysokość planszy.
Zmiana liczby kamieni: Użytkownik może zdefiniować liczbę kamieni na planszy.
Wykrywanie kolizji: Precyzyjne wykrywanie kolizji ze ścianami, kamieniami i samym sobą.
Wyświetlanie wyniku: Komunikat o wygranej/przegranej po zakończeniu gry.


## Sterowanie:

Gracz 1:
    Strzałka w górę: Ruch w górę
    Strzałka w dół: Ruch w dół
    Strzałka w lewo: Ruch w lewo
    Strzałka w prawo: Ruch w prawo
Gracz 2:
    W: Ruch w górę
    S: Ruch w dół
    A: Ruch w lewo
    D: Ruch w prawo