

Write blog post on Minimal API surface area for interfaces


Why the builder pattern is good:
- looks nice
- can allow for useful validation
why it sucks:
- code duplication when adding higher-level layers
- cannot be serialized directly, means more code duplication

JavaScript has a great philosophy for configuration: everything is an object, nested and nested configuration

I wish other languages (e.g. Rust) adopted that more. Every HTTP library I look at has no way to set Client or Request options directly. this means serializing the request metadata need a whole lot of custom code.

Functions should only take minimal paramaters, otherwise, they should take a `config` which can be serialized.

Validation should happen at the last possible steo when configuration is already all set.


A future version of programming: Semantic APIs, where we define standards that can be interoperated across languages and frameworks. But this requires a heavily interface, component, and serialized data view of programming.

On plugin architecture and registry.

- Most appealing option: Have en Executor or higher level object read the configuration and create an Agent with a static mapping from strings to trait type implementation types. However, might not be possible, build POC
  - With POC, seems limit is at the Config type
- Might need for flexibilty: Full `dyn` types. Won't work with associated types, generic params, self return. However, we could simplify the API surface of the Plugin traits.
  - This could be done by making Config a serde RawValue that then gets parsed manually by the plugin.
- typetag could be simple solution, at least for in-tree plugins

Remote plugins: wrappers that call HTTP or gRPC or DBUS or some MPI to access external plugins

```json
"metadata": {
    "remote_pl_name": {
        "config": "etc"
    }
}
```

Basically they are just a trait like
```rust
pub trait Remote {
    configure(pluginSpecificConfig: T) -> Result
    get_requestor() -> dyn Requestor // (or T: Requestor)
    get_other_plugin() -> Plugin
}
```

They get configured at beginning of file with connection information. Then they are called with `pluginID`??

Or


```json
"plugins": {
    "<id>": {
        "connection":"builtin", //or http, grpc, etc
        "connection_config": "blbla", // path for http, etc
        "config": "etc" // JSON config to send over
    }
}
```

In this case, we have a plugin map, which is used if there are no builtin plugins???
It allows for plugin-level connections, not just for all plugins from one remote.

We used to be able to use any format with `serde_any`. Now we rely on `serde_json`s `RawValue` type.

### Move to typetag

So far has been awesome.

Since serde is recursive, and JobCollection contains each trait, and Serde propagates type bounds, needed to make sure that Traits implement Debug, etc.

Also, during API redesign - allowed extractor to output `Box<dyn Any>` and data sink to consume same.

This theoretically means implementations could use any intermediate representations.

OK - realizing need to work on this.


## About the scraper_rs

It has expanded to be the bloated swiss army knife of all things

Transforms, previous values, generated

All feeling very crappy and antipattern, although am getting experience with match and recursively building out parsers, along with refs and such.

Will need to do API design to purposefully restrict scraper_rs scope. Go back to Spec and documentation. Think about using separate JSON transformation language.

On the existing path we either:
- allow previous to operate on arrays, thus having an "all links selector" work there
- allow sets of transforms, like SubstringAfter and then Template