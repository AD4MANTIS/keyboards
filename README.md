# keyboards

## rewritten in Rust

Comparison:

``` julia
temperature = 500
epoch = 20
coolingRate = 0.99
num_iterations = 25000
```

Julia:
> 62.265114 seconds (26.52 M allocations: 1.370 GiB, 0.35% gc time, 1.85% compilation time: 3% of which was recompilation)

Rust:
> Duration: PT27.854251S

## now with german qwertz support

I added support for the german keys with the QWERTZ layout.
Here is an example layout i got:
![example german layout](resources/example.png)

original readme:


Supporting simulated annealing code for the [Why I Made The World's Worst Keyboard](https://youtu.be/188fipF-i5I) YouTube video.

Written in Julia... because it's fast, easy to read, and annoys my labmates.

To run, clone the this repository and start Julia with
`julia --project=.` and run `include("keyboardSA.jl")`.
It should start by benchmarking your training data (myBook.txt)
against QWERTY followed by building it's own optimal layout.
Change the number of iterations and cooling rates as
desired within the `data` block at the beginning of the file.
The terminal will give some indication of current progress
(also stored by a new text file will give a iteration-by-iteration record of progress),
and .png files of the current best solution will be saved to your same directory.

To train on your own custom dataset either point the "myBook.txt" somewhere else or just replace its contents.

Good luck!

## more detailed instructions

Download at https://julialang.org/downloads/ and install the Julia language.

Assuming `julia` is in your path, run

```bash
git clone https://github.com/AtomicFrontierCode/keyboards.git
cd keyboards
julia -L 'keyboardSA.jl'
```

If you want to play with the script, it's recommended to use the
[Revise.jl](https://github.com/timholy/Revise.jl) package to minimize latency.
