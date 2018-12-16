# lmc
A Little Man Computer (http://peterhigginson.co.uk/LMC) emulator written in Rust (and a bit of Python).

Instructions to operate:
- Asemble your at http://peterhigginson.co.uk/LMC, making sure to have the "Show Decimal" option enabled.
- Pipe that into `stripper.py`.
- Replace the string in the `Lmc::from( ... )` command of the `main()` function with the output.
- Recompile and run (`cargo run` or `cargo run --release`).
