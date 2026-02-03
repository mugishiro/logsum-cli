# logsum-cli

A small CLI tool to summarize access logs.

## What it does
- Summarizes logs by `status` or `path`
- Sorts by count descending, then key ascending
- Counts invalid lines and prints `skipped N` at the end

## Usage
```
logsum <path> [status|path]
```

- `<path>`: log file path (required)
- If omitted, it defaults to `status`

### Examples
```
# Default is status
logsum access.log

# Explicit status
logsum access.log status

# Path summary
logsum access.log path
```

## Input format
Line format:
```
[YYYY-MM-DD] METHOD PATH STATUS
```

Example:
```
[2026-01-31] GET /index.html 200
[2026-01-31] POST /login 302
```

Path example (the `path` key):
```
/index.html
```

### Input rules
- Extra spaces or tabs are allowed
- `PATH` does not contain spaces
- `STATUS` is numeric only
- Lines that don't match are treated as invalid and skipped

## Output format
```
KEY COUNT
...
skipped N
```

- `KEY` is the `status` or `path`
- `COUNT` is the number of occurrences
- Order: count descending, key ascending
- `skipped N` is the number of invalid lines

## Output example
Sample log `access.log`:
```
[2026-01-31] GET /index.html 200
[2026-01-31] POST /login 302
[2026-01-31] GET /index.html 200
[2026-01-31] GET /help 404
bad line
```

```
logsum access.log
```
Output:
```
200 2
302 1
404 1
skipped 1
```

```
logsum access.log path
```
Output:
```
/index.html 2
/help 1
/login 1
skipped 1
```
