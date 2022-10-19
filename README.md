<div align="center">
  <img src=".github/img/banner.png">
  <br>
  <br>


  [![GitHub All Releases][release-img]][release]
  [![Issues][issues-img]][issues]
  [![crates.io][crates-img]][crates]
  ![Docker Pulls][docker-pulls]
  [![License: MIT][license-img]][license]
</div>

[crates]: https://crates.io/crates/qs-lite
[crates-img]: https://img.shields.io/crates/v/qs-lite
[release]: https://github.com/qsocket/qs-lite/releases
[release-img]: https://img.shields.io/github/v/release/qsocket/qs-lite?style=flat-square
[downloads]: https://github.com/qsocket/qs-lite/releases
[downloads-img]: https://img.shields.io/github/downloads/qsocket/qs-lite/total?logo=github&?style=flat-squ
[issues]: https://github.com/qsocket/qs-lite/issues
[issues-img]: https://img.shields.io/github/issues/qsocket/qs-lite?style=flat-square&color=red
[docker-pulls]: https://img.shields.io/docker/pulls/qsocket/qs-lite?logo=docker&label=docker%20pulls&?style=flat-square
[license]: https://raw.githubusercontent.com/qsocket/qs-lite/master/LICENSE
[license-img]: https://img.shields.io/github/license/qsocket/qs-lite.svg?style=flat-square
[google-cloud-shell]: https://console.cloud.google.com/cloudshell/open?git_repo=https://github.com/qsocket/qs-lite&tutorial=README.md
[qsrn]: https://github.com/qsocket/qsrn


qs-lite is the lightweight version of [qs-netcat](https://github.com/qsocket/qs-netcat) utility. It allows redirecting true PTY sessions with reverse connections over the [QSRN](qsrn), effectively accessing systems under NAT networks or firewalls.

## Installation
Or you can simply test it on a cloud shell.

[![Open in Cloud Shell](.github/img/cloud-shell.png)](google-cloud-shell)

|  **Tool**   |                    **Build From Source**                     |      **Docker Image**       | **Binary Release**  |
| :---------: | :----------------------------------------------------------: | :-------------------------: | :-----------------: |
| **qs-lite** | ```cargo install --git https://github.com/qsocket/qs-lite``` | [Download](#docker-install) | [Download](release) |

---
qs-lite supports 10 architectures and 12 operating systems, check **Supported Platforms** below for detailed table.

<details>
<summary>Supported Platforms</summary>

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

</details>

### Docker Install

[![Docker](http://dockeri.co/image/egee/qsocket)](https://hub.docker.com/r/egee/qsocket/)

```bash
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
```bash
$ qs-lite -l -i   # Workstation A
$ qs-lite -i      # Workstation B
```
