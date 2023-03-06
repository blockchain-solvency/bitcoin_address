import os
from hashlib import sha256
import sys


def progress_bar(it, prefix="", size=60, out=sys.stdout):  # Python3.3+
    count = len(it)

    def show(j):
        x = int(size * j / count)
        print(
            "{}[{}{}] {}/{}".format(prefix, "#" * x, "." * (size - x), j, count),
            end="\r",
            file=out,
            flush=True,
        )

    show(0)
    for i, item in enumerate(it):
        yield item
        show(i + 1)
    print("\n", flush=True, file=out)


def read_file_to_lines(path):
    with open(path) as f:
        return [line.rstrip("\n") for line in f]


digits58 = "123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz"


def decode_base58(bc, length):
    n = 0
    for char in bc:
        n = n * 58 + digits58.index(char)
    return n.to_bytes(length, "big")


def validate_address(bc):
    try:
        bcbytes = decode_base58(bc, 25)
        return bcbytes[-4:] == sha256(sha256(bcbytes[:-4]).digest()).digest()[:4]
    except Exception:
        return False


path = os.path.join(os.getcwd(), "addresses.txt")
lines = read_file_to_lines(path)
for address in progress_bar(lines):
    # Adapte from: https://rosettacode.org/wiki/Bitcoin/address_validation#Python
    validate_address(address)
print("All addresses are valid!")
