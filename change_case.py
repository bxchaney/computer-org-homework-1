import sys

def toUpper(s: str) -> str:
    t = ''
    for c in s:
        if c >= 'a' and c <= 'z':
            t += chr(ord(c) & 0b1011111)
        else:
            t += c
    return t

def toLower(s: str) -> str:
    t = ''
    for c in s:
        if c >= 'A' and c <= 'Z':
            t += chr(ord(c) | 0b0100000)
        else:
            t += c
    return t

def main(argv):
    if len(argv) <= 1:
        return
    
    for s in argv[1:]:
        print(f"toUpper: {toUpper(s)}")
        print(f"toLower: {toLower(s)}")


if __name__ == '__main__':
    main(sys.argv)
