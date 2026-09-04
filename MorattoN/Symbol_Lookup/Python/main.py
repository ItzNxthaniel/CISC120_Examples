# Nathaniel C. Moratto
# Symbol Lookup
from symbols import symbols

def is_valid_string(user_input: str) -> bool:
    return not user_input.isnumeric()

def print_symbol(found_symbol: list[str]):
    print(f"  Symbol: {found_symbol[0]}")
    print(f"  Given Name: {found_symbol[1][0].title()}")
    alias_names = list(map(lambda x: x.title(), found_symbol[1][1:]))
    if len(alias_names) > 0:
        print(f"  Also known as: {", ".join(alias_names)}")

def handle_symbol_search(user_symbol: str):
    found_symbol = list(filter(lambda x: x[0] == user_symbol, symbols))

    if len(found_symbol) == 0:
        print(f"\nYour symbol {user_symbol} is not in the database. Please try again!")
        get_input()
    else:
        found_symbol = found_symbol[0]
        print_symbol(found_symbol)
        print("\nGood News! We found your symbol!")


        print(f"\nThat was fun! Let's do it again!")
        get_input()

def handle_extended_search(user_input: str):
    if user_input == "all":
        print("\nAll available symbols:")
        for item in symbols:
            print(f"  {item[0]} - {item[1][0].title()}")

        print(f"\nBack it up! Back to search!")
        get_input()
    elif user_input == "exit":
        print("\nGoodbye, thank you for coming!")
        exit(0)

    found_symbol = None
    for item in symbols:
        names = item[1]

        if user_input in names:
            found_symbol = item

    if found_symbol is not None:
        print("\nGreat news! We found the symbol you were looking for:")
        print_symbol(found_symbol)

        print("\nBack to search!")
        get_input()
    else:
        print(f"\nUnfortunately I wasn't able to find what you meant by '{user_input}'. Please try again")
        get_input()

def get_input():
    user_input = input("Search: ").strip()

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
    print("You can look up symbols by what they are on your keyboard (!, @, #, etc.),")
    print("or if you know the name, you can also type that!")
    print("You can also type 'all' to display all symbols, and their given names.")

    print("\nLet's get started!")
    get_input()

if __name__ == "__main__":
    main()