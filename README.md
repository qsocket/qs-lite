# qs-lite
<p align="center">
  <img src="https://github.com/qsocket/qs-lite/raw/master/.github/img/banner.png">
  <br/><br/>
  <a href="https://github.com/qsocket/qs-lite">
    <img src="https://img.shields.io/github/v/release/qsocket/qs-lite?style=flat-square">
  </a>
  <a href="https://github.com/qsocket/qs-lite">
    <img src="https://img.shields.io/crates/v/qs-lite">
  </a>
  <a href="https://goreportcard.com/report/github.com/qsocket/qs-lite">
    <img src="https://goreportcard.com/badge/github.com/qsocket/qs-lite?style=flat-square">
  </a>
  <a href="https://github.com/qsocket/qs-lite/issues">
    <img src="https://img.shields.io/github/issues/qsocket/qs-lite?style=flat-square&color=red">
  </a>
  <a href="https://raw.githubusercontent.com/qsocket/qs-lite/master/LICENSE">
    <img src="https://img.shields.io/github/license/qsocket/qs-lite.svg?style=flat-square">
  </a>
</p>

qs-lite is the lightweit version of [qs-netcat](https://github.com/qsocket/qs-netcat) utility. It allows redirecting true PTY sessions with reverse connections over the [QSRN](https://github.com/qsocket/qsrn), effectively backdooring systems under NAT networks or firewalls.

## Installation
|  **Tool**   |                    **Build From Source**                     |      **Docker Image**       |                   **Binary Release**                    |
| :---------: | :----------------------------------------------------------: | :-------------------------: | :-----------------------------------------------------: |
| **qs-lite** | ```cargo install --git https://github.com/qsocket/qs-lite``` | [Download](#docker-install) | [Download](https://github.com/qsocket/qs-lite/releases) |

---

**Supported Platforms**
| **Platform**  | **AMD64** | **386** | **ARM** | **ARM64** | **MIPS** | **MIPS64** | **MIPS64LE** | **PPC64** | **PPC64LE** | **S390X** |
| :-----------: | :-------: | :-----: | :-----: | :-------: | :------: | :--------: | :----------: | :-------: | :---------: | :-------: |
|   **Linux**   |     ✅     |    ✅    |    ✅    |     ✅     |    ✅     |     ✅      |      ✅       |     ✅     |      ✅      |     ✅     |
|  **Darwin**   |     ✅     |    ❌    |    ❌    |     ✅     |    ❌     |     ❌      |      ❌       |     ❌     |      ❌      |     ❌     |
|  **Windows**  |     ✅     |    ✅    |    ✅    |     ✅     |    ❌     |     ❌      |      ❌       |     ❌     |      ❌      |     ❌     |
|  **OpenBSD**  |     ✅     |    ✅    |    ✅    |     ✅     |    ❌     |     ✅      |      ❌       |     ❌     |      ❌      |     ❌     |
|  **NetBSD**   |     ✅     |    ✅    |    ✅    |     ✅     |    ❌     |     ❌      |      ❌       |     ❌     |      ❌      |     ❌     |
|  **FreeBSD**  |     ✅     |    ✅    |    ✅    |     ✅     |    ❌     |     ❌      |      ❌       |     ❌     |      ❌      |     ❌     |
|  **Android**  |     ✅     |    ✅    |    ✅    |     ✅     |    ❌     |     ❌      |      ❌       |     ❌     |      ❌      |     ❌     |
|    **IOS**    |     ✅     |    ❌    |    ❌    |     ✅     |    ❌     |     ❌      |      ❌       |     ❌     |      ❌      |     ❌     |
|  **Solaris**  |     ✅     |    ❌    |    ❌    |     ❌     |    ❌     |     ❌      |      ❌       |     ❌     |      ❌      |     ❌     |
|  **Illumos**  |     ✅     |    ❌    |    ❌    |     ❌     |    ❌     |     ❌      |      ❌       |     ❌     |      ❌      |     ❌     |
| **Dragonfly** |     ✅     |    ❌    |    ❌    |     ❌     |    ❌     |     ❌      |      ❌       |     ❌     |      ❌      |     ❌     |
|    **AIX**    |     ❌     |    ❌    |    ❌    |     ❌     |    ❌     |     ❌      |      ❌       |     ✅     |      ❌      |     ❌     |

### Docker Install

[![Docker](http://dockeri.co/image/egee/qsocket)](https://hub.docker.com/r/egee/qsocket/)

```
docker pull egee/qsocket
docker run -it egee/qsocket
```

## Usage
```
qs-lite 1.0
Ege BALCI. <egebalci@pm.me>
Qsocket lite shell.

USAGE:
    qs-lite [OPTIONS]

OPTIONS:
    -C, --notls                  Disable TLS encryption.
    -e, --exec [<INPUT>...]      Program to execute. [default: /bin/bash]
    -g, --generate               Generate a random secret.
    -h, --help                   Print help information
        --pin                    Enable certificate fingerprint verification on TLS connections.
    -q, --quiet                  Disable output.
    -s, --secret [<INPUT>...]    Secret. (e.g. password).
    -t, --probe [<INPUT>...]     Probe interval for QSRN. [default: 5]
    -v, --verbose                Verbose output.
    -V, --version                Print version information
```
### Examples
1. Log in to Workstation A from Workstation B through any firewall/NAT
```
$ qs-lite -l -i   # Workstation A
$ qs-lite -i      # Workstation B
```
