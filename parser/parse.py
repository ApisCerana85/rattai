from abc import ABC, abstractmethod

import parser.types as parse_types

class ParserEngine(ABC):
    @abstractmethod
    def parse(self, inpt: in_types.UserInput): pass

class DummyParser(ParserEngine):
    def __init__(self):
        pass
    
    def parse(self, inpt: str):
        if inpt is None: return None
        return parse_types.Text(inpt)