from abc import ABC, abstractmethod

import parser.types as parse_types

class ParserEngine(ABC):
    @abstractmethod
    def parse(self, inpt: in_types.UserInput): pass

class DummyParser(ParserEngine):
    EXIT_KEYWORDS: [str]
    def __init__(self, exit_keywords: [str]):
        self.EXIT_KEYWORDS=exit_keywords
    
    def parse(self, inpt: str):
        if inpt is None: return None
        
        #this will be taken care of by ai in near future if you know what i mean
        for word in inpt.split():
            if word in self.EXIT_KEYWORDS:
                return parse_types.Exit()
        
        return parse_types.Text(inpt)