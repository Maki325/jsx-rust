So we need to trait the GetSignal

So that we can have GetSignals that update
And those that don't

So if the user passes a variable like this
```rust
let a = String::new("Hello, world!");

<Buttom name=a>
```
We can also use the `.get()` to get the value, even if it's constant
