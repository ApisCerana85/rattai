from abc import ABC, abstractmethod
import input.types as in_types

class ParserEngine(ABC):
    @abstractmethod
    def respond(self, inpt: in_types.UserInput): pass

class DummyParser(ParserEngine):
    def __init__(self):
        pass
    
    def respond(self, inpt: in_types.UserInput):
        match inpt:
            case in_types.Text():
                return inpt.text