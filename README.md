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

Example of input file:

```text
http://68.183.139.110:8888
http://32.223.6.94:80
http://37.19.220.179:8443
http://162.223.94.164:80
http://146.190.186.100:8888
http://5.161.41.17:80
http://162.223.94.163:80
http://138.199.48.1:8443
http://184.10.84.74:80
http://165.225.208.91:10605
http://68.188.59.198:80
http://50.122.86.118:80
http://172.108.208.74:80
http://50.168.163.176:80
http://50.171.32.231:80
http://50.171.32.230:80
http://50.168.163.181:80
http://50.168.163.180:80
http://50.237.89.164:80
http://50.171.32.227:80
http://50.171.32.229:80
http://50.171.32.228:80
http://50.171.32.224:80
http://50.171.32.225:80
http://50.171.32.222:80
http://50.168.163.178:80
http://50.168.163.179:80
http://50.237.89.165:80
http://50.237.89.160:80
http://50.237.89.161:80
http://50.237.89.162:80
http://50.237.89.163:80
http://50.168.163.177:80
http://50.237.89.170:80
http://50.168.163.183:80
```
