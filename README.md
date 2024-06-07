# Work in Progress

This is a work in progress, goal is to have a CLI tool that automates AppStore Connect and Google Play Console without manually having to go to the website. Eventually add JSON output to be used in CI/CD pipelines.

So far I have implemented the following features:

Apple:

- [x] Patch "what's new" text for an app pending release (Available in the CLI as a command)

GooglePlay:

- [x] Patch "release notes" text for an app pending release (Available in the CLI as a command)

## Usage

```bash
freitool notes --version-name 1.0.0 --platform android --google-key-path /path/to/key.json --package-name com.example.app --message "Patched from CLI" --locale en-GB
```

## Features on the roadmap

- [ ] Add support for yaml configuration file
- [ ] Make it available on homebrew
- [ ] Add json output for CI/CD pipelines
- [ ] Automated tests

> **Note:** This is a personal project and not affiliated with Apple or Google in any way. Use at your own risk. ALSO I work on it when I have time, so it might take a while to get to the roadmap features.
