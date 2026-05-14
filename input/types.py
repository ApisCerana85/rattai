class UserOutput(): pass

class Text(UserInput):
    text: str
    def __init__(self, text: str):
        self.text = text

class Exit(UserInput):
    pass