# Funkcja dzieląca całkowicie liczby dodatnie
# Argumenty:
#   a0: dzielna
#   a1: dzielnik
#
# Zwracane wartości:
#   a0: wynik dzielenia
#   a1: reszta z dzielenia
.globl div
div:
    beq a1, zero, 3f    # Czy użytkownik próbuje dzielić przez zero?
    mv t0, a0           # Zapisz dzielną i dzielnik do rejestrów tymczasowych
    mv t1, a1
    li a0, 0            # Inicjalizacja rejestrów
    mv a1, t0
1:  blt  a1, t1, 2f     # Czy pozostała liczba jest mniejsza od dzielnika?
    sub  a1, a1, t1     # Odejmi od pozostałej liczby dzielnik
    addi a0, a0, 1      # Zanotuj że dzielnik jeszcze mieści się w liczbie
    j 1b                # Sprawdz czy możemy jeszcze odjąć dzielnik od liczby
2:  ret                 # Reszta z dzielenia jest mniejsza od dzielnika
                        # podzieliliśmy liczbę maksymalnie
3:  li a0, 0xE0000000   # Kod "próba dzielenia przez zero"
    ebreak
