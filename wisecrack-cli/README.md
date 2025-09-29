# Rust Wisecrack cli app

A command line application to fetch daily dad jokes and motivational quotes of the day

## Options

### Fetch dad joke

```sh
-j, --joke
```

### Fetch Quote of the day

```sh
-q, --quote
```

### Format output (optional)

Default: text

```sh
-o, --output text|json
```

### Config file option (optional)

```sh
-c, --config <file-path>
```

config options precedence

1. command line options are present
2. default location HOME/.wisecrack
3. env vars

Note: only one option is allowed out of jokes/quotes
