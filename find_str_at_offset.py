import sys

if len(sys.argv) < 3:
    print('Usage: python read_binary.py <filename> <offset>')
    sys.exit()

filename = sys.argv[1]
offset = int(sys.argv[2], 16)

with open(filename, 'rb') as f:
    f.seek(offset)
    string = b''

    while True:
        char = f.read(1)
        if char == b'\x00':
            break
        string += char

    print(string.decode())
