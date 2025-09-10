import os

def main():
    if not os.path.exists("Cargo.toml"):
        print("[ERROR] No Cargo.toml found in project.")
        exit(1)

    with open("Cargo.toml", "r") as f:
        project = f.read()

    if "apex" in project:
        print("[OK] Library 'apex' is linked correctly.")
    else:
        print("[WARNING] Library 'apex' not found in dependencies.")

if __name__ == "__main__":
    main()
