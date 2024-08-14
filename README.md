# spring-initialzr-in-rust

Command line application that creates java spring projects using rust.<br>
To create a project you must specify the dependencies, the package name and the application name.<br>
The application name will be applied to the spring server initializer file.

**Accepted dependencies** <br>
*In the future more may be added <br>

"web", "data-jpa", "security", "thymeleaf", "actuator", "batch", "kafka", "devtools", "mail", "flyway", "liquibase", "redis", "amqp", "validation", "h2", "web-services", "graphql", "cache", "mysql", "postgresql"

**Commands** <br>
-n for project name (required) <br>
-d for add dependencies <br>
-p for package name (required) <br>
-h for help <br>

1. **Clone the repository**
   ```bash
   git clone https://github.com/dev-gabrielferreira/spring-initialzr-in-rust

2. **Go to the directory**
   ```bash
   cd spring-initialzr-in-rust
<br>

**There are different ways to run the application, so here are some**
1. **Run with cargo command**
   ```bash
   cargo run -- -n PROJECT_NAME -p PACKAGE_NAME

2. **Generate the bin archive**
   ```bash
   cargo build --release
   ```
   Now run the following command to install the project and use outside the root folder

   ```bash
   cargo install --path .
   ```
   Now you can use the command specified in Cargo.toml to use the application

   ```bash
   spring-start -n MySpringApp -p com.myspringapp -d web,data-jpa,h2
   ```
