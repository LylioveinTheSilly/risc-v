# Dane są trzy liczby
#
# s0 - dzielna
# s1 - dzielnik
# s2 - number cyfry
#
# Dzieląc s0 przez s1 uzyskujemy liczbę b. (s0 / s1 = b)
# s2 wskazuję n-tą cyfrę za *pierwszą niezerową* cyfrą wyniku
# która jest odpowiedzią na zadanie. 
#
# Przykład:
# s0: 100
# s1: 70
# s2: 4
#
# s0 / s1 = 1.4285714285714285
#               ^
#               s2
#
# Napisz program wskazujący tą liczbę. Cyfra powinna
# znaleść się w rejestrze a0.
.globl main
main:
    li s0, 100
    li s1, 70
    li s2, 70
    li s3, 0                # Flaga, która sprawdza, czy znaleśliśmy pierwszą, niezerową cyfrę

1:  blt s0, s1, 2f          # Aby algorytm zadziałał poprawnie, dzielnik musi być większy od dzielnej
    mv a0, s1
    li a1, 10
    call mul                # Jeśli tak nie jest to przemnóż dzielnik przez 10 w pętli, dopóki nie stanie się większy do dzielnej 
    mv s1, a0
    j 1b
2:  mv a0, s0
    mv a1, s1

    loop:
        beq s2, zero, end   # Czy znajeźliśmy naszą cyfrę w poprzedniej iteracji?

        # Nie, dalej dzielenie pod kreską! :D
        call div            # Ile s1'ek mieści się w s0?
        mv s0, a0           # Zapisz wynik
        mv a0, a1           # Przenieś resztę z dzielenia do a0 i przemnóż przez 10
        li a1, 10
        call mul
        mv a1, s1           # Przenieś dzielnik do a1 (aby przygotować się na kolejną operację dzielenia)

        sltu t0, zero, s3   # Czy napotkaliśmy już pierwszą niezerową cyfrę?
        sltu t1, zero, s0   # Czy natrafiliśmy po dzieleniu na niezerową cyfrę?  
        or t0, t0, t1       # Czy którykolwiek z warunków wyżej jest prawdziwy?
        li t1, 1
        bne t0, t1, loop    # Sprawdź czy któryś warunek jest prawdziwy
        addi s2, s2, -1     # Prawda! Licznik cyfr do dołu
        li s3, 1            # Ustawiamy flagę, że odnaleźliśmy już niezerową cyfrę
        j loop
    
    end: 
        mv a0, s0
        ebreak

mul:
    mv t0, a0
    li a0, 0
1:  beq a1, zero, 2f
    add a0, a0, t0
    addi a1, a1, -1
    j 1b
2:  ret