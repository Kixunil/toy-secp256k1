# Toy implementation of secp256k1 in Rust

## BIG FAT WARNING

**Do NOT dare to use this code in any serious project!
This is literally me rolling my own crypto STRICTLY FOR EDUCATION PURPOSE.
If you use this code without review by a team of cryptography experts knowledgeable about Rust I WILL PUBLICLY SHAME YOU AND DECLARE YOU AS A SCAMMER.
I will also do everything in my power to help your victims to sue you.
If you need secp256k1 library for serious project, use [libsecp256k1](https://github.com/rust-bitcoin/rust-secp256k1)**

## About

It's probably clear from the warning now: this is an implementation of `secp256k1` in pure idiomtic Rust for **educational purposes**.
I wrote it to **teach myself** about `secp256k1`/ECC.

If you want to learn too, I recommend writing your own - better than getting it on silver plate.
However you may use this as a cheat sheet in case you get stuck.
But from my experience it was great to not use any cheat sheet.
So if you get stuck, get a break and try it next day - worked well for me.
I only reused `U256` type and avoided writing one particular algorithm by hand - not going to spoil you. :)
Good luck!

## License

MITNFA with this additional clause:

By using this software *for any non-educational purpose* you agree to be publicly ridiculed, mocked, shamed and described as a "scammer" or "fraudster" by me or anyone else
unless you demonstrate that a team of experienced cryptographers with deep knowledge of Rust reviewed the code and deemed it secure.
In case of doubt you may reach out to me and ask whether I find your particular case acceptable.
