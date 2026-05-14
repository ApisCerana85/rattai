import threading
import queue
import sys

from input.listen import Listener
from output.respond import Responder

EXIT_KEYWORDS = ["quit", "q", "exit"]

def main():
    print("start")
    quit_event = threading.Event()

    #listening thread
    listen_thread = Listener(quit_event, EXIT_KEYWORDS)
    listen_thread.start()

    responder = Responder(quit_event, "dummy")

    while not quit_event.is_set():
        inpt = listen_thread.get_nowait()
        responder.respond(inpt)

    print("closing program...")
    sys.exit()

if __name__ == "__main__":
    main()