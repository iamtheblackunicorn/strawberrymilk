<p align="center">
 <img src="/assets/banner.png"/>
</p>

# STRAWBERRY MILK

***A small, sweet tool written in Rust to compile your content for the web.***

![GitHub CI](https://github.com/iamtheblackunicorn/strawberrymilk/actions/workflows/rust.yml/badge.svg)

## ABOUT :books:

Since I am also an author and artist, I was wondering how I would write a small program that turns files with content written in [Markdown](https://en.wikipedia.org/wiki/Markdown) into a website. ***Strawberry Milk*** is that tool. You initialize a new project, write your content in Markdown,
run ***Strawberry Milk*** and voilá! You have a nice and shiny new webpage that has your content in it, styled and ready for the world!

## EXAMPLE PROJECT

You can view a live, deployed ***Strawberry Milk*** project [here](https://blckunicorn.art/strawberrymilk).

## BUILDING

### Tools

You will need the following tools installed and available:

- Rust
- Git

### Steps

- 1.) Get the source code:
```bash
$ git clone https://github.com/iamtheblackunicorn/strawberrymilk.git
```
- 2.) Change directory:
```bash
$ cd strawberrymilk
```
- 3.) Build the source code:
```bash
$ cargo build --release
```

## INSTALLATION

Move the executable on the path `strawberrymilk/target/release/strawberrymilk` to the directory where you keep your binary executables. If you are on Linux or Mac OSX, you might have to change permissions like this: `chmod a+x strawberrymilk`.

## USAGE

### Command-line usage

To compile your project, simply run this command on the command-line:

```bash
$ strawberrymilk yourprojectdir
```

`yourprojectdir` represents the path of your project.

### Creating a new project.

To create a new project, run the following command:

```bash
$ strawberrymilk new myproject
```

This will create a new folder called `myproject`.
Your project's file structure will look something like this:

```text
myproject
├── config.json
└── content
    └── 01.markdown
```

The file, `config.json`, will contain the following:

```JSON
{
  "styles": "https://blckunicorn.art/assets/generic/strawberrymilk.css",
  "content": "content",
  "name": "myproject",
  "output": "index.html"
}
```

- `name`: What is your project called?
- `content`: Which sub-folder contains the project's Markdown files?
- `styles`: To make your content look pretty, you need a stylesheet. Load this from somewhere else. ***Strawberry Milk*** doesn't support local stylesheets.
- `output`: What is the output HTML file supposed to be called?

Next, open up `01.markdown` located in the `content` folder. (Please note that this folder's name has to be the same as the `content` field in the configuration file.) It will contain something like this:

```markdown
# YOUR PROJECT
Your awesome content goes here.
```

You can now fill this out and create Markdown files with numerical filenames (`01.markdown`,`02.markdown`,`03.markdown`, etc.) and when you are done, you can run this command in the project's root directory:

```bash
$ strawberrymilk .
```

If everything is A-OK, you should now have a file called `index.html` in a sub-directory called `build`.

### Deployment to GitHub Pages.

If you have a GitHub account, you can upload your project to a repository, create a new branch called `gh-pages`, create a new file called `rust.yml` at `.github/workflows` in your repository, fill it with the code below, and voilá: You can now view your project on the web under the URL of `yourusername.github.io/yourporject`.

```YAML
on: [push]
name: Strawberry Milk Project CI
jobs:
  build_and_test:
    name: Strawberry Milk Project CI
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v2
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      - uses: actions-rs/cargo@v1
        with:
          command: build
          args: --release
      - uses: actions-rs/cargo@v1
        with:
          command: run
          args: .
      - name: Deploy
        uses: JamesIves/github-pages-deploy-action@v4.2.5
        with:
          branch: gh-pages
          folder: build
```

## CONTRIBUTING

If you have some suggestions for improvement or you want to contribute, either file an issue or fork the repository. If you want to do the latter, make and test your changes, and file a Pull Request.

## CHANGELOG

### Version 1.0.0

- initial release
- upload to GitHub

## NOTE :scroll:

- *Strawberry Milk* by Alexander Abraham a.k.a. *"The Black Unicorn"*
- Licensed under the MIT license.
