# Construction Site Manager - MMM Labs

[![ConstructionSIteManagerBack GitHub Actions][gh-image]][gh-checks]
[![codecov.io][codecov-img]][codecov-link]

[gh-image]: https://github.com/Wabtey/mmm_construction_site_manager_back/actions/workflows/rust.yml/badge.svg?branch=feature-sites
[gh-checks]: https://github.com/Wabtey/mmm_construction_site_manager_back/actions/workflows/rust.yml?query=branch%3Afeature-sites
[codecov-img]: https://img.shields.io/codecov/c/github/Wabtey/mmm_construction_site_manager_back%2Fbranch%2Ffeature-sites?logo=codecov
[codecov-link]: https://codecov.io/gh/Wabtey/mmm_construction_site_manager_back

## Features

- authentication mechanism to distinguish `SiteManager`s (chef de chantier) of `SiteSupervisor`s (responsable des chantiers) of `Client`s.

## "Quick"start

- run the back
  - install rust (minimal profile).
  `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh` from [rust-lang](https://www.rust-lang.org/tools/install)
  - setup the database
    - install `mysql`
      - in archLinux:
        - `sudo pacman -S mariadb`
        - `mariadb-install-db --user=mysql --basedir=/usr --datadir=/var/lib/mysql`
    - create the user and the table
      - `sudo mysql -u root -p -e "CREATE DATABASE IF NOT EXISTS sitemanagerdb;"` (password = your root password, `mariadb` in arch)
      - `sudo mysql -u root -p -e "CREATE USER 'sitemanager'@'localhost' IDENTIFIED BY 'icantbelieveiletyougetaway';"`
      - `sudo mysql -u root -p -e "GRANT ALL PRIVILEGES ON sitemanagerdb.* TO sitemanager'@'localhost';"`
    <!-- DOCKER 
    - `sudo docker pull mysql:latest`
    - `sudo docker run --name mysql-container-site-manager -e MYSQL_ROOT_PASSWORD=rootpassword -e MYSQL_DATABASE=site-manager-database -e MYSQL_USER=sitemanager -e MYSQL_PASSWORD=icantbelieveiletyougetaway -d mysql:latest`
    - check if running: `sudo docker ps`
    - test mysql: `sudo docker exec -it mysql-container-site-manager mysql -u root -p`
      - type the root password (here `rootpassword`)
    -->
  - setup `rocket`
    - create a `Rocket.toml` file and put these lines

    ```toml
    [default.oauth.github]
    provider = "GitHub"
    # register a [github app here](https://github.com/settings/apps/new)
    client_id = "<the_client_id_of_your_newly_created_app>"
    client_secret = "<the_private_key_of_your_newly_created_app>"

    [default.databases.diesel_mysql]
    url = "mysql://sitemanager:icantbelieveiletyougetaway@localhost:3306/sitemanagerdb"
    ```

- `cargo run`

## Model

```mermaid
---
title: Construction Site Manager
---
classDiagram
    class Site {
        + id: Uuid
        + name: String
        + purpose: String
        + location: String
        + start_date: Date
        + end_date: Date
        + status: SiteStatus
        + resources: Resources
        + client: Client
        + workers: List~Workers~
        + site_manager: SiteManager
    }
    
    Site -- SiteStatus : status
    Site -- Resources : resources
    Site -- Client : client
    Site -- Worker : workers
    Site -- SiteManager : manager

    %% --------------------------- Site Attributes --------------------------- %%

    class SiteStatus {
        <<Enumeration>>
        NotCarried
        InProgress
        Interrupted
        Completed
    }

    class Resources {
        + vehicles: List~Vehicle~
    }

    Resources -- Vehicle : vehicles

    class Vehicle {
        + name: String
        + reserved_dates: List~ReservedDate~
    }

    class ReservedDate {
        + start_date: Date
        + end_date: Date
    }
    
    Vehicle -- ReservedDate : reserved_dates

    %% -------------------------------- Roles -------------------------------- %%

    class AppRole {
        <<Abstract>>
        + name: String
    }

    class Users {
        + id: Uuid
        + username: String
        + role: AppRole
    }
    
    Users -- AppRole : role

    class Client {
        + phone_number: String
    }

    class Worker {
    }

    note for SiteManager "manage a specific site"
    class SiteManager {
    }

    note for SiteSupervisor "manage all region sites"
    class SiteSupervisor {
    }

    AppRole <|-- Client
    AppRole <|-- Worker
    AppRole <|-- SiteManager
    AppRole <|-- SiteSupervisor
```

## TODOs

- [x] Auth
  - [x] GitHub Auth
- [x] Database
  - [x] Diesel Model
  - [x] connect database to rocket diesel
  - [x] create and read
- [ ] Roles
  - [ ] SiteManager
  - [ ] SiteSupervisor
- [ ] TODO: containerize rust app and sql db

## Debug

> [!NOTE]
> Within `sudo mysql -u root -p` / `sudo mariadb -u root -p`.
> You can check all your tables with `SHOW TABLES FROM sitemanagerdb;`.
