#i already feel sorry for sombody reading this code. (it was kinda vibecoded)

import subprocess
import sounddevice as sd
import soundfile as sf
import numpy as np

PIPER_PATH   = ".venv/bin/piper"
PIPER_MODEL  = "models/pl_PL-darkman-medium.onnx"
PIPER_CONFIG = "models/pl_PL-darkman-medium.onnx.json"
# !!! check in *.onnx.json file
PIPER_SAMPLE_RATE = 22050

sentences = [
    # hej rattatai jestem bardzo glodny ale nie mam nic w lodowce
    "jasne, pokaż składniki i coś wymyślimy",
    # okej to jest co mi zostało
    "Okej, widze że masz pomidora, cytryne, jabłko i bułkę. Możemy zrobić naleśniki",
    # wsumie to nie głupie powiedz mi prosze jak to zrobić
    "najpierw na patelnie kładziesz cytryne..."
]

dtype = np.int16

with sd.RawOutputStream(
    samplerate=PIPER_SAMPLE_RATE,
    channels=1,
    dtype=dtype
) as stream:
    chunk_size = 4096
    
    for sentence in sentences:
        input("-- press ENTER --")

        p = subprocess.Popen(
            [PIPER_PATH, "-m", PIPER_MODEL, "-c", PIPER_CONFIG, "--output-raw"],
            stdin=subprocess.PIPE,
            stdout=subprocess.PIPE,
            bufsize=0
        )
        p.stdin.write(sentence.encode("utf-8") + b"\n")
        p.stdin.close()
        while True:
            data = p.stdout.read(chunk_size)
            if not data:
                break
            stream.write(data)