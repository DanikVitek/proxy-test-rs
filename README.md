# Simple proxy tester

This is a simple proxy tester that will test a list of proxies and output the results to a file or to the STDOUT.

## Usage

```text
Usage: proxy_test.exe [OPTIONS] --input <INPUT>

Options:
  -i, --input <INPUT>              Path to the file containing proxies
  -o, --output-file <OUTPUT_FILE>  Path to the file to write the proxies to [default: ./output.txt]
      --stdout                     Print the proxies to stdout
  -p, --ping-addr <PING_ADDR>      Address to send the request to [default: http://www.google.com]
  -t, --timeout <TIMEOUT>          Timeout for the request in seconds [default: 10]
  -v, --verbose                    Print more information
  -h, --help                       Print help
  -V, --version                    Print version
```
