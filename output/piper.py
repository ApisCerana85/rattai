import subprocess
import sounddevice as sd
import soundfile as sf
import numpy as np

class Piper():
    PATH:        str # path to piper executable
    MODEL:       str # path to piper speech model
    CONFIG:      str # path to piper speech model config
    SAMPLE_RATE: int # just audio sample rate !!! check in the config file if its the same

    def __init__(
        self,
        path=".venv/bin/piper",
        model="output/piper_models/pl_PL-darkman-medium.onnx",
        config = "output/piper_models/pl_PL-darkman-medium.onnx.json",
        sample_rate = 22050 # !!! check in the config file if its the same
    ):
        self.PATH        = path
        self.MODEL       = model
        self.CONFIG      = config
        self.SAMPLE_RATE = sample_rate

    def say(self, inpt: str):
        print(f"piper > {inpt}")
        with sd.RawOutputStream(
            samplerate=self.SAMPLE_RATE,
            channels=1,
            dtype=np.int16
        ) as stream:
            CHUNK_SIZE = 4096

            p = subprocess.Popen(
                [self.PATH, "-m", self.MODEL, "-c", self.CONFIG, "--output-raw"],
                stdin=subprocess.PIPE,
                stdout=subprocess.PIPE,
                bufsize=0
            )
            p.stdin.write(inpt.encode("utf-8") + b"\n")
            p.stdin.close()
            while True:
                data = p.stdout.read(CHUNK_SIZE)
                if not data: break
                stream.write(data)