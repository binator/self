Terminology:

| Name         | Description                                                                         |
|:------------:|-------------------------------------------------------------------------------------|
| `Stream`     | A structure that would produce `Item` when asked                                    |
| `Parser`     | Something that will check that `Item` produced by `Stream` are correct              |
| `Context`    | A structure that will manage `Failure` from `Parser`                                |
| `Token`      | Represent what a `Parser` return for Success                                        |
| `Atom`       | A structure that contain information about the `Failure` or `Error` from a `Parser` |
| `Element`    | Something, generally an enumeration, that will contain all different kind of `Atom` |
| `Parsed`     | Enumeration that indicate result of a `Parser`                                      |
| `Parse`      | A trait that all `Parser` implement, used to use a `Parser`                         |
| `Failure`    | Indicate a `Parser` didn't validate the input                                       |
| `Success`    | Indicate a `Parser` validate the input                                              |
| `Error`      | Indicate a `Parser` encounter an irrecoverable error.                               |
| `Streaming`  | A trait that `Stream` implement to make their job                                   |
| `Item`       | `Item` produced by a `Stream`, generally just an `u8`                               |
| `Span`       | A delimited part of the `Stream`                                                    |
| `Contexting` | A trait that all `Context` will implement, used to accumulate failure of `Parser`   |
