import subprocess

sentences = [
    # hej rattai mam jakieś resztki w lodówce i niewiem co z nimi zrobic
    "Oczywiście, Z składników ktore mamy możemy zrobić prosty i szybki makaron.",
    "Potrzebujesz tylko: "
    "Paprykę, "
    "Cebule, "
    "Śmietane, "
    "Pomidory w puszce, "
    "i Makaron",
    "Pokrój Paprykę i cebulę, połóż makaron na patelnie, rozlej śmietane i pomidory, dolej wody i wszystko zagotuj.",
    "Oczywiście, przypomne ci kiedy wyłączyć gaz"
]

PIPER_PATH   = ".venv/bin/piper"
PIPER_MODEL  = "models/pl_PL-darkman-medium.onnx"
PIPER_CONFIG = "models/pl_PL-darkman-medium.onnx.json"

for i in range(len(sentences)):
    p = subprocess.Popen(
        [PIPER_PATH, "-m", PIPER_MODEL, "-c", PIPER_CONFIG, "-f", f"out/sentence{i}.mp3"],
        stdin=subprocess.PIPE,
        stdout=subprocess.PIPE,
        bufsize=0
    )
    p.stdin.write(sentences[i].encode("utf-8") + b"\n")
    p.stdin.close()