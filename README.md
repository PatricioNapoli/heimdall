<div align="center">

  <a style="margin-right:15px" href="#"><img src="https://forthebadge.com/images/badges/made-with-rust.svg" alt="Made with Rust"/></a>


  <a href="https://opensource.org/licenses/MIT"><img src="https://img.shields.io/badge/License-MIT-brightgreen.svg" alt="License MIT"/></a>
  <a href="https://www.rust-lang.org/"><img src="https://img.shields.io/badge/rust-1.64-orange.svg" alt="Rust 1.64"/></a>

</div>


# Heimdall

Project heavily in progress.

## Overview

Rust based reverse proxy, service discovery and auth layer for K8s or Docker Swarm clusters.

## Prerequisites

rust 1.64

## Environment

```
export ENVIRONMENT=local
export HEIMDALL_HQ=localhost:8888
export HEIMDALL_REDIS_HOST=127.0.0.1
export HEIMDALL_SECRET=secret
```

## Build & Run

`cargo run`  
