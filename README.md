# intolines
Rust iterators are weird, in that it is very difficult (nay, impossible?) to return a reference to `self`. 
Ideally, the type for this would be `Iterator<Item=&str>` but this seems to violate some rule about the lifetime of the returned reference so it is merely a struct with a `next` method.
