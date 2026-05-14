import threading
from output.piper import Piper
import input.types as in_types
from parse.parse import ParserEngine, DummyParser

class Responder():
    quit_event: threading.Event
    parser: ParserEngine
    piper: Piper

    def __init__(self, quit_event: threading.Event, parser_engine: str):
        self.piper = Piper()
        self.quit_event = quit_event
        #match parser_engine:
        #    case "dummy": self.parser=DummyParser()
        self.parser = DummyParser()

    def respond(self, inpt: UserInput):
        match inpt:
            case in_types.Text():
                outpt = self.parser.respond(inpt)
                self.piper.say(outpt)
            case in_types.Exit(): 
                self.quit_event.set()
            case None: pass