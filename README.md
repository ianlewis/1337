# 1337 Code

[![tests](https://github.com/ianlewis/1337/actions/workflows/pre-submit.units.yml/badge.svg)](https://github.com/ianlewis/1337/actions/workflows/pre-submit.units.yml) [![OpenSSF Scorecard](https://api.securityscorecards.dev/projects/github.com/ianlewis/1337/badge)](https://securityscorecards.dev/viewer/?uri=github.com%2Fianlewis%2F1337)

This repository contains various LEET (1337) code examples and solutions to
coding problems, challenges, and algorithms. The goal is to provide a collection
of well-documented and efficient code snippets that can be used as references.

## Makefile

The `Makefile` is used for managing files and maintaining code quality. It
includes a default `help` target that prints all make targets and their
descriptions grouped by function.

```shell
1337$ make
1337 Makefile
Usage: make [COMMAND]

  help                      Print all Makefile targets (this message).
Tools
  license-headers           Update license headers.
Formatting
  format                    Format all files
  json-format               Format JSON files.
  md-format                 Format Markdown files.
  yaml-format               Format YAML files.
Linting
  lint                      Run all linters.
  actionlint                Runs the actionlint linter.
  zizmor                    Runs the zizmor linter.
  markdownlint              Runs the markdownlint linter.
  renovate-config-validator Validate Renovate configuration.
  textlint                  Runs the textlint linter.
  yamllint                  Runs the yamllint linter.
Maintenance
  clean                     Delete temporary files.
```

## Formating and linting

Some `Makefile` targets for basic formatters and linters are included along
with GitHub Actions pre-submits. Versioning of these tools is done via the
`requirements.txt` and `packages.json`. This is so that the versions can be
maintained and updated via `dependabot`-like tooling.

Required runtimes:

- [`node`]: Node.js is required to run some linters and formatters.
- [`python`]: Node.js is required to run some linters and formatters.

The following tools need to be installed:

- [`actionlint`]: For linting GitHub Actions workflows.
- [`shellcheck`]: For linting shell code in GitHub Actions workflows.

The following tools are installed locally:

- [`yamllint`]: For YAML (e.g. GitHub Actions workflows). (installed in Python
  virtualenv `.venv`).
- [`prettier`]: For formatting markdown and yaml (installed in local
  `node_modules`).
- [`markdownlint`]: For linting markdown (installed in local `node_modules`).

`Makefile` targets and linter/formatter config are designed to respect
`.gitignore` and not cross `git` submodules boundaries. However, you will need
to add files using `git add` for new files before they are picked up.

`Makefile` targets for linters will also produce human-readable output by
default, but will produce errors as [GitHub Actions workflow
commands](https://docs.github.com/en/actions/writing-workflows/choosing-what-your-workflow-does/workflow-commands-for-github-actions)
so they can be easily interpreted when run in Pull-Request [status
checks](https://docs.github.com/en/pull-requests/collaborating-with-pull-requests/collaborating-on-repositories-with-code-quality-features/about-status-checks).

## Contributing

See [`CONTRIBUTING.md`](./CONTRIBUTING.md) for contributor documentation.

[`actionlint`]: https://github.com/rhysd/actionlint
[`markdownlint`]: https://github.com/DavidAnson/markdownlint
[`yamllint`]: https://www.yamllint.com/
[`prettier`]: https://prettier.io/
[`shellcheck`]: https://www.shellcheck.net/
[`node`]: https://nodejs.org/
[`python`]: https://www.python.org/
