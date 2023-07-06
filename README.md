# MycoKV

MycoKV is a lightweight, fast, and persistent hierarchical key-value store built in Rust.

## Getting Started

MycoKV is still in active development without a release, however if you would like to demo the application, you can follow the steps below.

1. Install Rust from [the official website](https://www.rust-lang.org/tools/install)
2. Clone this repository to your local machine to create the project directory
3. Run the command `cargo run` from within the directory

## Using MycoKV

At this time, MycoKV is only configured to run with the built-in REPL that opens upon starting the database server. 
In the future, drivers and SDKs for MycoKV will be developed in many popular languages and frameworks, including Java, Node.js, and Python.

### Basic Usage

At its current stage of development, MycoKV currently supports three commands, `GET`, `PUT`, and `DELETE`.

Example usage:
```
PUT mykey "my value"
GET mykey
DELETE mykey
```
When sending `GET mykey`, the result is returned as a JSON object consisting of the requested key-value pair: 
```
"{"mykey":"my value"}"
```

### Nested Keys

MycoKV also supports "nested" keys, useful for grouping values together and querying multiple values at once.
Nested values are delimited by a `.`.
Querying multiple nested keys can be done using a wildcard `*` as the last nested value.

Example usage:
```
PUT mykey "Value 1"
PUT mykey.a "Value 2"
PUT mykey.b "Value 3"
GET mykey.*
```

The result of sending the above GET request would be the following JSON object consisting of all nested keys as well as the parent key:
```
"{"mykey":"Value 1","mykey.a":"Value 2","mykey.b":"Value 3"}"
```

### Depth Limiting

MycoKV keys can be nested arbitralily deep if desired - however in some cases you may wish to limit the depth of the keys you are retrieving. 
This can be done by appending a "maximum depth" integer to the wildcard operator.

Example usage:
```
PUT players.p1 = "John Doe"
PUT players.p2 = "Jane Doe"
PUT players.p1.health = "80"
PUT players.p2.health = "75"
GET players.*1
```

The result of sending the above get request is to only fetch values one level deep from the `players` key, returning the below JSON result:
```
"{"players.p1":"John Doe","players.p2":"Jane Doe"}"
```
Note that the nested "health" keys were not returned, because those exist at a nested depth of 2 and the max depth requested was 1.

## Contributing

MycoKV is in active development, and any contributions are welcome. You can contribute by providing any of the following:

1. Suggesting new features or improvements
2. Reporting bugs or issues
3. Contributing code or design decisions
4. Providing feedback on existing or developing features

Please reach out if you have any questions!
