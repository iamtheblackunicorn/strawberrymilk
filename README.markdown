<p align="center">
 <img src="/assets/banner.png"/>
</p>

# STRAWBERRY MILK :strawberry: :milk_glass: :crab:

***A small, sweet tool written in Rust to compile your content for the web.:strawberry: :milk_glass: :crab:***

![GitHub CI](https://github.com/iamtheblackunicorn/strawberrymilk/actions/workflows/rust.yml/badge.svg)

## ABOUT :books:

Since I am also an author and artist, I was wondering how I would write a small program that turns Markdown content into a website. ***Strawberry Milk*** is that tool. You write a small configuration file, `config.json`, write your Markdown content,
run ***Strawberry Milk*** and voilá! You have a nice and shiny new webpage that has your content in it, styled and perfected.

## BUILDING :pick:

You will need the following tools installed and available:

- Rust
- Git

To compile ***JAML***, follow these steps:

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

## INSTALLATION :inbox_tray:

Move the executable on the path `strawberrymilk/target/release/strawberrymilk` to the directory where you keep your binary executables. If you are on Linux or Mac OSX, you might have to change permissions like this: `chmod a+x strawberrymilk`.

## USAGE :hammer:

### Command-line usage

To compile your project, simply run this command on the command-line:

```bash
$ strawberrymilk yourprojectdir
```

`yourprojectdir` represents the path of your project.

### Creating a new project.

To create a new project, you need a file called `config.json`. It chould contain the following:

```JSON
{
  "name":"Strawberry Milk Test",
  "content":"content",
  "styles":"https://blckunicorn.art/assets/generic/strawberrymilk.css",
  "output":"index.html"
}
```

- `name`: What is your project called?
- `content`: Where are the Markdown files to be compiled?
- `styles`: To make your content look pretty, you need a stylesheet. Load this from somewhere else. ***Strawberry Milk*** doesn't support local stylesheets.
- `output`: What is the output HTML file supposed to be called?

After you have done this, you need to create a directory by the name of what you filled the `content` field with.
In this directory, create your Markdown files with your content.

Once that is done, change directory to where `config.json` is located and run this command:

```bash
$ strawberrymilk .
```

## CONTRIBUTING :heart:

If you have some suggestions for improvement or you want to contribute, either file an issue or fork the repository, make and test your changes, and file a Pull Request.

## CHANGELOG :black_nib:

### Version 1.0.0

- initial release
- upload to GitHub

## NOTE :scroll:

- *Strawberry Milk :strawberry: :milk_glass: :crab:* by Alexander Abraham a.k.a. *"The Black Unicorn"*
- Licensed under the MIT license.
