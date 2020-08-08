# HTML Tests

These are used by more complex extractors like the Rust XPath Extractor.

Unfortunately, the sxd-xpath library only supports a hand-rolled XML DOM implementation which doesn't handle HTML idiosyncracies and slight errors like `<meta blabla>` tag that doesn't have a closing tag and doesn't self-close.

It also didn't support PUBLIC doctypes, but I made a patch for that.

So for now, it seems we must parse with HTML5ever and reserialize to get a compliant HTML fragment that can be understood as XML.


<!-- From github LOL_HTML -->
In some cases, users may find that the querying capabilities and extremely high performance of this library are useful, but would rather extract