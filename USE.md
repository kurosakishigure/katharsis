# Use the Katharsis

## Prerequisites

### Linux and macOS

First, download the appropriate binary from the [Releases](https://github.com/kurosakishigure/katharsis/releases) page.

Next, execute the following command in your home directory to create the necessary folders:

```bash
mkdir -p Formulae/bin
```

Move the downloaded binary to ~/Formulae/bin:

```bash
mv ~/your/download/path/katharsis ~/Formulae/bin/katharsis
```

Modify your .zshrc or .bashrc file by adding the following configuration:

```bash
# Formulae
export PATH=$HOME/Formulae/bin:$PATH
```

You can also add an alias in the .zshrc or .bashrc file:

```bash
alias rss=katharsis
```

Alternatively, you can rename the binary file directly:

```bash
mv ~/Formulae/bin/katharsis ~/Formulae/bin/rss
```

Run the following command to apply the changes:

```bash
source .zshrc
# or
source .bashrc
```

## Command Overview

| Command | Description                                                      |
|---------|------------------------------------------------------------------|
| init    | Generates a katharsis.config.toml file in the current directory. |
| help    | Displays help information.                                       |

## Parameter Overview

| Parameter | Description                                                  |
|-----------|--------------------------------------------------------------|
| -c        | Specifies a katharsis.config.toml file as the configuration. |
| -h        | Displays help information.                                   |
| -V        | Displays the current version of Katharsis.                   |

## Explanation of katharsis.config.toml Fields

You can refer to the [RSS 2.0 at Harvard Law](https://cyber.harvard.edu/rss/rss.html) for more detailed documentation.

### rss

| Field       | Description                        |
|-------------|------------------------------------|
| title       | The title of the website.          |
| description | A description of the website.      |
| site_url    | The main URL of the website.       |
| image       | Path to the channel's logo.        |
| copyright   | Copyright information.             |
| language    | Preferred language.                |
| output      | Path to the local output RSS file. |

> Important Notes:
>
> - The site_url field should not include a trailing slash (e.g., https://example.com rather than https://example.com/).
> - The image field is relative to the site_url, for example, favicon.png corresponds
    to https://example.com/favicon.png.
> - The output field specifies the path relative to the working directory (e.g., rss.xml corresponds to ./rss.xml).

### article

| Field       | Description                                                |
|-------------|------------------------------------------------------------|
| title       | The tag containing the article's title.                    |
| description | The tag or attribute containing the article's description. |
| input       | The file(s) to be used as input, matching specified rules. |
| author      | Information about the article's author.                    |
| link        | The prefix for the article's URL.                          |
| content     | The tag or attribute containing the article's content.     |
| date        | The tag containing the article's publication date.         |
| image       | File(s) to be used as the article's cover image.           |
| sort        | Whether to sort articles by their publication date.        |

> Important Notes:
>
> - The input field corresponds to the file body, which is the same as the article's URL slug.
> - The date field's tag must include a [datetime](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/time) attribute, and the attribute value must follow the `%Y-%m-%d` format.
> - The image field’s folder name must match the article's URL slug.
