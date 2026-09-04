# Nathaniel C. Moratto
# Symbol Lookup
from symbols import symbols

def is_valid_string(input: str) -> bool:
    try:
        _ = int(input)
        return False
    except:
        return True

def handle_symbol_search(user_symbol: str):
    if user_symbol not in symbols:
        print(f"\nYour symbol {user_symbol} is not in the database. Please try again!")
        get_input()
    else:
        found_symbol = symbols[user_symbol]
        print("\nGood News! We found your symbol!")
        print(f"  Symbol: {user_symbol}")
        print(f"  Given Name: {found_symbol["givenName"]}")
        aliasNames = found_symbol["aliasNames"]
        if len(aliasNames) > 0:
            print(f"  Also known as: {", ".join(aliasNames)}")

        print(f"\nThat was fun! Let's do it again!")
        get_input()

def handle_extended_search(input: str):
    if input == "all":
        print("\nAll available symbols:")
        for key, value in symbols.items():
            print(f"  {key} - {value["givenName"]}")

        print(f"\nBack it up! Back to search!")
        get_input()
    elif input == "exit":
        print("\nGoodbye, thank you for coming!")
        exit(0)

    found_symbol = None
    for key, value in symbols.items():
        names = [value["givenName"], *value["aliasNames"]]
        names = list(map(lambda x: x.lower(), names))

        if input in names:
            found_symbol = (key, value)

    if found_symbol is not None:
        print("\nGreat news! We found the symbol you were looking for:")
        print(f"  Symbol: {found_symbol[0]}")
        print(f"  Given Name: {found_symbol[1]["givenName"]}")
        aliasNames = found_symbol[1]["aliasNames"]
        if len(aliasNames) > 0:
            print(f"  Also known as: {", ".join(aliasNames)}")

        print("\nBack to search!")
        get_input()
    else:
        print(f"\nUnfortuantely I wasn't able to find what you meant by '{input}'. Please try again")
        get_input()

def get_input(user_input: str | None = None):
    user_input = input("Search: ")

    if len(user_input) <= 0:
        print("\nYour search term cannot be empty! Please try again!")
        get_input()
    elif len(user_input) == 1:
        if is_valid_string(user_input):
            handle_symbol_search(user_input.lower())
        else:
            print("\nYour search term was not a valid string! Please try again!")
            get_input()
    else:
        if is_valid_string(user_input):
            handle_extended_search(user_input.lower())
        else:
            print("\nYour search term was not a valid string! Please try again!")
            get_input()

def main():
    print("Hello! Welcome to the Symbol Lookup!")
    print("At anytime you can type 'exit' to close the program.")
    print("You can look up symbols by what they are on your keybard (!, @, #, etc.),")
    print("or if you know the name, you can also type that!")
    print("You can also type 'all' to display all symbols, and their given names.")

    print("\nLet's get started!")
    get_input()

if __name__ == "__main__":
    main()