

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