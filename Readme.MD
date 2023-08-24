# Port Scanner

A simple multi-threaded port scanner written in Rust.

## Usage

Run the port scanner by providing the target IP address and optional flags.

```bash
$ ./port-scanner [FLAGS] <IP_ADDRESS> 
```

## Flags
- `-h`, `--help`: Show help message.
- `-j <THREADS>`: Specify the number of threads for scanning.

## Examples

1. Scan target IP address using default settings (4 threads):
```bash
$ ./port-scanner 192.168.1.1
```

2. Scan target IP address with a specific number of threads:
```bash
$ ./port-scanner -j 8 192.168.1.1
```

3. Display help message
```bash
$ ./port-scanner -h
```

