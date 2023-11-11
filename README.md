<div align="center">
  <img src=".github/img/banner.png">
  <br>
  <br>


  [![GitHub All Releases][release-img]][release]
  [![Build][workflow-img]][workflow]
  [![Issues][issues-img]][issues]
  [![Crates][crates-img]][crates]
  ![Docker Pulls][docker-pulls]
  [![License: MIT][license-img]][license]
</div>

[crates]: https://crates.io/crates/qs-lite
[crates-img]: https://img.shields.io/crates/v/qs-lite
[release]: https://github.com/qsocket/qs-lite/releases
[release-img]: https://img.shields.io/github/v/release/qsocket/qs-lite
[downloads]: https://github.com/qsocket/qs-lite/releases
[downloads-img]: https://img.shields.io/github/downloads/qsocket/qs-lite/total?logo=github
[issues]: https://github.com/qsocket/qs-lite/issues
[issues-img]: https://img.shields.io/github/issues/qsocket/qs-lite?color=red
[docker-pulls]: https://img.shields.io/docker/pulls/qsocket/qs-lite?logo=docker&label=docker%20pulls
[license]: https://raw.githubusercontent.com/qsocket/qs-lite/master/LICENSE
[license-img]: https://img.shields.io/github/license/qsocket/qs-lite.svg
[google-cloud-shell]: https://console.cloud.google.com/cloudshell/open?git_repo=https://github.com/qsocket/qs-lite&tutorial=README.md
[workflow-img]: https://github.com/qsocket/qs-lite/actions/workflows/main.yml/badge.svg
[workflow]: https://github.com/qsocket/qs-lite/actions/workflows/main.yml
[qsrn]: https://github.com/qsocket/qsrn


