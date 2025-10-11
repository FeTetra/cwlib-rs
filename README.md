# cwlib-rs
## A rust-based library for dealing with assets from the Craftworld engine used by the LittleBigPlanet series

### Right now this is very minimal, heres a list of what it can do at the moment.
#### (De)serialization
It can (de)serialize two formats at the moment
1. GUID maps (traditionally blurayguids.map)
2. File ARChives (traditionally data.farc)

#### Files
It can extract files out of a FARC using a SHA-1 hash (poorly)

### Running Tests
If for whatever reason you want to run the tests I have written, clone the repository and `cd` into the downloaded folder. From here you can run `cargo test --lib -- --nocapture --test-threads=1`. Note that the tests are currently written in a way where they cannot be multithreaded (which cargo does by default) due to race conditions with using similar filenames between tests, I may fix this in the future but it's not much of an issue right now.

### Other notes
I would like to write some reference documentation at some point on how to use this library, but right now it's so barebones that it's frankly not worth the time to even try using it for anything serious.
