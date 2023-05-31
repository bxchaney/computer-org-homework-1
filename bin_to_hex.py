import sys

def bin_to_hex(s: str) -> str:
    """
    Assuming that s is only 0's and 1's, this function converts a binary
    representation to a hexideciaml representation.
    """
    hex_char = [
        '0','1','2','3','4','5','6','7','8','9','a','b','c','d','e','f'
    ]
    # padding the front of the string with 0's until the total number of
    # characters in the string is a multiple of 4
    r = len(s) % 4
    if r != 0:
        s = '0' * (4 - r) + s
    i = 0
    hex_str = ''
    iterations = len(s) // 4
    while i < iterations:
        t = s[i*4:(i+1)*4]
        j = 1
        v = 0
        for c in t[::-1]:
            if c == '1':
                v += j 
            j <<= 1
        hex_str += hex_char[v]       
        i += 1
    return hex_str


def main(argv: list[str]) -> None:
    if len(argv) <= 1:
        return 

    for s in argv[1:]:
        print(f"bin: {s} -> hex: {bin_to_hex(s)}")  

if __name__ == '__main__':
    main(sys.argv)