qs-lite is a alternative lightweight implementation of [qs-netcat](https://github.com/qsocket/qs-netcat) utility with Rust language. It allows redirecting true PTY sessions with reverse connections over the [QSRN](qsrn), effectively accessing systems under NAT networks or firewalls.

> [!WARNING]  
> This tool is in its early alpha development stage, featuring experimental functionality that may lack backwards compatibility, and users are advised to exercise caution and not use it in production environments.

## Installation

[![Open in Cloud Shell](.github/img/cloud-shell.png)](google-cloud-shell)

|  **Tool**   |                    **Build From Source**                     |      **Docker Image**       | **Binary Release**  |
| :---------: | :----------------------------------------------------------: | :-------------------------: | :-----------------: |
| **qs-lite** |                 ```cargo install qs-lite```                  | [Download](#docker-install) | [Download](release) |

---
qs-lite currently supports 3 architectures and 4 operating systems, check **Supported Platforms** below for detailed table.

<details>
<summary>Supported Platforms</summary>

| **Platform**  | **AMD64** | **386** | **ARM** | **ARM64** | **MIPS** | **MIPS64** | **MIPS64LE** | **PPC64** | **PPC64LE** | **S390X** |
| :-----------: | :-------: | :-----: | :-----: | :-------: | :------: | :--------: | :----------: | :-------: | :---------: | :-------: |
|   **Linux**   |     ✅     |    ✅    |    ❌    |     ✅     |    ❌     |     ❌      |      ❌       |     ❌     |      ❌      |     ❌     |
|  **Darwin**   |     ✅     |    ❌    |    ❌    |     ✅     |    ❌     |     ❌      |      ❌       |     ❌     |      ❌      |     ❌     |
|  **Windows**  |     ✅     |    ✅    |    ❌    |     ✅     |    ❌     |     ❌      |      ❌       |     ❌     |      ❌      |     ❌     |
|  **OpenBSD**  |     ❌     |    ❌    |    ❌    |     ❌     |    ❌     |     ❌      |      ❌       |     ❌     |      ❌      |     ❌     |
|  **NetBSD**   |     ❌     |    ❌    |    ❌    |     ❌     |    ❌     |     ❌      |      ❌       |     ❌     |      ❌      |     ❌     |
|  **FreeBSD**  |     ❌     |    ❌    |    ❌    |     ❌     |    ❌     |     ❌      |      ❌       |     ❌     |      ❌      |     ❌     |
|  **Android**  |     ✅     |    ✅    |    ❌    |     ✅     |    ❌     |     ❌      |      ❌       |     ❌     |      ❌      |     ❌     |
|    **IOS**    |     ❌     |    ❌    |    ❌    |     ❌     |    ❌     |     ❌      |      ❌       |     ❌     |      ❌      |     ❌     |
|  **Solaris**  |     ❌     |    ❌    |    ❌    |     ❌     |    ❌     |     ❌      |      ❌       |     ❌     |      ❌      |     ❌     |
|  **Illumos**  |     ❌     |    ❌    |    ❌    |     ❌     |    ❌     |     ❌      |      ❌       |     ❌     |      ❌      |     ❌     |
| **Dragonfly** |     ❌     |    ❌    |    ❌    |     ❌     |    ❌     |     ❌      |      ❌       |     ❌     |      ❌      |     ❌     |
|    **AIX**    |     ❌     |    ❌    |    ❌    |     ❌     |    ❌     |     ❌      |      ❌       |     ❌     |      ❌      |     ❌     |

</details>

## Usage
```
Usage: qs-lite [OPTIONS]

Options:
  -s, --secret <SECRET>
          secret. (e.g. password) [default: ]
  -e, --exec <EXEC>
          program to execute [default: "bash -il"]
  -f, --forward-addr <FORWARD_ADDR>
          forward address (IP:PORT) for traffic forwarding [default: ]
  -x, --proxy-addr <PROXY_ADDR>
          user socks proxy address for connecting QSRN [default: ]
  -X, --cert-fingerprint <CERT_FINGERPRINT>
          hex encoded TLS certificate fingerprint for validation [default: ]
  -n, --probe <PROBE>
          probe interval for connecting QSRN [default: 5]
  -C, --no-encryption
          disable all (TLS+E2E) encryption
      --no-e2e
          disable End-to-End encryption
  -i, --interactive
          initiate a full PTY (interactive) shell
  -l, --listen
          server mode. (listen for connections)
  -g, --generate
          generate a random secret
  -T, --use-tor
          use TOR network for connecting QSRN
      --qr
          generate a QR code with given stdin and print on the terminal
  -q, --quiet
          quiet mode. (no stdout)
  -v, --verbose
          verbose output mode
  -h, --help
          Print help
  -V, --version
          Print version
```
### Examples
1. Log in to Workstation A from Workstation B through any firewall/NAT
```bash
$ qs-lite -l -i   # Workstation A
$ qs-lite -i      # Workstation B
```
---
**Crypto / Security Mumble Jumble**
- The connections are end-2-end encrypted. This means from User-2-User (and not just to the Relay Network). The Relay Network relays only (encrypted) data to and from the Users.
- The QSocket uses [SRP](https://en.wikipedia.org/wiki/Secure_Remote_Password_protocol) with [SPAKE2](https://docs.rs/spake2/latest/spake2/) and ChaCha20 for ensuring [perfect forward secrecy](https://en.wikipedia.org/wiki/Forward_secrecy). This means that the session keys a
re always different, and recorded session traffic cannot be decrypted by the third parties even if the user secret is known.
- The session key is 256 bit and ephemeral. It is freshly generated for every session and generated randomly (and is not based on the password).
- A brute force attack against weak secrets requires a new TCP connection for every guess. But QSRN contains a strong load balancer which is limiting the consecutive connection attempts.
- Do not use stupid passwords like 'password123'. Malice might pick the same (stupid) password by chance and connect. If in doubt use *qs-netcat -g* to generate a strong one. Alice's and Bob's password should at least be strong enough so that Malice can not guess it by chance while Alice is waiting for Bob to connect.
- If Alice shares the same password with Bob and Charlie and either one of them connects then Alice can not tell if it is Bob or Charlie who connected.
- Assume Alice shares the same password with Bob and Malice. When Alice stops listening for a connection then Malice could start to listen for the connection instead. Bob (when opening a new connection) can not tell if he is connecting to Alice or to Malice.
- We did not invent SRP. It's a well-known protocol, and it is well-analyzed and trusted by the community. 

