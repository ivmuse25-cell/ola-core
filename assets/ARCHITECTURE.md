# Ola Core - Architecture Diagram

```
┌──────────────────────────────────────────────────────────────────────────┐
│                          Ola Core Architecture                           │
└──────────────────────────────────────────────────────────────────────────┘

┌─────────────┐         ┌──────────────┐         ┌─────────────────────┐
│   Client    │ JSON-   │  Unix Socket │ systemd │   Tokio Runtime     │
│ (Python/CLI)│ RPC     │ /run/ola/    │ socket  │   (Async I/O)       │
│             ├────────▶│   ola.sock   ├────────▶│                     │
└─────────────┘         └──────────────┘ activ.  │  ┌──────────────┐   │
                                                  │  │ Connection   │   │
                                                  │  │ Handlers     │   │
                                                  │  │ (async tasks)│   │
                                                  │  └──────┬───────┘   │
                                                  │         │ mpsc::    │
                                                  │         │ channel   │
                                                  └─────────┼───────────┘
                                                            │
                                                            ▼
                                            ┌───────────────────────────┐
                                            │  Camera Worker Thread     │
                                            │   (Blocking I/O)          │
                                            │                           │
                                            │  • blocking_recv()        │
                                            │  • JoinHandle for cleanup │
                                            │  • Graceful shutdown      │
                                            └──────────┬────────────────┘
                                                       │
                                                       ▼
                                              ┌────────────────┐
                                              │ /dev/video*    │
                                              │ Camera Devices │
                                              └────────────────┘

┌──────────────────────────────────────────────────────────────────────────┐
│  Security Layer                                                          │
│  • UID Allowlist (/etc/ola/allowlist)                                   │
│  • SO_PEERCRED authentication (every connection)                         │
│  • Hardened systemd (ProtectSystem, NoNewPrivileges, PrivateTmp)       │
│  • File permissions: 0640 (secrets), 0770 (socket)                       │
└──────────────────────────────────────────────────────────────────────────┘
```

## Key Design Decisions

### Why Dedicated Camera Worker Thread?
- **Problem**: Camera I/O is blocking (v4l/opencv)
- **Solution**: Dedicated OS thread for blocking operations
- **Benefit**: Tokio runtime never blocks, clean resource management

### Why JoinHandle?
- **Problem**: `spawn_blocking` can leak resources
- **Solution**: Explicit `JoinHandle` + `.join()` on shutdown
- **Benefit**: Guaranteed clean shutdown, no zombie threads

### Why systemd Socket Activation?
- **Problem**: Service startup overhead, privilege issues
- **Solution**: systemd manages socket, activates service on-demand
- **Benefit**: Zero-downtime restarts, lazy start, privilege separation

### Why UID Allowlist?
- **Problem**: Need explicit access control
- **Solution**: `/etc/ola/allowlist` with UID-based auth
- **Benefit**: Fine-grained control, deny-by-default security
