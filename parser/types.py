class ParseOutput(): pass

class Text(ParseOutput):
    text: str
    def __init__(self, text: str):
        self.text = text

class Exit(ParseOutput):
    pass