def reply_greeting(argument):
    print(f"Hello, {argument}! how are you?")
def ask_name():
    print("What is your name?")
    name = input()
    return name
if __name__ == "__main__":
    user_name = ask_name()
    reply_greeting(user_name)