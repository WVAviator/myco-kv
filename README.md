# MycoKV

MycoKV is a lightweight, fast, and persistent hierarchical key-value store built in Rust.

## Installation

Note: MycoKV is not currently available for Windows operating systems.

To install MycoKV for MacOS or Linux, navigate to the [releases page](https://github.com/WVAviator/myco-kv/releases/latest) and download the latest release for your operating system.

For MacOS, if you have an M1/M2 chip, you will need to download the `darwin-arm64` release. Otherwise, download the `darwin-amd64` release.

For Linux, download the `linux-amd64` release.

Once downloaded, extract the archive and run the `myco-kv` executable. If you wish to run MycoKV from anywhere on your machine, you can move the executable to a directory in your `$PATH` such as `/usr/local/bin`.

#### Coming Soon

- Installation via package managers such as Homebrew and APT.
- Running MycoKV as a Docker container.
- Windows support.

## Using MycoKV

MycoKV is configured to run with the built-in REPL that opens upon starting the database server, or if you prefer you can use a client/server application protocol such as [Telnet](https://en.wikipedia.org/wiki/Telnet) to connect to the server and send commands.

By default, MycoKV runs on port 6922, however this can be changed by flag when starting the server, using either of the following commands:

```bash
cargo run -- --port 1234
cargo run -- -p 1234
```

In the future, drivers and SDKs for MycoKV will be developed in many popular languages and frameworks, including Java, Node.js, and Python.

### Basic Usage

At its current stage of development, MycoKV currently supports three basic commands, `GET`, `PUT`, and `DELETE`. You can store floats, integers, strings, booleans, and even null as values.
Example usage:

```
PUT mykey "my value"
GET mykey
DELETE mykey
```

When sending `GET mykey`, the resulting value is returned as a plain string:

```
> GET mykey
value
```

### Expiring Keys

MycoKV supports expiring keys after a certain amount of time. This can be done by using the `EXPIRE` or `EXPIREAT` commands. `EXPIREAT` takes a UNIX timestamp as an argument, while `EXPIRE` takes a number of milliseconds as an argument.
Example usage:

```
> PUT mykey "my value"
"my value"
> EXPIRE mykey 1000
OK
```

After only half a second has passed, you will still be able to get the key's value:

```
> GET mykey
"my value"
```

After one second has passed, the key will no longer exist:

```
> GET mykey
E09: Key mykey not found
```

MycoKV manages key expirations internally by periodically removing expired keys and by ensuring all expired keys are removed before executing any operations such as `GET` or `DELETE`.

If an expiration already exists for the key, calling `EXPIRE` or `EXPIREAT` again will overwrite the previous expiration.

If the key is deleted using the `DELETE` command, any existing expiration will be removed.

### Purging Data

If you want to clear all entries in the database, you can use the `PURGE` command:

```
> PUT mykey 123
123
> GET mykey
123
> PURGE
> GET mykey
Error: Key not found
```

By default, MycoKV persists your data between server start/stop by writing to a log stored on your machine.

If you do not wish to persist data between start/stop of the MycoKV server, you can start the application with the `--purge` flag, which will delete all log entries and start the database with a clean slate.

```bash
> cargo run -- --purge
```

### Nested Keys

MycoKV also supports "nested" keys, useful for grouping values together and querying multiple values at once.
Nested values are delimited by a `.`.

Querying multiple nested keys can be done using a wildcard `*` as the last nested value.

Example usage:

```
PUT kitchen.cupboards 4
PUT kitchen.countertops "granite"
PUT kitchen.refrigerator.eggs 12
GET kitchen.*
```

The result of sending the above GET request would be the following JSON object consisting of all nested keys:

```json
{
  "cupboards": 4,
  "countertops": "granite",
  "refrigerator": {
    "eggs": 12
  }
}
```

Note that if the parent key has a value set as well, for example by calling `PUT kitchen "tuscan"`, that value will appear as the key "\_" in the resulting JSON object:

```json
{
  "_": "tuscan",
  "cupboards": 4,
  "countertops": "granite",
  "refrigerator": {
    "eggs": 12
  }
}
```

### Depth Limiting

MycoKV keys can be nested arbitralily deep if desired - however in some cases you may wish to limit the depth of the keys you are retrieving.
This can be done by appending a "maximum depth" integer to the wildcard operator.

Example usage:

```
PUT players.p1 "John Doe"
PUT players.p2 "Jane Doe"
PUT players.p1.health 80
PUT players.p2.health 75.5
GET players.*1
```

The result of sending the above get request is to only fetch values one level deep from the `players` key, returning the below JSON result:

```json
{
  "p1": "John Doe",
  "p2": "Jane Doe"
}
```

Note that the nested "health" keys were not returned, because those exist at a nested depth of 2 and the max depth requested was 1. The max depth is inclusive, so a max depth of 2 would return the health keys, and the values of the "p1" and "p2" keys would then be represented by the key "\_" as in the previous example.

## Additional Notes

- You cannot return the entire database as a JSON object by sending `GET *` - this is to prevent accidental expensive operations. If you really need to do this, you will need to intentionally nest every key one level deep.
- Because the "\*" and the "\_" characters are used as special characters, they cannot be used in key names.

## Contributing

MycoKV is in active development, and any contributions are welcome. You can contribute by providing any of the following:

1. Suggesting new features or improvements
2. Reporting bugs or issues
3. Contributing code or design decisions
4. Providing feedback on existing or developing features

Please reach out if you have any questions!
