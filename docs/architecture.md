# Architecture

Software is development in MVC-like approach without coupling internal functionality to interface with which we're interacting (in this case: CLI). Software will be built with attention to modularity and self-extendability instead solidering everything into one layer. Software is built within [local-first]() approach as traditional software was built, there is no telemetry enabled by default combined privacy-focused approach to do not leak user's data into network for purposes other than intented cloud-sync of journal in future.

## High-level Architecture of Project

### Repository Structure

#### `cli` directory

Directory responsible for Command Line Interface (CLI) of application, should be threated as front-end to other packages and the "core" functionality of growing application. Structure of CLI files and modules should reflect the path of interface, which means top-tier items that are shared across application such as database connection or configuration container should be placed in root directory of `cli` app and then if for example subcommand is `ingestion` the all related operations should be placed in such directory. In long-term this will help navigate through codebase instead relying on query/command separation which are not so obvious for CLI applications.

- CLI Application manages high-level configuration for lower-tier modules.
- CLI Application manages database connection which is later used by modules like `neuij`.


#### `packages` directory

Application will be built in pseudo monorepository pattern, as it's hard to not use one when developing application in Rust, there will be definitely few packages which later will be merged with upstream of Neuronek's project.

##### `nudb`: "SeaORM-Managed SQLite"

##### `neuij`: "Neuronek Inestion Journal"

#### `docs` directory

Defines a book with all publicly available information about the project (whole documentation that could be wrapped in words is available in repository as project is open-source in all of the meaning of this word)

### Dependency Graph of Packages
