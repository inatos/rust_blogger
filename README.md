## Overview

  Hi All, this is a quickstart blogging app for anyone who wants to try out Rust in a fullstack web environment. This app follows the MVC (Model-View-Controller) design pattern. The backend leverages Rust’s Actix-Web framework, which handles all server operations (i.e. routing, authentication, data management, etc.). The Frontend uses jQuery and Bootstrap to create a dynamic UI experience. All data is stored in a Postgres database via Diesel’s ORM (Object-relational mapping) framework. Finally, this project covers the deployment process for those who would like to deploy their own site using Docker compose.

 Here’s a running example of the project hosted from my VPS: [arathyll.com](https://arathyll.com/)


## Setup

**Database**
1. Install [Postgresql](https://www.postgresql.org/download/).
2. (Optional) Install [pgAdmin](https://www.pgadmin.org/download/). (if you prefer using command line)
3. Create your postgres user.

**App**
1. Install [Rust](https://www.rust-lang.org/tools/install).
2. Update Rust (command line):  `rustup update`
3. Install Diesel (command line): `cargo install diesel_cli --no-default-features --features postgres`
4. Clone/Download this project.
5. Navigate to the project’s root directory.
6. Edit .env to use your postgres user credentials.
7. Setup Diesel (command line): `diesel setup` (If it doesn’t work use full command `diesel setup --database-url "postgres://postgres:password@localhost/rust_blogger"`)
8. Run it: `cargo run` 

**Startup**
1. In your browser navigate to the homepage:  [http://127.0.0.1/](http://127.0.0.1/)
2. Click the “Signup” button in the top right.
3. Enter “admin” for the username.
4. Now you can access the admin view.

**Create Your First Post**
1. Login as the “admin” user.
2. Navigate to “Editor” in the navbar.
3. Set mode to “create”.
4. Choose a post category.
5. Set visibility to “published”.
6. (Optional) Add a new group (used for grouping posts).
7. Add a title.
8. (Optional) Add a label (used for URL navigation for Git category posts).
9. Add your content. Tip: You can use markdown here for custom styling.
10. Submit! Congrats you created your first post.


## Development Resources

**WatchExec**: Used for auto compiling your project.
* [WatchExec Install](https://github.com/watchexec/watchexec)

**Diesel**: Database and ORM commands.
* [Guides](https://diesel.rs/guides/)
* [Diesel Rust Types](https://gist.github.com/steveh/7c7145409a5eed6b698ee8b609b6d1fc)

**Templating**: 
* [Tera](https://tera.netlify.app/docs/https://tera.netlify.app/docs/)

**VPS**: Deploying on a VPS.
* [Point Domain to VPS](https://www.hostinger.com/tutorials/dns/how-to-point-domain-to-vps)

**Docker**: 
* [Docker Install](https://docs.docker.com/get-docker/)
* [Docker Compose Install](https://docs.docker.com/compose/install/)
* [Docker Best Practices](https://docs.docker.com/develop/develop-images/dockerfile_best-practices/)


## Questions?

Feel free to [reach out](mailto:gredder@outlook.com) if you have any questions or need help troubleshooting.

Cheers!
