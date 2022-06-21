import uuid

def main() -> None:
    keys = [
        [
            [uuid.uuid4().int & (1<<64)-1 for _ in range(64)]
            for _ in range(5)
        ]
        for _ in range(2)
    ]
    print(f"pub const ZOBRIST_KEYS: [[[u64; 64]; 5]; 2] = {keys};")

if __name__ == "__main__":
    main()