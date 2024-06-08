# Freitool lets you interact with GooglePlay and AppStore Connect from the command line

[!["Buy Me A Coffee"](https://www.buymeacoffee.com/assets/img/custom_images/orange_img.png)](https://www.buymeacoffee.com/andersonfds)


This is a work in progress, goal is to have a CLI tool that automates AppStore Connect and Google Play Console without manually having to go to the website. Eventually add JSON output to be used in CI/CD pipelines.

Available commands, so far:
- **notes**: Patches release notes for Google Play or AppStore release

## Installation

```bash
brew tap andersonfds/freitool
brew install freitool
```

## Usage

```bash
# Patches release notes for google play in English
freitool notes --version-name 1.0.0 --platform android --google-key-path /path/to/key.json --package-name com.example.app --message "Patched from CLI" --locale en-GB
```

## Features on the roadmap

- [ ] Add support for yaml configuration file
- [ ] Add json output for CI/CD pipelines
- [ ] Automated tests
- [ ] Add `rollout` command so you can release the app from the CLI

> **Note:** This is a personal project and not affiliated with Apple or Google in any way. Use at your own risk. ALSO I work on it when I have time, so it might take a while to get to the roadmap features.
