
U64_MAX = 2 ** 64 - 1
SHIFT_RIGHT_MASK = 9259542123273814144
SHIFT_LEFT_MASK = SHIFT_RIGHT_MASK >> 7

def print_bitboard(board):
    string = bin(board)[2:]
    string = "0" * (64 - len(string)) + string
    for i in reversed(range(8)):
        print(string[i * 8:(i + 1) * 8][::-1].replace("0", ". ").replace("1", "1 "))
    print("=" * 16)
    input()

def calculate_seal_pattern():
    result = []
    for i in range(64):
        bit = 1 << i
        right = (((bit & ~SHIFT_RIGHT_MASK) << 1 & ~SHIFT_RIGHT_MASK) << 1) & U64_MAX
        left = (((bit & ~SHIFT_LEFT_MASK) >> 1 & ~SHIFT_LEFT_MASK) >> 1) & U64_MAX
        up = (bit >> 16) & U64_MAX
        down = (bit << 16) & U64_MAX
        destinations = right << 8 | right >> 8
        destinations |= left << 8 | left >> 8
        destinations |= (down & ~SHIFT_RIGHT_MASK) << 1 | (down & ~SHIFT_LEFT_MASK) >> 1
        destinations |= (up & ~SHIFT_RIGHT_MASK) << 1 | (up & ~SHIFT_LEFT_MASK) >> 1
        result.append(destinations & U64_MAX)
    print("#[rustfmt::skip]")
    print(f"pub const SEAL_PATTERN: [u64; 64] = {result};")

def calculate_starfish_pattern():
    result = []
    # Red
    for i in range(64):
        bit = 1 << i
        destinations = (bit & ~SHIFT_LEFT_MASK) << 7 | (bit & ~SHIFT_LEFT_MASK) >> 9
        destinations |= (bit & ~SHIFT_RIGHT_MASK) << 9 | (bit & ~SHIFT_RIGHT_MASK) >> 7
        destinations |= (bit & ~SHIFT_RIGHT_MASK) << 1
        result.append(destinations & U64_MAX)
    # Blue
    for i in range(64):
        bit = 1 << i
        destinations = (bit & ~SHIFT_LEFT_MASK) << 7 | (bit & ~SHIFT_LEFT_MASK) >> 9
        destinations |= (bit & ~SHIFT_RIGHT_MASK) << 9 | (bit & ~SHIFT_RIGHT_MASK) >> 7
        destinations |= (bit & ~SHIFT_LEFT_MASK) >> 1
        result.append(destinations & U64_MAX)
    print("#[rustfmt::skip]")
    print(f"pub const STARFISH_PATTERN: [u64; 128] = {result};")

def calculate_cockle_pattern():
    result = []
    # Red
    for i in range(64):
        bit = 1 << i
        destinations = ((bit & ~SHIFT_RIGHT_MASK) << 9 | (bit & ~SHIFT_RIGHT_MASK) >> 7) & U64_MAX
        result.append(destinations)
    # Blue
    for i in range(64):
        bit = 1 << i
        destinations = ((bit & ~SHIFT_LEFT_MASK) << 7 | (bit & ~SHIFT_LEFT_MASK) >> 9) & U64_MAX
        result.append(destinations)
    print("#[rustfmt::skip]")
    print(f"pub const COCKLE_PATTERN: [u64; 128] = {result};")

def calculate_gull_pattern():
    result = []
    for i in range(64):
        bit = 1 << i
        destinations = ((bit & ~SHIFT_RIGHT_MASK) << 1 | (bit & ~SHIFT_LEFT_MASK) >> 1 | bit >> 8 | bit << 8) & U64_MAX
        result.append(destinations)
    print("#[rustfmt::skip]")
    print(f"pub const GULL_PATTERN: [u64; 64] = {result};")

if __name__ == "__main__":
    calculate_seal_pattern()
    calculate_starfish_pattern()
    calculate_cockle_pattern()
    calculate_gull_pattern()

