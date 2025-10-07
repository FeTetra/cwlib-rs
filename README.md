# cwlib-rs
## A rust-based library for dealing with assets from the Craftworld engine used by the LittleBigPlanet series

### Right now this is very minimal, heres a list of what it can do at the moment.
#### (De)serialization
It can (de)serialize two formats at the moment
1. GUID maps (traditionally blurayguids.map)
2. File ARChives (traditionally data.farc)

Soon, it will use the deserialized data from the FARC to allow reading data out of the archive, as it gives me the exact offset and size of any given file in the archive, which can then be read and manipulated as a standalone file. Serializing is as easy as assembling some metadata about a file (namely the file's size and SHA-1 hash, as well as a precalculated offset where the file will be) into a FARCTableEntry structure. Then you would get the amount of entries you have and write out the footer. Things get a bit more complicated for the GUID map entry, however.

I would like to write some reference documentation at some point on how to use this library, but right now it's so barebones that it's frankly not worth the time to even try using it for anything serious.
