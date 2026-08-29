# External sidecar certification policy

- This repository must remain independent of the `happy-wakey` production
  organization and consume an exact immutable sidecar revision.
- Certify the packaged/installed binary, not only a source checkout or dry run.
- Keep all servers, ports, install prefixes, and processes test-owned and
  temporary. Never contact production services.
- Do not accept or invent Shared Auth, Opto Sync, GitHub, or cluster secrets.
- Keep stdout expectations explicit because the sidecar reserves stdout.
- Update the revision in `Cargo.toml`, `README.md`, and `scripts/certify.sh`
  together; CI rejects drift through Cargo's exact Git dependency.
