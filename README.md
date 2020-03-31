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

#### TODOS

[ ] Create tools for parameter smoothing.
[ ] Create a consistent interpolation scheme.
[ ] Add a biquad filter
[ ] Add virtual keyboard
