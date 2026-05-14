import threading
from output.piper import Piper
import parser.types as parse_types
from parser.parse import ParserEngine, DummyParser

class Responder():
    quit_event: threading.Event
    piper: Piper

    def __init__(self, quit_event: threading.Event):
        self.piper = Piper()
        self.quit_event = quit_event

    def respond(self, action: ParseOutput):
        match action:
            case parse_types.Text():
                self.piper.say(action.text)
            case parse_types.Exit(): 
                self.quit_event.set()
            case None: pass