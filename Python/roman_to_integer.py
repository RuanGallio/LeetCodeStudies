def romanToInt(rom: str) -> int:
    """Given a roman numeral, convert it to an integer. """

    r = {"I": 1, "V": 5, "X": 10, "L": 50, "C": 100, "D": 500, "M": 1000}
    n = 0
    i = 0
    while i < len(rom):
        rom_key = rom[i]
        num = r[rom_key]
        if i+1 < len(rom):
            next = rom[i+1]
        else:
            next = rom[i]
        if r[next] > r[rom_key]:
            num = r[next] - r[rom_key]
            i += 1
        n = n+num
        i += 1
    return n
