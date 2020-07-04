# Audio Tools

This library contains useful tools for audio DSP, such as modules for
converting midi note values to frequency, wavetable creation, and delay
effects. This library is new highly unstable.

This library aims to provide tools that are as flexible and composable as
possible. To achieve this parameters to signal generators and effects are
generally provided on every tick, which gives callers the flexibility to
modulate parameters in any way imaginable. This makes signal generation and
effects modules much more flexible than trying to provide accessors for every
possible modulation scheme, while placing more responsibility on callers.

## Blocks
**TODO**: rename from modules

### Conversion

Purely functional caclulations between samples, bpm, time, etc.

### Wavetable

Purely functional utilities for creating wavetables

### Osc

Utitities for wavetable-based oscillation.

It may be really nice to move toward purely-functional implementation for 
`increment`. There may be no need to contain state in this module.

### MIDI

Contains a function to create a vec of MIDI frequencies. See Pirkle 2014 for 
details on using pitch bend and other frequency modulation in tandem with MIDI 
frequencies.

### Interpolation

Not implemented.

Simple, purely functional implementations for linear and cubic interpolations

## Modules 

User-facing audio modules with user controls.

**Most-wanted modules**:

+ Mixer
+ Smoother
+ State-variable Filter
+ Modal Bank, Impulse Generator
+ FM Oscillator
+ Pitch Shifter (monophonic)
+ Phase Vocoder Pitch Shifter
+ Reverb

## Delay

### Synth

Prebuilt synthesis modules, ready to interact with user controls and audio 
callbacks.
