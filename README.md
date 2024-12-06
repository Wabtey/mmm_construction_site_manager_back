# Construction Site Manager - MMM Labs

<!-- [![Chrono GitHub Actions][gh-image]][gh-checks]
[![codecov.io][codecov-img]][codecov-link]

[gh-image]: https://github.com/chronotope/chrono/actions/workflows/test.yml/badge.svg?branch=main
[gh-checks]: https://github.com/chronotope/chrono/actions/workflows/test.yml?query=branch%3Amain
[codecov-img]: https://img.shields.io/codecov/c/github/chronotope/chrono?logo=codecov
[codecov-link]: https://codecov.io/gh/chronotope/chrono -->

![Coverage](https://img.shields.io/badge/Coverage-100%25-brightgreen)
![Tests Passed](https://img.shields.io/badge/Tests%20Passed-9%2F9-yellow)

## Model

```mermaid
---
title: Construction Site Manager
---
classDiagram
    class Site {
    }
    
    Site <-- Worker

    class Worker {
        + name
    }
    note for SiteManager "manage a specific site"
    class SiteManager {
    }
    note for SitesGlobalManager "manage all region sites"
    class SitesGlobalManager {
    }
```

## TODOs

- [x] Auth
  - [x] GitHub Auth
- [ ] Database
  - [ ] Diesel Model
- [ ] Roles
  - [ ] SiteManager
  - [ ] SitesGlobalManager
