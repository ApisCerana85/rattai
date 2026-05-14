import threading
import queue
import sys

from input.listen import Listener
from output.respond import Responder
from parser.parse import *

EXIT_KEYWORDS = ["quit", "q", "exit", "wyłącz"]
WAKE_KEYWORDS = ["hej", "hejka"]

def main():
    print("start")
    quit_event = threading.Event()

    #listening thread
    listen_thread = Listener(quit_event, WAKE_KEYWORDS)
    listen_thread.start()

    responder = Responder(quit_event)

    parser = DummyParser(EXIT_KEYWORDS)

    while not quit_event.is_set():
        inpt = listen_thread.get_nowait()
        action = parser.parse(inpt)
        responder.respond(action)

    print("closing program... (you propably need to press enter one more time)")
    sys.exit()

if __name__ == "__main__":
    main()