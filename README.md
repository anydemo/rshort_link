# RTodo

Rust Todo web service
<!-- One Paragraph of project description goes here -->

## Getting Started

These instructions will get you a copy of the project up and running on your local machine for development and testing purposes. See deployment for notes on how to deploy the project on a live system.

### Prerequisites

What things you need to install the software and how to install them

```
make
```

### Installing

A step by step series of examples that tell you how to get a development env running

Say what the step will be

```
cp .env.example .env
docker-compose up -d
```

And repeat

```
make
make run
```

```
outpub:

[2019-09-12T00:51:12Z DEBUG rshort_link] input config = Config { db_link: "postgres://postgres:postgres@localhost/rshort_link", redis_session_link: "127.0.0.1:6379", listen_on: "127.0.0.1:8088" }
[2019-09-12T00:51:12Z INFO  rshort_link] Starting server listen on 127.0.0.1:8088
[2019-09-12T00:51:12Z DEBUG rshort_link] Constructing the App
[2019-09-12T00:51:12Z DEBUG rshort_link] Constructing the App
[2019-09-12T00:51:12Z DEBUG rshort_link] Constructing the App
[2019-09-12T00:51:12Z DEBUG rshort_link] Constructing the App
```

## Running the tests

Explain how to run the automated tests for this system

### Break down into end to end tests

Explain what these tests test and why

```
Give an example
```

## Deployment

Add additional notes about how to deploy this on a live system

### TODO
* [x] log
* [x] web
* [ ] short_link utils
* [x] postgres with orm
* [ ] cache

## Built With

* [Cargo](https://crates.io/) - Dependency Management

## Contributing

Please read [CONTRIBUTING.md](https://gist.github.com/PurpleBooth/b24679402957c63ec426) for details on our code of conduct, and the process for submitting pull requests to us.

## Versioning

We use [SemVer](http://semver.org/) for versioning. For the versions available, see the [tags on this repository](https://github.com/your/project/tags). 

## Authors

* **Exfly** - *Initial work* - [Exfly](https://github.com/exfly)

See also the list of [contributors](https://github.com/your/project/contributors) who participated in this project.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details

## Acknowledgments

* Hat tip to anyone whose code was used
* Inspiration
* etc

* [snowflake](https://github.com/twitter-archive/snowflake) vs [Sharding & ID in Instagram](https://instagram-engineering.com/sharding-ids-at-instagram-1cf5a71e5a5c)

## Thanks

* [diesel](https://github.com/diesel-rs/diesel) with [guides](https://diesel.rs/guides/getting-started/)
* [demo](https://github.com/code4wt/short-url)
