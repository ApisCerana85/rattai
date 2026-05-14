import threading
import queue
import re

class Listener(threading.Thread):
    input_queue:   queue.Queue
    quit_event:    threading.Event
    WAKE_KEYWORDS: [str]

    def __init__(self, quit_event: threading.Event, wake_keywords: [str]):
        super().__init__(daemon=True)
        
        self.input_queue = queue.Queue()
        self.quit_event = quit_event
        self.WAKE_KEYWORDS = wake_keywords
    
    def run(self):
        while not self.quit_event.is_set():
            try:
                text = input("user: ")

                #definetly not optimal
                for word in text.split():
                    if word in self.WAKE_KEYWORDS:    
                        self.input_queue.put(text)
            except EOFError:
                self.quit_event.set()
    
    def get_nowait(self): 
        try:
            return self.input_queue.get_nowait()
        except queue.Empty:
            return None