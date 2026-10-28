"""Generate Moonveil's original little synth score and effects using only Python's standard library."""
from pathlib import Path
import math
import struct
import wave

ROOT = Path(__file__).resolve().parents[1] / 'assets'
RATE = 22050
TAU = math.tau

def write(name, duration, voices):
    samples = []
    for i in range(int(duration * RATE)):
        t = i / RATE
        v = sum(voice(t) for voice in voices)
        samples.append(struct.pack('<h', int(max(-1, min(1, v)) * 26000)))
    with wave.open(str(ROOT / name), 'wb') as f:
        f.setnchannels(1)
        f.setsampwidth(2)
        f.setframerate(RATE)
        f.writeframes(b''.join(samples))

write('strike.wav', 0.16, [lambda t: math.sin(TAU * (800*t - 1600*t*t)) * math.exp(-t*38) * 0.4,
                           lambda t: math.sin(TAU*3200*t)*math.sin(TAU*1797*t)*math.exp(-t*50)*0.12])
write('chime.wav', 0.65, [lambda t: sum(math.sin(TAU*f*t)*math.exp(-t*(6+j)) for j,f in enumerate([880,1108.73,1318.51]))*0.16])
write('magic.wav', 1.2, [lambda t: sum(math.sin(TAU*f*t + math.sin(t*5)*0.2)*math.exp(-t*3)*(1-math.exp(-t*60)) for f in [523.25,783.99,1046.5,1567.98])*0.12])

# A seamless 32-second D-major pentatonic ambient loop. Bell envelopes use
# circular time, preserving tails at the loop boundary instead of cutting them.
notes = [293.66,369.99,440.0,587.33,739.99,659.25,440.0,369.99,
         246.94,369.99,493.88,587.33,739.99,587.33,493.88,369.99]
duration=32.0
def bells(t):
    result=0.0
    for j,f in enumerate(notes):
        age=(t-j*2.0)%duration
        envelope=math.exp(-age*1.3)*(1-math.exp(-age*30))
        result += (math.sin(TAU*f*age)+0.24*math.sin(TAU*f*2.0*age))*envelope*0.19
    return result
def pad(t):
    # Integer loop frequencies ensure phase continuity at 32 seconds.
    result=0.0
    for f in [146.84375,220.0,293.65625,369.96875]:
        result += math.sin(TAU*f*t+0.10*math.sin(TAU*t/16))*0.023
    return result*(0.83+0.17*math.sin(TAU*t/32))
write('moonlight.wav',duration,[bells,pad])
print('Created four original audio assets.')
