# Happy Wakey sidecar external certification

This repository is an independent consumer in the `happy-wakey-test`
organization. It certifies exact Happy Wakey sidecar revision
`6bee12449fb421b142d3c5836bbc0547f805462a` in two ways:

1. Cargo resolves the sidecar library as an immutable Git dependency, then the
   consumer exercises its public configuration and reducer API.
2. `scripts/certify.sh` clones that exact revision, installs the binary into an
   isolated temporary prefix, and runs a black-box readiness scenario against a
   real loopback product server.

The black-box scenario proves the installed process starts without writing to
stdout, probes an adjacent product over bounded loopback HTTP, crosses its
success threshold, and exposes HTTP 200 from the inherited `/readyz` endpoint.
It does not claim a deployed Kubernetes cluster or live Shared Auth/Opto Sync
authority.

```bash
scripts/certify.sh
```
