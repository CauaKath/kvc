### KVC

It is a version control app that I created to learn more how does git works and to learn Rust.

ℹ️ The name means to KVC = Kath Version Control

### Prerequisites

To run it in your machine you'll need to install rust and cargo. If you don't have, just click [here](https://www.rust-lang.org/tools/install).

Now you can clone the repository and enter the repository folder using:

```bash
git clone https://github.com/CauaKath/kvc.git
cd kvc
```

### Install

Now that you're inside the repository you can run the following to create a binary and use the project globally:

```bash
cargo install --path .
```

### Using

For now, we only have the following commands:

* help -> That will show how to use all the other commands.
* init -> That will initiate a kvc repository in the current directory.
* config -> That will enable you to access or change any of the default configurations.
* add -> (WIP) For now just validates the passed path.
